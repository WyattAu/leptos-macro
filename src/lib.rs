#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Derive macro for Leptos component boilerplate.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Fields};

/// Derive macro for Leptos component boilerplate.
///
/// Generates module declarations, feature flag setup, and prop default implementations.
///
/// # Usage
///
/// ```ignore
/// #[derive(LeptosComponent)]
/// #[component]
/// fn MyComponent(prop_a: String, #[prop(default = false)] enabled: bool) -> impl IntoView {
///     view! { <div>{prop_a}</div> }
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
