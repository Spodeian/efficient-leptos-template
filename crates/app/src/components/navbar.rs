//! Responsive top navigation bar with quick action buttons and mobile drawer.

use crate::components::theme::ThemeToggle;
use crate::storage::{query_storage_diagnostics, trigger_pwa_install};
use leptos::prelude::*;
use shared::ThemeMode;

#[component]
pub fn Navbar(
    theme: RwSignal<ThemeMode>,
    show_reset_modal: RwSignal<bool>,
    show_help_modal: RwSignal<bool>,
    show_import_modal: RwSignal<bool>,
    show_export_modal: RwSignal<bool>,
    show_storage_modal: RwSignal<bool>,
    mobile_menu_open: RwSignal<bool>,
) -> impl IntoView {
    let toggle_mobile_menu = move |_| {
        mobile_menu_open.update(|open| *open = !*open);
    };

    let close_mobile_menu = move || {
        mobile_menu_open.set(false);
    };

    let diag = Memo::new(move |_| query_storage_diagnostics());

    view! {
        <header class="app-header">
            <div class="header-container">
                <div class="brand-section">
                    <a href="/" class="brand-logo" on:click=move |_| close_mobile_menu()>
                        <div class="logo-text">
                            <span class="brand-name">"Leptos Serverless"</span>
                            <span class="brand-subtitle">"& Desktop Template"</span>
                        </div>
                    </a>
                    <span class="badge-tech">"Leptos 0.8"</span>
                </div>

                <nav class="desktop-nav">
                    {move || {
                        let d = diag.get();
                        if d.pwa_install_available && !d.is_pwa_installed {
                            view! {
                                <button class="nav-btn nav-btn-accent" on:click=move |_| trigger_pwa_install()>
                                    "Install App"
                                </button>
                            }.into_any()
                        } else {
                            view! {}.into_any()
                        }
                    }}

                    <button class="nav-btn" on:click=move |_| show_storage_modal.set(true)>
                        {move || {
                            let d = diag.get();
                            match d.is_persisted {
                                Some(true) => "Storage (Persistent)",
                                Some(false) => "Storage (Ephemeral)",
                                None => "Storage",
                            }
                        }}
                    </button>
                    <button class="nav-btn" on:click=move |_| show_import_modal.set(true)>
                        "Import"
                    </button>
                    <button class="nav-btn" on:click=move |_| show_export_modal.set(true)>
                        "Export"
                    </button>
                    <button class="nav-btn nav-btn-subtle" on:click=move |_| show_help_modal.set(true)>
                        "Help"
                    </button>
                    <button class="nav-btn nav-btn-danger" on:click=move |_| show_reset_modal.set(true)>
                        "Reset"
                    </button>
                    <ThemeToggle theme=theme />
                </nav>

                <button
                    class="mobile-menu-toggle"
                    on:click=toggle_mobile_menu
                    aria-label="Toggle Navigation Menu"
                >
                    {move || if mobile_menu_open.get() { "✕" } else { "☰" }}
                </button>
            </div>

            // Mobile dropdown drawer
            {move || if mobile_menu_open.get() {
                view! {
                    <div class="mobile-drawer">
                        {move || {
                            let d = diag.get();
                            if d.pwa_install_available && !d.is_pwa_installed {
                                view! {
                                    <button class="drawer-btn drawer-btn-accent" on:click=move |_| {
                                        close_mobile_menu();
                                        trigger_pwa_install();
                                    }>
                                        "Install App"
                                    </button>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }
                        }}
                        <button class="drawer-btn" on:click=move |_| {
                            close_mobile_menu();
                            show_storage_modal.set(true);
                        }>
                            "Storage Diagnostics"
                        </button>
                        <button class="drawer-btn" on:click=move |_| {
                            close_mobile_menu();
                            show_import_modal.set(true);
                        }>
                            "Import Data"
                        </button>
                        <button class="drawer-btn" on:click=move |_| {
                            close_mobile_menu();
                            show_export_modal.set(true);
                        }>
                            "Export Data"
                        </button>
                        <button class="drawer-btn" on:click=move |_| {
                            close_mobile_menu();
                            show_help_modal.set(true);
                        }>
                            "Help & Shortcuts"
                        </button>
                        <button class="drawer-btn drawer-btn-danger" on:click=move |_| {
                            close_mobile_menu();
                            show_reset_modal.set(true);
                        }>
                            "Reset State"
                        </button>
                        <div class="drawer-theme">
                            <ThemeToggle theme=theme />
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {}.into_any()
            }}
        </header>
    }
}
