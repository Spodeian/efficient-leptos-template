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
        let has_ssr_content = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|doc| doc.body())
            .map(|body| {
                let children = body.children();
                let mut app_nodes = 0;
                for i in 0..children.length() {
                    if let Some(el) = children.item(i) {
                        let tag = el.tag_name().to_ascii_lowercase();
                        if tag != "script" && tag != "noscript" && tag != "style" {
                            app_nodes += 1;
                        }
                    }
                }
                app_nodes > 0
            })
            .unwrap_or(false);

        if has_ssr_content {
            tracing::info!("Hydrating Serverless Leptos application from server-rendered HTML...");
            leptos::mount::hydrate_body(app::App);
        } else {
            tracing::info!("Static HTML shell detected; mounting Serverless Leptos application via mount_to_body...");
            leptos::mount::mount_to_body(app::App);
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        tracing::info!("Mounting Serverless Leptos application in CSR fallback mode...");
        leptos::mount::mount_to_body(app::App);
    }
}
