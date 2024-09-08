use leptos::*;

/// Renders the promt area of the chat page.
#[component]
pub fn PromptArea(
    on_submit: Action<String, Result<(), ServerFnError>>,
    is_loading: ReadSignal<bool>,
) -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        if !input.get().is_empty() && !is_loading.get() {
            on_submit.dispatch(input.get());
            set_input.set(String::new());
        }
    };

    let handle_input_change = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        set_input.set(value);
    };

    // Handle keyup event to check if "Enter" key was pressed
    let handle_keyup = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" && !ev.shift_key() {
            // Check if Enter is pressed without Shift key
            ev.prevent_default(); // Prevent newline in the textarea
            if !input.get().is_empty() && !is_loading.get() {
                on_submit.dispatch(input.get());
                set_input.set(String::new());
            }
        }
    };

    view! {
        <div class="fixed bottom-0 w-full">
            <div class="bg-white dark:bg-gray-800">
                <div class="max-w-3xl px-2.5 -mb-0.5 mx-auto inset-x-0">
                    <div class="bg-gradient-to-t from-white dark:from-gray-800 from-40% pb-2">
                        <form
                            class="flex flex-col relative w-full rounded-xl border dark:border-gray-600 bg-white dark:bg-gray-800 dark:text-gray-100"
                            on:submit=handle_submit
                        >
                            <div class="flex">
                                <textarea
                                    class="dark:bg-gray-800 dark:text-gray-100 outline-none w-full py-3 px-2 pl-4 rounded-xl resize-none h-[48px]"
                                    rows=3
                                    prop:value=input
                                    placeholder="Send a message"
                                    on:input=handle_input_change
                                    prop:disabled=is_loading
                                    on:keyup=handle_keyup
                                />
                                <div class="self-end mb-2 flex space-x-0.5 mr-2">
                                    <button
                                        class=move || {
                                            let base_classes = "transition rounded-lg p-1 mr-0.5 w-7 h-7 self-center";
                                            if !input.get().is_empty() {
                                                format!("{} bg-black text-white hover:bg-gray-900 dark:bg-white dark:text-black dark:hover:bg-gray-100", base_classes)
                                            } else {
                                                format!("{} text-white bg-gray-100 dark:text-gray-800 dark:bg-gray-600 disabled", base_classes)
                                            }
                                        }
                                        type="submit"
                                        prop:disabled=move || input.get().is_empty() || is_loading.get()
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            viewBox="0 0 20 20"
                                            fill="currentColor"
                                            class="w-5 h-5"
                                        >
                                            <path
                                                fill-rule="evenodd"
                                                d="M10 17a.75.75 0 01-.75-.75V5.612L5.29 9.77a.75.75 0 01-1.08-1.04l5.25-5.5a.75.75 0 011.08 0l5.25 5.5a.75.75 0 11-1.08 1.04l-3.96-4.158V16.25A.75.75 0 0110 17z"
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </button>
                                </div>
                            </div>
                        </form>
                        <div class="mt-1.5 text-xs text-gray-500 text-center">LLMs can make mistakes. Verify important information.</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
