//! Browser multi-tiered storage synchronization, persistence manager, and platform interaction utilities.

use serde::{Deserialize, Serialize};
use shared::AppState;
use spodeian_cache::{ContentAddressedStorage, PreferentialRouter};
#[allow(unused_imports)]
use spodeian_cache::StorageTier;
#[allow(unused_imports)]
use tracing::{error, info, warn};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

pub const STORAGE_KEY_STATE: &str = "serverless_leptos_app_state";
pub const STORAGE_KEY_THEME: &str = "serverless_leptos_theme";
pub const STORAGE_KEY_DYSLEXIA: &str = "serverless_leptos_dyslexia";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StorageBackend {
    #[default]
    LocalStorage,
    IndexedDb,
    CacheApi,
    NativeCas,
    MemoryOnly,
}

impl StorageBackend {
    pub fn label(self) -> &'static str {
        match self {
            Self::LocalStorage => "Local Storage (Fast Tier)",
            Self::IndexedDb => "IndexedDB (Structured Relational Tier)",
            Self::CacheApi => "Cache API (Large Binary Weights Tier)",
            Self::NativeCas => "Native CAS (Content-Addressed Disk Tier)",
            Self::MemoryOnly => "In-Memory Only (Ephemeral)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StorageDiagnostics {
    pub is_persisted: Option<bool>,
    pub pwa_install_available: bool,
    pub is_pwa_installed: bool,
    pub backend: StorageBackend,
    pub quota_exceeded: bool,
    pub idb_active: bool,
}

/// Query current storage persistence and PWA status from browser environment
#[allow(unused_mut)]
pub fn query_storage_diagnostics() -> StorageDiagnostics {
    let mut diag = StorageDiagnostics::default();

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(val) = js_sys::Reflect::get(
                &window,
                &wasm_bindgen::JsValue::from_str("__pwaInstallAvailable"),
            ) {
                diag.pwa_install_available = val.as_bool().unwrap_or(false);
            }
            if let Ok(val) =
                js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__pwaInstalled"))
            {
                diag.is_pwa_installed = val.as_bool().unwrap_or(false);
            }
            if let Ok(val) = js_sys::Reflect::get(
                &window,
                &wasm_bindgen::JsValue::from_str("__storagePersisted"),
            ) {
                if let Some(b) = val.as_bool() {
                    diag.is_persisted = Some(b);
                }
            }
        }
    }

    diag
}

/// Request persistent storage from the browser
pub fn request_persistent_storage() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(func) = js_sys::Reflect::get(
                &window,
                &wasm_bindgen::JsValue::from_str("__requestPersistentStorage"),
            ) {
                if let Some(func) = func.dyn_ref::<js_sys::Function>() {
                    let _ = func.call0(&window);
                    info!("Triggered __requestPersistentStorage from Leptos");
                }
            }
        }
    }
}

/// Trigger native PWA install prompt
pub fn trigger_pwa_install() {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(func) = js_sys::Reflect::get(
                &window,
                &wasm_bindgen::JsValue::from_str("__triggerPWAInstall"),
            ) {
                if let Some(func) = func.dyn_ref::<js_sys::Function>() {
                    let _ = func.call0(&window);
                    info!("Triggered __triggerPWAInstall from Leptos");
                }
            }
        }
    }
}

