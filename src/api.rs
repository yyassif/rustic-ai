use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use actix_web::{web, HttpRequest, HttpResponse, Error};
        use actix_web::web::Payload;
        use actix_ws::Message as Msg;
        use futures::StreamExt;
        use std::sync::Arc;
        use std::io::Error as StdErr;
        use std::io::ErrorKind as StdErrKind;
        use ollama_rs::Ollama;
        use ollama_rs::generation::chat::{ChatMessage, MessageRole};
        use ollama_rs::generation::chat::request::ChatMessageRequest;
        use std::env;
        use dotenv::dotenv;

        async fn infer(
            ollama: Arc<Ollama>,
            chat_history: &mut Vec<ChatMessage>,
            user_message: &String,
            tx: tokio::sync::mpsc::Sender<String>
        ) -> Result<Option<String>, Error> {
            chat_history.push(ChatMessage::new(MessageRole::User, user_message.clone()));
            dotenv().ok();
            let model_name = env::var("OLLAMA_MODEL_NAME").expect("OLLAMA_MODEL_NAME Env Var must be set");
            let chat_req = ChatMessageRequest::new(model_name.to_string(), chat_history.clone());

            let stream = ollama.send_chat_messages_stream(chat_req).await
                .map_err(|e| Error::from(StdErr::new(StdErrKind::Other, format!("Ollama error: {:?}", e))))?;

            let mut response = String::new();

            futures::pin_mut!(stream);

            while let Some(res) = stream.next().await {
                let res = res.map_err(|_| Error::from(StdErr::new(StdErrKind::Other, "Stream error")))?;

                if let Some(msg) = res.message {
                    response.push_str(&msg.content);

                    tx.send(msg.content).await.map_err(|_| Error::from(StdErr::new(StdErrKind::Other, "Anything Error")))?;
                }

                if res.final_data.is_some() {
                    break;
                }
            }

            chat_history.push(ChatMessage::new(MessageRole::Assistant, response.clone()));

            Ok(Some(response))
        }

        fn session_setup() -> Vec<ChatMessage> {
            dotenv().ok();
            let persona = env::var("OLLAMA_SYSTEM_PROMPT").expect("OLLAMA_SYSTEM_PROMPT Env Var must be set");
            
            vec![
                ChatMessage::new(MessageRole::System, persona.to_string()),
                ChatMessage::new(MessageRole::User, "Hello there?".to_string()),
                ChatMessage::new(MessageRole::Assistant, "Hello - How may I help you today?".to_string()),
            ]
        }

        pub async fn ws(req: HttpRequest, body: Payload, ollama: web::Data<Ollama>) -> Result<HttpResponse, Error> {
            use std::sync::Mutex;
            use tokio::sync::mpsc;

            let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

            let (send_inference, mut receive_inference) = mpsc::channel(100);
            let ollama_instance: Arc<Ollama> = ollama.into_inner().clone();
            let sess = Arc::new(Mutex::new(session));

            let sess_clone = sess.clone();
            actix_rt::spawn(async move {
                let (send_new_user_message, mut receive_new_user_message) = mpsc::channel(100);

                let ollama_clone = Arc::clone(&ollama_instance);

                actix_rt::spawn(async move {
                    let mut chat_history = session_setup();

                    while let Some(new_user_message) = receive_new_user_message.recv().await {
                        let _ = infer(ollama_clone.clone(), &mut chat_history, &new_user_message, send_inference.clone()).await;
                    }
                });

                while let Some(Ok(msg)) = msg_stream.next().await {
                    match msg {
                        Msg::Ping(bytes) => {
                            let res = sess_clone.lock().unwrap().pong(&bytes).await;
                            if res.is_err() {
                                return;
                            }
                        }
                        Msg::Text(s) => {
                            let _ = send_new_user_message.send(s.to_string()).await;
                        }
                        _ => break,
                    }
                }
            });

            actix_rt::spawn(async move {
                while let Some(message) = receive_inference.recv().await {
                    sess.lock().unwrap().text(message).await.expect("Issue sending over WebSocket");
                }
            });

            Ok(response)
        }
    }
}