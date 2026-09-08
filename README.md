# leptos-macro

Procedural macros for Leptos components — boilerplate reduction for lib.rs, feature flags, and prop defaults.

## Purpose

Leptos components require repetitive boilerplate: module declarations, feature flag gating, and `impl` blocks for prop defaults. `leptos-macro` eliminates that with a single derive macro.

## Scope (small by design)

This crate is a boilerplate saver for **personal component patterns**. It
complements — not competes with — the official Leptos macros:
`#[component]` and `#[server]` remain the source of truth for component
definitions, reactivity, and server functions.

What it does NOT do:

- No prop-default codegen: `#[prop(default = ...)]` attributes are
  recognized and preserved for Leptos to honor, but the macro emits no
  `Default` impls or builder setters (generated inherent methods could
  collide with yours — deliberately out of scope).
- No component registration, routing, or server-function support.

No new derive conveniences are planned beyond this scaffold: the only
candidates (emitted `Default` impls, generated setters) are not trivially
safe, so the macro stays minimal on purpose.

## Usage

```rust,ignore
use leptos_macros::LeptosComponent;

#[derive(LeptosComponent)]
pub struct CardProps {
    pub title: String,
    #[prop(default = false)]
    pub collapsed: bool,
}
```

The macro generates an `impl` block with a `builder()` entry point,
preserving generics and where clauses. Fields with Leptos' `#[component]`
attribute (e.g. `children`) are skipped.

## Comparison

| Manual boilerplate | With `leptos-macro` |
|--------------------|---------------------|
| ~30 lines per component | 1 derive + attribute |
| Easy to forget a field | All props handled automatically |
| Inconsistent defaults | Declarative `#[prop(default = ...)]` |

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
