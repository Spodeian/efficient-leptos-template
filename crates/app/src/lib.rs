//! Main Leptos UI application view controller, layout components, and state synchronization.

pub mod components;
pub mod storage;

use crate::components::theme::{apply_document_theme, get_initial_theme};
use crate::components::{
    ExportModal, HelpModal, ImportModal, ItemList, Navbar, ResetModal, StorageModal,
};
use crate::storage::{
    load_state_from_storage, query_storage_diagnostics, request_persistent_storage,
    trigger_binary_download,
};
use leptos::prelude::*;
use shared::{export_to_compressed_bson, AppState};
use tracing::info;

#[component]
pub fn App() -> impl IntoView {
    // Initialize Theme
    let theme = RwSignal::new(get_initial_theme());
    apply_document_theme(theme.get());

    // Initialize State from local storage or sample data
    let initial_state = load_state_from_storage().unwrap_or_else(|| {
        info!("Initializing new default AppState");
        AppState::new()
    });
    let state = RwSignal::new(initial_state);

    // Modal state signals
    let show_reset_modal = RwSignal::new(false);
    let show_help_modal = RwSignal::new(false);
    let show_import_modal = RwSignal::new(false);
    let show_export_modal = RwSignal::new(false);
    let show_storage_modal = RwSignal::new(false);
    let mobile_menu_open = RwSignal::new(false);

    // Warning banner dismissal signals
    let dismissed_ephemeral = RwSignal::new(false);
    let dismissed_quota = RwSignal::new(false);
    let dismissed_combined = RwSignal::new(false);

    let diag = Memo::new(move |_| query_storage_diagnostics());

    // Global keyboard listener for Escape key to close modals
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Escape" {
            show_reset_modal.set(false);
            show_help_modal.set(false);
            show_import_modal.set(false);
            show_export_modal.set(false);
            show_storage_modal.set(false);
            mobile_menu_open.set(false);
        }
    };

    let on_backup_bson = move |_| {
        let current_state = state.get();
        if let Ok(bytes) = export_to_compressed_bson(&current_state) {
            trigger_binary_download("data_backup.bson", &bytes, "application/octet-stream");
        }
    };

    view! {
        <div class="app-root" on:keydown=on_keydown tabindex="0">
            <Navbar
                theme=theme
                show_reset_modal=show_reset_modal
                show_help_modal=show_help_modal
                show_import_modal=show_import_modal
                show_export_modal=show_export_modal
                show_storage_modal=show_storage_modal
                mobile_menu_open=mobile_menu_open
            />

            <main class="main-wrapper">
                // Diagnostics / Warning Banners
                {move || {
                    let d = diag.get();
                    let is_ephemeral = d.is_persisted == Some(false);
                    let is_quota = d.quota_exceeded;

                    if is_ephemeral && is_quota && !dismissed_combined.get() {
                        view! {
                            <div class="banner-alert banner-danger" style="margin-bottom: 12px; padding: 10px 14px; background: rgba(218, 54, 51, 0.2); border: 1px solid var(--danger); border-radius: 8px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 8px;">
                                <span><strong>"Storage Alert:"</strong> " Storage is Ephemeral AND Quota Limit Exceeded!"</span>
                                <div style="display: flex; gap: 6px;">
                                    <button class="btn btn-sm btn-primary" on:click=on_backup_bson>"Save .bson Backup"</button>
                                    <button class="btn btn-sm btn-outline" on:click=move |_| request_persistent_storage()>"Request Permission"</button>
                                    <button class="btn btn-sm btn-secondary" on:click=move |_| dismissed_combined.set(true)>"Dismiss"</button>
                                </div>
                            </div>
                        }.into_any()
                    } else if is_ephemeral && !dismissed_ephemeral.get() {
                        view! {
                            <div class="banner-alert banner-warning" style="margin-bottom: 12px; padding: 10px 14px; background: rgba(210, 153, 34, 0.2); border: 1px solid var(--warning); border-radius: 8px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 8px;">
                                <span><strong>"Ephemeral Storage:"</strong> " Browser may clear local data under storage pressure."</span>
                                <div style="display: flex; gap: 6px;">
                                    <button class="btn btn-sm btn-primary" on:click=on_backup_bson>"Backup .bson"</button>
                                    <button class="btn btn-sm btn-outline" on:click=move |_| request_persistent_storage()>"Request Persistence"</button>
                                    <button class="btn btn-sm btn-secondary" on:click=move |_| dismissed_ephemeral.set(true)>"Dismiss"</button>
                                </div>
                            </div>
                        }.into_any()
                    } else if is_quota && !dismissed_quota.get() {
                        view! {
                            <div class="banner-alert banner-warning" style="margin-bottom: 12px; padding: 10px 14px; background: rgba(210, 153, 34, 0.2); border: 1px solid var(--warning); border-radius: 8px; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 8px;">
                                <span><strong>"Quota Exceeded:"</strong> " Data migrated to IndexedDB fallback tier."</span>
                                <div style="display: flex; gap: 6px;">
                                    <button class="btn btn-sm btn-primary" on:click=on_backup_bson>"Save .bson Backup"</button>
                                    <button class="btn btn-sm btn-secondary" on:click=move |_| dismissed_quota.set(true)>"Dismiss"</button>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }
                }}

                <ItemList state=state />
            </main>

            <footer class="app-footer">
                <div class="footer-container">
                    <p>
                        "Built with "
                        <strong>"Rust & Leptos 0.8"</strong>
                        " • Serverless WASM (Cloudflare Pages) & Native Desktop (Tauri v2)"
                    </p>
                    <div class="footer-links">
                        <a href="https://github.com/Spodeian/Revisited-IPIP-NEO" target="_blank" rel="noopener noreferrer">
                            "Inspired by Revisited IPIP-NEO"
                        </a>
                        <span class="footer-dot">"•"</span>
                        <a href="https://leptos.dev" target="_blank" rel="noopener noreferrer">
                            "Leptos Docs"
                        </a>
                    </div>
                </div>
            </footer>

            // Modals
            <ResetModal is_open=show_reset_modal state=state />
            <HelpModal is_open=show_help_modal />
            <ImportModal is_open=show_import_modal state=state />
            <ExportModal is_open=show_export_modal state=state />
            <StorageModal is_open=show_storage_modal show_import_modal=show_import_modal state=state />
        </div>
    }
}
