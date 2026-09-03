//! Theme management component with instantaneous DOM synchronization and persistence.

use leptos::prelude::*;
use shared::ThemeMode;
use tracing::info;

/// Applies the theme to the HTML document element.
pub fn apply_document_theme(_theme: ThemeMode) {
    #[cfg(target_arch = "wasm32")]
    {
        let theme = _theme;
        if let Some(window) = web_sys::window() {
            if let Some(doc) = window.document() {
                if let Some(html) = doc.document_element() {
                    let theme_str = if theme.is_dark() { "dark" } else { "light" };
                    let _ = html.set_attribute("data-theme", theme_str);
                }
            }
            if let Ok(Some(storage)) = window.local_storage() {
                let theme_str = if theme.is_dark() { "dark" } else { "light" };
                let _ = storage.set_item(crate::storage::STORAGE_KEY_THEME, theme_str);
            }
        }
    }
}

/// Reads the stored theme or defaults to Dark mode.
pub fn get_initial_theme() -> ThemeMode {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(val)) = storage.get_item(crate::storage::STORAGE_KEY_THEME) {
                    if val == "light" {
                        return ThemeMode::Light;
                    }
                }
            }
        }
    }
    ThemeMode::Dark
}

#[component]
pub fn ThemeToggle(theme: RwSignal<ThemeMode>) -> impl IntoView {
    let on_toggle = move |_| {
        let new_theme = theme.get().toggle();
        theme.set(new_theme);
        apply_document_theme(new_theme);
        info!("Theme switched to: {:?}", new_theme);
    };

    view! {
        <button
            class="btn-theme-toggle"
            on:click=on_toggle
            title=move || if theme.get().is_dark() { "Switch to Warm Light Mode" } else { "Switch to Dark Mode" }
            aria-label="Toggle Theme"
        >
            {move || if theme.get().is_dark() {
                "Light Mode"
            } else {
                "Dark Mode"
            }}
        </button>
    }
}
