# leptos-macro

Procedural macros for Leptos components — boilerplate reduction for lib.rs, feature flags, and prop defaults.

## Purpose

Leptos components require repetitive boilerplate: module declarations, feature flag gating, and `impl` blocks for prop defaults. `leptos-macro` eliminates that with a single derive macro.

## Usage

```rust,ignore
use leptos_macro::LeptosComponent;

#[derive(LeptosComponent)]
#[component]
fn MyComponent(
    title: String,
    #[prop(default = false)] visible: bool,
) -> impl IntoView {
    view! { <div>{title}</div> }
}
```

The macro generates:
- Module declarations for component isolation
- Feature flag setup (mirrors `cfg` attributes)
- Prop default implementations and builder methods

## Comparison

| Manual boilerplate | With `leptos-macro` |
|--------------------|---------------------|
| ~30 lines per component | 1 derive + attribute |
| Easy to forget a field | All props handled automatically |
| Inconsistent defaults | Declarative `#[prop(default = ...)]` |

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
