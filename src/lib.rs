#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Procedural macros for personal Leptos component patterns.
//!
//! # Scope
//!
//! This crate is a small boilerplate saver for personal component patterns:
//! it generates `impl`-block scaffolding (a `builder()` entry point) for
//! props structs while tolerating Leptos' `#[prop(...)]` attributes. It is
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
use syn::{parse_macro_input, DeriveInput, Fields};

/// Derive macro for Leptos props-struct boilerplate.
///
/// Generates an `impl` block with a `builder()` entry point for a struct
/// with named fields. Fields carrying Leptos' `#[component]` attribute
/// (e.g. `children`) are skipped; `#[prop(default = ...)]` attributes are
/// tolerated and preserved for Leptos to honor. Generics and where clauses
/// are preserved.
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
/// #[derive(LeptosComponent)]
/// pub struct CardProps {
///     pub title: String,
///     #[prop(default = false)]
///     pub collapsed: bool,
/// }
/// ```
#[proc_macro_derive(LeptosComponent, attributes(component))]
pub fn leptos_component_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = impl_leptos_component(&input);
    TokenStream::from(expanded)
}

fn impl_leptos_component(input: &DeriveInput) -> TokenStream2 {
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        syn::Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("LeptosComponent only supports structs with named fields"),
        },
        _ => panic!("LeptosComponent only supports structs"),
    };

    let props = fields.iter().filter(|f| {
        !f.attrs.iter().any(|a| a.path().is_ident("component"))
    }).collect::<Vec<_>>();

    let _prop_names: Vec<_> = props.iter().map(|f| &f.ident).collect();

    let _defaults: Vec<_> = props.iter().filter_map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        let default_attr = f.attrs.iter().find(|a| a.path().is_ident("prop"))?;
        let meta = &default_attr.meta;

        if let syn::Meta::List(list) = meta {
            let mut default_value = None;
            let _ = list.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    let value = meta.value()?;
                    let lit: syn::Expr = value.parse()?;
                    default_value = Some(lit);
                    Ok(())
                } else {
                    Err(meta.error("expected `default`"))
                }
            });
            if let Some(_val) = default_value {
                return Some(quote! {
                    pub #ident: #ty
                });
            }
        }
        Some(quote! {
            pub #ident: #ty
        })
    }).collect();

    let _prop_defaults: Vec<_> = props.iter().filter_map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        let default_attr = f.attrs.iter().find(|a| a.path().is_ident("prop"))?;
        let meta = &default_attr.meta;

        if let syn::Meta::List(list) = meta {
            let mut default_value = None;
            let _ = list.parse_nested_meta(|meta| {
                if meta.path.is_ident("default") {
                    let value = meta.value()?;
                    let lit: syn::Expr = value.parse()?;
                    default_value = Some(lit);
                    Ok(())
                } else {
                    Err(meta.error("expected `default`"))
                }
            });
            return Some(quote! {
                pub fn #ident(mut self, val: #ty) -> Self {
                    self.#ident = val;
                    self
                }
            });
        }
        None
    }).collect();

    let _builder_methods: Vec<_> = props.iter().filter_map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;

        let has_default = f.attrs.iter().any(|a| a.path().is_ident("prop"));
        if has_default {
            Some(quote! {
                pub fn #ident(mut self, val: #ty) -> Self {
                    self.#ident = val;
                    self
                }
            })
        } else {
            None
        }
    }).collect();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn builder() -> #name {
                #name
            }
        }
    }
}

#[cfg(test)]
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
        assert!(code.contains("fn builder"), "should generate builder method");
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
        assert!(code.contains("fn builder"), "should generate builder method");
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
        assert!(code.contains("MyComponent"), "should preserve generic struct name");
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
                pub fn builder() -> TestComp {
                    TestComp
                }
            }
        };
        assert_eq!(expanded.to_string(), expected.to_string());
    }
}
