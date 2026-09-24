# Serverless & Full-Stack Leptos Template Documentation Standards

**Leptos 0.8 Architecture, Reactive Signal Conventions, and Quality Enforcement**

---

## 1. Overview & Core Philosophy

This template provides a production-ready, reactive web and desktop architecture powered by Rust and Leptos 0.8. All components, server functions, stores, and router definitions must be documented with clarity and precision.

### The Five Pillars:
1. **Architectural Clarity**: Clear separation between shared domain models (`crates/shared`), full-stack UI components (`crates/app`), desktop runner (`crates/desktop`), and static WASM client (`crates/web`).
2. **Reactive Signal Invariants**: Explicit documentation of fine-grained reactive signals (`RwSignal`, `Memo`, `Resource`), ownership semantics, and reactivity scopes.
3. **Hydration & SSR Hygiene**: Clear demarcation of code executing on Server vs. Client (WASM) to eliminate hydration mismatches.
4. **Zero-Warning Hygiene**: `cargo doc --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` must compile with zero warnings.
5. **Strict Test Isolation**: All tests reside in dedicated test files under `tests/`; no inline tests inside production source files.

---

## 2. Component Documentation Conventions

Document components with their input props, signals, and slot behaviors:

```rust
/// Interactive header bar with reactive theme toggle and navigation.
///
/// ## Reactive Signals
/// - Consumes [`ThemeSignal`] from the Leptos context hierarchy.
#[component]
pub fn Navbar() -> impl IntoView {
    // ...
}
```

---

## 3. Verification Checklist

Before opening PRs to `main`:
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes with 0 warnings.
- [ ] `cargo test --workspace` passes 100% of integration tests.
- [ ] No `DOCUMENTATION_STANDARDS.md` or internal roadmap files are included on `main`.
