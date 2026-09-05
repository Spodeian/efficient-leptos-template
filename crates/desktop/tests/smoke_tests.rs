#[test]
fn test_desktop_smoke() {
    let context: tauri::Context<tauri::Wry> = tauri::generate_context!();
    assert_eq!(context.config().product_name.as_deref(), Some("Leptos Desktop"));
}
