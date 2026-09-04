//! WebAssembly client entry point for the Leptos template.

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js() {
    // Better panic errors in browser console
    console_error_panic_hook::set_once();

    // Initialize tracing logging for browser console
    let _ = tracing_wasm::set_as_global_default();

    #[cfg(feature = "hydrate")]
    {
        tracing::info!("Hydrating Serverless Leptos application from server-rendered HTML...");
        leptos::mount::hydrate_body(app::App);
    }

    #[cfg(not(feature = "hydrate"))]
    {
        tracing::info!("Mounting Serverless Leptos application in CSR fallback mode...");
        leptos::mount::mount_to_body(app::App);
    }
}