/// Loads the persistent application state from local storage.
pub fn load_state_from_storage() -> Option<AppState> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json_str)) = storage.get_item(STORAGE_KEY_STATE) {
                    match shared::import_from_json(&json_str) {
                        Ok(state) => {
                            info!("Successfully loaded app state from local storage");
                            return Some(state);
                        }
                        Err(err) => {
                            error!("Failed to parse state from local storage: {}", err);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Saves the current application state using preferential multi-tiered routing:
/// - Cache API for large binaries / models (> 512 KB)
/// - IndexedDB for structured relational states
/// - LocalStorage for lightweight config (< 16 KB)
/// - ContentAddressedStorage on native desktop & mobile
pub fn save_state_to_storage(state: &AppState) -> StorageBackend {
    if let Ok(json_str) = shared::export_to_json(state) {
        let size = json_str.len();
        let is_large_or_binary = size > 512 * 1024;
        let recommended_tier = PreferentialRouter::determine_tier(size, "application/json", is_large_or_binary);
        let _ = &recommended_tier;

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                // Tier 1 (Large/Binary): Cache API
                if recommended_tier == StorageTier::CacheApi {
                    if let Ok(func) = js_sys::Reflect::get(
                        &window,
                        &wasm_bindgen::JsValue::from_str("__saveToCacheApi"),
                    ) {
                        if let Some(func) = func.dyn_ref::<js_sys::Function>() {
                            let k = wasm_bindgen::JsValue::from_str(STORAGE_KEY_STATE);
                            let v = wasm_bindgen::JsValue::from_str(&json_str);
                            let _ = func.call2(&window, &k, &v);
                            info!("Preferentially stored large asset to Cache API [{}]", STORAGE_KEY_STATE);
                            return StorageBackend::CacheApi;
                        }
                    }
                }

                // Tier 2: Try localStorage for fast session data if small
                if size < 16 * 1024 {
                    if let Ok(Some(storage)) = window.local_storage() {
                        if storage.set_item(STORAGE_KEY_STATE, &json_str).is_ok() {
                            return StorageBackend::LocalStorage;
                        }
                    }
                }

                // Tier 3: IndexedDB for structured entities and fallback
                if let Ok(func) = js_sys::Reflect::get(
                    &window,
                    &wasm_bindgen::JsValue::from_str("__saveToIndexedDB"),
                ) {
                    if let Some(func) = func.dyn_ref::<js_sys::Function>() {
                        let k = wasm_bindgen::JsValue::from_str(STORAGE_KEY_STATE);
                        let v = wasm_bindgen::JsValue::from_str(&json_str);
                        let _ = func.call2(&window, &k, &v);
                        info!("Saved structured state to IndexedDB [{}]", STORAGE_KEY_STATE);
                        return StorageBackend::IndexedDb;
                    }
                }
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let cas_dir = std::env::temp_dir().join("leptos_template_cas_store");
            if let Ok(cas) = ContentAddressedStorage::new(&cas_dir) {
                let _ = cas.put(json_str.as_bytes());
                return StorageBackend::NativeCas;
            }
        }
    }

    StorageBackend::MemoryOnly
}

/// Triggers a browser file download for exported text data.
pub fn trigger_file_download(filename: &str, content: &str, mime_type: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let blob_parts = js_sys::Array::new();
                blob_parts.push(&wasm_bindgen::JsValue::from_str(content));
                let blob_props = web_sys::BlobPropertyBag::new();
                blob_props.set_type(mime_type);
                match web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &blob_props) {
                    Ok(blob) => {
                        if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                            if let Ok(element) = document.create_element("a") {
                                if let Ok(anchor) = element.dyn_into::<web_sys::HtmlAnchorElement>()
                                {
                                    anchor.set_href(&url);
                                    anchor.set_download(filename);
                                    anchor.click();
                                    let _ = web_sys::Url::revoke_object_url(&url);
                                    info!("Triggered client-side download for '{}'", filename);
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to create download blob: {:?}", e),
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = mime_type;
        match std::fs::write(filename, content) {
            Ok(()) => info!("Successfully wrote local file: {}", filename),
            Err(e) => error!("Failed to write export file '{}': {}", filename, e),
        }
    }
}

/// Triggers client-side binary download (e.g., Compressed BSON).
pub fn trigger_binary_download(filename: &str, bytes: &[u8], mime_type: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                let uint8_array = js_sys::Uint8Array::from(bytes);
                let blob_parts = js_sys::Array::new();
                blob_parts.push(&uint8_array.buffer());
                let blob_props = web_sys::BlobPropertyBag::new();
                blob_props.set_type(mime_type);
                if let Ok(blob) =
                    web_sys::Blob::new_with_u8_array_sequence_and_options(&blob_parts, &blob_props)
                {
                    if let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) {
                        if let Ok(element) = document.create_element("a") {
                            if let Ok(anchor) = element.dyn_into::<web_sys::HtmlAnchorElement>() {
                                anchor.set_href(&url);
                                anchor.set_download(filename);
                                anchor.click();
                                let _ = web_sys::Url::revoke_object_url(&url);
                                info!("Triggered client-side binary download for '{}'", filename);
                            }
                        }
                    }
                }
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = mime_type;
        match std::fs::write(filename, bytes) {
            Ok(()) => info!("Successfully wrote binary file: {}", filename),
            Err(e) => error!("Failed to write binary export file '{}': {}", filename, e),
        }
    }
}

/// Copies content to the system clipboard.
pub fn copy_to_clipboard(text: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            let clipboard = navigator.clipboard();
            let _ = clipboard.write_text(text);
            info!("Copied {} bytes to clipboard", text.len());
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = text;
    }
}
