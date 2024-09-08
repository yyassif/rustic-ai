# RusticAI - A Fast and Scalable Chatbot Built with Leptos and Ollama-rs

RusticAI is an advanced chatbot developed using the Leptos web framework and Ollama-rs for maintaining chat conversations. Designed for performance and scalability, this chatbot leverages the power of Rust's concurrency and memory safety to deliver fast and intelligent conversations.

<picture>
<img src="https://raw.githubusercontent.com/yyassif/rustic-ai/main/assets/demo.png" />
</picture>

## Features

* Built with Leptos for responsive and dynamic user interfaces.
* Powered by Ollama-rs for robust language processing.
* High-performance, memory-safe architecture using Rust.
* Easily scalable with support for async operations.
* Optimized for both real-time interactions and efficient model serving.

Perfect for developers looking to build reliable and scalable AI-driven chat applications!

## Future Works

- [ ] Implement user authentication
  - Secure access to personal chat histories
  - Enable user-specific features and preferences

- [ ] Integrate database storage for conversations
  - Persist chat histories across sessions
  - Enable retrieval and analysis of past conversations

## Setup Instructions

### Hardware

By default, the project is configured for Nvidia GPU acceleration using CUDA. If you're running the chatbot on a macOS system with Apple Silicon, you can test it with Metal acceleration by enabling the `metal` feature in `Cargo.toml`. If you encounter issues or successfully configure for other platforms, feel free to submit a PR to update the `README.md`.

### Rust Toolchain

You'll need to use the nightly Rust toolchain, and install the `wasm32-unknown-unknown` target as well as the Trunk and `cargo-leptos` tools:

```bash
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli cargo-leptos
```
### Models

You'll also need to install [Ollama](https://ollama.com/download), and download a models (i.e [Llama3.1](https://ollama.com/library/llama3.1)) or any model of your choice.

In the root of the project directory, you'll find a `.env` file where two environment variables called `OLLAMA_SYSTEM_PROMPT` & `OLLAMA_MODEL_NAME` are defined. Replace these values with the the desired model and prompt.

```bash
OLLAMA_SYSTEM_PROMPT=
OLLAMA_MODEL_NAME=
```

### TailwindCSS

Install TailwindCSS with `npm install -D sass tailwindcss`

### Run

To run the project locally,

1. run `npx sass style/main.scss | npx tailwindcss -i - -o style/main.css` in a terminal - this will build `style/main.css` and automatically rebuild when a change is detected in `styles/main.css` in the `Cargo.toml` file.

2. `cargo leptos watch` in the project directory.

3. In in your browser, navigate to [http://localhost:3000/](http://localhost:3000/).

## Tested Models

The following list of models was seemless in terms of integration and I did't have any sort of problems working with them.

* [Llama3.1](https://ollama.com/library/llama3.1).
* [Phi3.5](https://ollama.com/library/phi3.5).
* [Gemma2](https://ollama.com/library/gemma2).

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
