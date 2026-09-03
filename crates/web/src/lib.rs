//! WebAssembly client entry point for the Leptos template.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() {
    // Better panic errors in browser console
    console_error_panic_hook::set_once();

    // Initialize tracing logging for browser console
    let _ = tracing_wasm::set_as_global_default();

    tracing::info!("Initializing Serverless Leptos WASM application...");

    // Mount Leptos root component to document body
    leptos::mount::mount_to_body(app::App);
}
