#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Procedural macros for personal Leptos component patterns.
//!
//! # Scope
//!
//! This crate is a small boilerplate saver for personal component patterns:
//! it generates a `builder()` entry point for props structs (returning
//! `Self::default()`, so the struct must implement `Default`) while
//! tolerating Leptos' `#[prop(...)]` and `#[component]` attributes. It is
//! small by design and intentionally stays that way.
//!
//! It complements — not competes with — the official Leptos macros:
//! `#[component]` and `#[server]` remain the source of truth for component
//! definitions, reactivity, and server functions. Use this crate only for
//! the repetitive `impl`-block glue around your own props conventions.
//!
//! # What it does NOT do
//!
//! - No prop-default codegen: `#[prop(default = ...)]` attributes are
//!   recognized (and left for Leptos to honor) but this macro emits no
//!   `Default` impls or builder setters — generated inherent methods could
//!   collide with yours, so that is deliberately out of scope.
//! - No component registration, routing, or server-function support.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Fields, parse_macro_input};

/// Derive macro for Leptos props-struct boilerplate.
///
/// Generates a `builder()` entry point that returns `Self::default()` —
/// the struct must implement `Default` (this macro does not emit the
/// `Default` impl itself). Fields carrying Leptos' `#[component]`
/// attribute (e.g. `children`) are tolerated; `#[prop(default = ...)]`
/// attributes are tolerated and preserved for Leptos to honor. Generics
/// and where clauses are preserved.
///
/// Scope: a small, personal-patterns helper that complements the official
/// `#[component]` / `#[server]` macros — not a replacement for them. Kept
/// small by design: no `Default` impls, no setters, no codegen beyond the
/// `impl` scaffold.
///
/// # Usage
///
/// ```ignore
/// use leptos_macros::LeptosComponent;
///
/// #[derive(Default, LeptosComponent)]
/// pub struct CardProps {
///     pub title: String,
///     #[prop(default = false)]
///     pub collapsed: bool,
/// }
///
/// let props = CardProps::builder();
/// ```
#[proc_macro_derive(LeptosComponent, attributes(component, prop))]
pub fn leptos_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_leptos_component(&input);
    TokenStream::from(expanded)
}

fn impl_leptos_component(input: &DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(_) => {}
            _ => panic!("LeptosComponent only supports structs with named fields"),
        },
        _ => panic!("LeptosComponent only supports structs"),
    }

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn builder() -> Self
            where
                Self: Default,
            {
                Self::default()
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;

    fn expand_derive(input: TokenStream2) -> TokenStream2 {
        let input: DeriveInput = syn::parse2(input).expect("valid DeriveInput");
        impl_leptos_component(&input)
    }

    #[test]
    fn named_fields_struct_generates_builder() {
        let input = quote! {
            struct MyComponent {
                name: String,
                enabled: bool,
            }
        };
        let expanded = expand_derive(input);
        let code = expanded.to_string();
        assert!(
            code.contains("fn builder"),
            "should generate builder method"
        );
        assert!(code.contains("MyComponent"), "should reference struct name");
    }

    #[test]
    fn struct_with_prop_default_attr() {
        let input = quote! {
            struct MyComponent {
                name: String,
                #[prop(default = false)]
                enabled: bool,
            }
        };
        let expanded = expand_derive(input);
        let code = expanded.to_string();
        assert!(
            code.contains("fn builder"),
            "should generate builder method"
        );
    }

    #[test]
    fn struct_with_component_attr_filtered() {
        let input = quote! {
            struct MyComponent {
                #[component]
                children: Option<String>,
                name: String,
            }
        };
        let expanded = expand_derive(input);
        let code = expanded.to_string();
        assert!(code.contains("fn builder"), "should generate builder");
    }

    #[test]
    fn generics_preserved() {
        let input = quote! {
            struct MyComponent<T> {
                value: T,
            }
        };
        let expanded = expand_derive(input);
        let code = expanded.to_string();
        assert!(
            code.contains("MyComponent"),
            "should preserve generic struct name"
        );
    }

    #[test]
    fn expected_output_matches() {
        let input = quote! {
            struct TestComp {
                field: String,
            }
        };
        let expanded = expand_derive(input);
        let expected = quote! {
            impl TestComp {
                pub fn builder() -> Self
                where
                    Self: Default,
                {
                    Self::default()
                }
            }
        };
        assert_eq!(expanded.to_string(), expected.to_string());
    }

    #[test]
    fn builder_requires_default_bound() {
        let input = quote! {
            struct TestComp {
                field: String,
            }
        };
        let expanded = expand_derive(input).to_string();
        assert!(
            expanded.contains("Self : Default"),
            "builder() should carry `where Self: Default` so it only exists on Default structs"
        );
        assert!(
            expanded.contains("Self :: default ()"),
            "builder() should construct via Self::default()"
        );
    }
}
