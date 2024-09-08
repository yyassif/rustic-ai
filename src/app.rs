use crate::model::conversation::{Conversation, Message};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::cell::RefCell;
use futures_util::sink::SinkExt;
use futures::StreamExt;
use std::rc::Rc;
use chrono::Utc;
use uuid::Uuid;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message as WsMessage;

mod components;
use components::chat_area::ChatArea;
use components::prompt_area::PromptArea;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rustic-ai.css"/>

        // sets the document title
        <Title text="Chat Page"/>

        // content for this welcome page
        <Router>
            <main class="max-w-4xl mx-auto my-2 px-2 text-gray-700 dark:text-gray-100">
                <Routes>
                    <Route path="" view=ChatPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the ChatPage of the application.
#[component]
fn ChatPage() -> impl IntoView {
    let (conversation, set_conversation) = create_signal(Conversation::new("Default Conversation".to_string()));
    let (is_loading, set_is_loading) = create_signal(false);

    let ws_sender: Rc<RefCell<Option<futures_util::stream::SplitSink<WebSocket, WsMessage>>>> = Rc::new(RefCell::new(None));
    let ws_sender = Rc::new(ws_sender);

    {
        let ws_sender = ws_sender.clone();
        create_effect(move |_| {
            let location = web_sys::window().unwrap().location();
            let hostname = location.hostname().expect("failed to retrieve origin hostname");
            let ws_url = format!("ws://{hostname}:3000/ws");

            let ws = WebSocket::open(&ws_url).expect("failed to establish WebSocket connection");
            let (sender, mut receiver) = ws.split();

            *ws_sender.borrow_mut() = Some(sender);

            let set_conversation = set_conversation.clone();
            let set_is_loading = set_is_loading.clone();

            spawn_local(async move {
                while let Some(msg) = receiver.next().await {
                    match msg {
                        Ok(WsMessage::Text(text)) => {
                            set_conversation.update(|c| {
                                if let Some(last_message) = c.messages.last_mut() {
                                    if last_message.role == "assistant" {
                                        last_message.content.push_str(&text);
                                    } else {
                                        c.messages.push(Message {
                                            id: Uuid::new_v4(),
                                            role: "assistant".to_string(),
                                            content: text,
                                            timestamp: Utc::now().timestamp().to_string(),
                                        });
                                    }
                                }
                            });
                            set_is_loading.set(false);
                        }
                        Err(_) => {
                            set_is_loading.set(false);
                            break;
                        }
                        _ => {}
                    }
                }
            });
        });
    }

    let send_message = create_action(move |new_message: &String| {
        let user_message = Message {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: new_message.clone(),
            timestamp: Utc::now().timestamp().to_string(),
        };

        set_conversation.update(|c| {
            c.messages.push(user_message);
        });

        set_is_loading.set(true);

        let msg = new_message.clone();
        let ws_sender = ws_sender.clone();
        async move {
            if let Some(sender) = ws_sender.borrow_mut().as_mut() {
                sender.send(WsMessage::Text(msg))
                    .await
                    .map_err(|_| ServerFnError::ServerError("WebSocket send error".to_string()))
            } else {
                Err(ServerFnError::ServerError("WebSocket not connected".to_string()))
            }
        }
    });

    view! {
        <div class="min-h-screen w-full flex justify-center">
            <ChatArea conversation/>
            <PromptArea on_submit=send_message is_loading=is_loading/>
        </div>
    }
}

/// 404 - Not Found Component
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="flex flex-col h-screen justify-center items-center bg-gray-100">
            <div class="flex flex-col items-center">
                <h1 class="text-[120px] font-extrabold text-gray-700">"404"</h1>
                <p class="text-2xl font-medium text-gray-600 mb-6">"Page Not Found"</p>
                <a href="/"
                    class="px-4 py-2 font-medium text-white bg-indigo-500 rounded-md hover:bg-indigo-600 transition-all duration-200 ease-in-out">
                    "Go Back Home"
                </a>
            </div>
        </div>
    }
}
