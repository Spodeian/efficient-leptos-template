//! Native desktop and mobile application runner using Tauri v2.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,desktop=debug")),
        )
        .init();

    tracing::info!("Starting Leptos Native Application (Tauri v2)...");

    tauri::Builder::default()
        .setup(|_app| {
            tracing::info!("Tauri window setup successfully initialized");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
