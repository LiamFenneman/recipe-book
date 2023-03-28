#![allow(clippy::let_with_type_underscore)]

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod home;
pub mod recipe;
pub mod storage;
pub mod add_recipe;
pub mod chatgpt;

mod app;
use app::*;

#[wasm_bindgen(start)]
pub fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! {
            cx, <App />
        }
    });
}
