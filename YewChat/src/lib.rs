#![recursion_limit = "512"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="font-family: Arial; text-align: center; background: linear-gradient(135deg, #e8f5e9, #bbdefb); min-height: 100vh; padding: 40px;">
            <h1>{"Qowiy WebChat"}</h1>
            <p>{"Welcome to my creative WebSocket chat application using Rust and Yew!"}</p>

            <div style="margin: 30px auto; padding: 20px; max-width: 500px; background: white; border-radius: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.15);">
                <h2>{"Real-Time Chat"}</h2>
                <p>{"This page was modified for Experiment 3.2: Be Creative!"}</p>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}