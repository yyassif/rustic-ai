use crate::model::conversation::Conversation;
use leptos::{html::Div, *};
use pulldown_cmark::{html, Options, Parser};

/// Renders the chat area of the chat page.
#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div class="py-2.5 flex flex-col justify-between w-full">
            <div class="h-full mt-10 mb-32 w-full flex flex-col">
                <div class="w-full">
                    <div class="flex justify-between px-5 mb-3 max-w-4xl mx-auto rounded-lg group flex-col" node_ref=chat_div_ref>
                        {move || conversation.get().messages.iter().map(move |message| {
                            view! {
                            <Message
                                message=message.content.clone()
                            />
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Renders the Message component for the ChatArea.
#[component]
fn Message(message: String) -> impl IntoView {
    let (is_btn_pressed, set_is_btn_pressed) = create_signal(false);

    create_effect(move |_| {
        if is_btn_pressed.get() {
            set_timeout(
                move || {
                    set_is_btn_pressed.set(false);
                },
                2000,
            );
        }
    });

    view! {
        <div class="group w-full text-gray-800 dark:text-gray-100 border-b border-gray-700 last:border-b-0 text-balance">
            <div class="text-base gap-4 md:gap-6 flex lg:px-0 m-auto w-full text-balance">
                <div class="flex flex-row gap-4 md:gap-6 p-4 md:py-6 lg:px-0 m-auto w-full text-balance">
                    <div class="w-8 flex flex-col relative items-end">
                        <div class="relative h-7 w-7 p-1 rounded-sm text-white flex items-center justify-center text-opacity-100">
                            <div class="absolute h-8 w-8 rounded-full bg-gradient-to-r from-green-300 to-purple-400"></div>
                        </div>
                    </div>
                    <div class="relative flex w-[calc(100%-50px)] flex-row gap-1 md:gap-3 lg:w-[calc(100%-115px)]">
                        <div class="flex flex-grow flex-col gap-3 text-balance w-full">
                            <Markdown markdown=message />
                        </div>
                        <div class="flex space-x-2">
                            <button
                                on:click=move |_| {
                                    set_is_btn_pressed.set(true);
                                }
                                class="flex items-center justify-center w-8 h-8 rounded-full bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                            >
                                {move || if !is_btn_pressed.get() {
                                    view! { <ClipboardIcon /> }
                                } else {
                                    view! { <CheckIcon /> }
                                }}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Renders the Markdown Parser.
#[component]
pub fn Markdown(markdown: String) -> impl IntoView {
    let parsed_html = create_memo(move |_| {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);

        let parser = Parser::new_ext(&markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    });

    view! {
        <div class="markdown-body" inner_html=parsed_html/>
    }
}

/// SVG ClipBoard Component.
#[component]
fn ClipboardIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="w-4 h-4 text-gray-400 group-hover:text-gray-500"
        >
            <rect width="8" height="4" x="8" y="2" rx="1" ry="1"/>
            <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>
        </svg>
    }
}

// SVG CheckIcon Component.
#[component]
fn CheckIcon() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="w-4 h-4 text-green-400 group-hover:text-green-500"
        >
            <polyline points="20 6 9 17 4 12"/>
        </svg>
    }
}

/// Util function for setting timeouts via WASM & Web-Sys.
fn set_timeout<F>(f: F, millis: i32)
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::prelude::*;

    let win = web_sys::window().expect("no global `window` exists");
    let closure = Closure::once(f);
    win.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        millis,
    )
    .expect("failed to set timeout");
    closure.forget();
}
