# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed
- spodeian-core v3.3.0: native content stores now delete superseded data instead of keeping every version on disk.
- Storage loads every item in one IndexedDB transaction and saves all changed and removed items in one transaction each (`spodeian-cache` 1.2 batch API), instead of one sequential transaction per item.
- Service worker: `/fonts/` is served cache-first, like `/pkg/` (`_headers` already marks it immutable), and cache writes are kept alive with `waitUntil`.

### Added
- **Edge mode** replaces `efficient-leptos-edge-template`: answering No to the Tauri prompt (`-d include_tauri_desktop=false`) generates a pure serverless edge site with no `crates/desktop`, Tauri bridge, `csr` features, Trunk shell, mobile scripts or mobile toolchain targets.
- Schema.org `WebApplication` JSON-LD and a `twitter:card` tag in every pre-rendered page (from the edge template).
- `template.yml` CI generates the template in desktop and edge modes and lints and tests both; it is not copied into generated projects.

### Fixed
- `cargo generate` failed with "Substitution skipped, found invalid syntax" and created no files: workflows, `crates/shared/src/export.rs` and binary assets are now copied without Liquid substitution.
- `CHANGELOG.md`, `CONTRIBUTING.md`, `Cargo.lock` and build output are no longer copied into generated projects (moved from `exclude` to `ignore`).
- Generated projects take their own name in `wrangler.toml` and the `repository` URL instead of deploying over this template's Worker; this repo's own deploy passes `--name` from the repository name.

### Changed
- Desktop-only CI (CSR lint, CSR browser tests, Android/iOS checks) moved to `desktop.yml`, which edge-mode projects do not get; `ci.yml` browser tests now run the `hydrate` build.
- `scripts/build-web.sh` also requires `_headers`, `sw.js` and `manifest.json` when the project ships them (from the edge template).
- `template-scaffold-check` CI now runs real `cargo generate` (defaults and every option off) instead of deleting crates to simulate it.
- Lint policy lives in `[workspace.lints]` (`unsafe_code = "deny"`, `clippy::unwrap_used = "warn"`) with `clippy.toml` allowing unwraps in tests; `missing_docs` stays on each library crate root.
- `escape_html` and the BSON import reuse `spodeian-export` instead of local copies.
- `base64` pinned to 0.22 to match spodeian-core, so builds carry one copy; Dependabot skips its minor/major bumps.

### Removed
- Unused dependencies: direct `miniz_oxide`, and core crates no crate called.

## [0.2.3] - 2026-11-18

### Added
- `spodeian-parallel` integration in `crates/shared` and `crates/desktop` powering multi-threaded Rayon sweeps on native/SSR/Tauri and seamless sequential SIMD128 loops on `wasm32` (`sample_points`, `batch_harmonic_sweep`).
- Bundled WebGPU WGSL compute shader (`PARAMETRIC_HARMONIC_WGSL_SHADER`), parity reference (`evaluate_harmonic_wgsl_cpu_reference`), and `ComputeRuntimeSummary` exposed in both `AnalyticsPage` and `AboutPage`.
- Typed Tauri v2 backend IPC commands (`desktop_runtime_summary` and `compute_harmonic_sweep_native`) in `crates/desktop/src/lib.rs`, plus `runtime_environment_label()` and async `invoke_tauri_command()` in `crates/app/src/tauri_bridge.rs`.
- Dedicated [`ARCHITECTURE.md`](ARCHITECTURE.md) and 3-template comparison table in [`README.md`](README.md).

## [0.2.2] - 2026-11-18

### Added
- `AppPage::Analytics` (`/analytics`) route with an SSR-safe reactive priority breakdown SVG bar chart and interactive damped harmonic waveform explorer (`ParametricPreviewConfig`).
- Self-contained SVG (`export_to_svg`), printable HTML (`export_to_printable_html`), and compressed `#state=<base64url>` shareable URL hash (`encode_shareable_url_hash` / `decode_shareable_url_hash`) interchange, plus standalone `ItemCollection` JSON import fallback and async `write_clipboard_text` confirmation in `ExportModal`.
- Bounded 64-step `UndoStack<ItemCollection>` (`Ctrl+Z` / `Ctrl+Y` and navbar Undo/Redo controls), `"Clear Completed"` batch action, searchable `CommandPaletteModal` (`Ctrl+K` / `Cmd+K`), and `tauri_bridge::is_tauri_desktop()` runtime detection.
- Automated `sitemap.xml` and `robots.txt` generation in `crates/ssg`, runnable `crates/shared/examples/state_interchange_and_report.rs`, pre-paint dyslexia synchronization (`THEME_PREPAINT_SCRIPT`), `<noscript>` fallback, and OpenGraph + canonical link metadata in `PageMeta`.

### Changed
- Fixed pre-paint dyslexia FOUT across SSR (`shell.rs`) and Desktop CSR (`index.html`), replaced emoji theme glyphs with inline SVGs, and applied `trap_tab_within_element` focus trapping across all modal dialogs.
- Fixed `cargo-generate.toml` (`crates/desktop` conditional), switched workspace `members = ["crates/*"]` and synced crate/Tauri versions to `0.2.2`, enabled `#![warn(missing_docs)]` and `#![warn(clippy::unwrap_used)]` on `shared` and `app`, and expanded CI (>2 MB file check, `RUSTDOCFLAGS="-D warnings"`, examples/benches, `analytics.html`/`sitemap.xml`/`robots.txt` build assertions, and `template-scaffold-check`).

## [0.2.0] - 2026-11-14

### Added
- `AppState::schema_version` (`CURRENT_SCHEMA_VERSION = 1`) backed by `spodeian_persist::StateMigrator` for forward-compatible JSON snapshots and legacy unversioned payload upgrades.
- Recoverable `<ErrorBoundary>` around application routes with `ErrorFallbackView` (`role="alert"`, details list, and reactive reset action).
- Interactive `cargo-generate.toml` configuration supporting optional Tauri v2 desktop, offline PWA, and Cloudflare Pages scaffolding.
- `.github/dependabot.yml` automation for weekly `cargo` and `github-actions` updates.
- Property-based tests (`proptest`) covering versioned JSON/BSON serialization, legacy schema migration, RFC 4180 CSV round-trips, and theme cycle normalization, plus SSR component tests for `ErrorFallbackView` and `AboutPage`.

### Changed
- Deduplicated static site generation (`crates/ssg`), DOM theme/dyslexia bridges, modal focus trapping, telemetry KPI cards, and PWA diagnostics onto `spodeian-core` (`spodeian-web-utils::pwa`, `spodeian-persist`, and `spodeian-leptos-ui`).
- Replaced the hardcoded `"Version 0.1.0"` badge in `AboutPage` with dynamic `env!("CARGO_PKG_VERSION")`.

## [0.1.0] - 2026-08-18

### Added
- Initial multi-crate Leptos 0.8 + Tauri v2 + Cloudflare Pages template with static SSR + hydration, Trunk desktop CSR, `spodeian-cache` persistence, WCAG AA contrast verification, and offline PWA support.
