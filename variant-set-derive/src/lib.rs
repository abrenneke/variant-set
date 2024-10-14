#![warn(clippy::all, clippy::pedantic)]

use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

/// Derives a `_Variant` enum for the given enum, and derives the `VariantEnum` trait.
///
/// The `VariantEnum` trait is used to convert an enum into a variant enum, which is an enum that has a variant for
/// each variant of the input enum, but without any data. This is used for the
/// `VariantSet<T>` type, which is a set of variants of type T.
///
/// # Panics
///
/// Panics if the input is not an enum.
#[proc_macro_derive(VariantEnum)]
pub fn derive_variant_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants_enum_name = format_ident!("{}Variant", &input.ident);

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => panic!("VariantEnum can only be derived for enums"),
    };

    let variant_idents: Vec<_> = variants.iter().map(|variant| &variant.ident).collect();

    let enum_variants = variant_idents.iter().map(|variant| {
        quote! {
            #variant
        }
    });

    let variant_cases = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Unit => {
                quote! {
                    #name::#variant_name => #variants_enum_name::#variant_name,
                }
            }
            syn::Fields::Named(_) => {
                quote! {
                    #name::#variant_name { .. } => #variants_enum_name::#variant_name,
                }
            }
            syn::Fields::Unnamed(_) => {
                quote! {
                    #name::#variant_name(..) => #variants_enum_name::#variant_name,
                }
            }
        }
    });

    let expanded = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #variants_enum_name {
            #(#enum_variants),*
        }

        impl From<#name> for #variants_enum_name {
            fn from(value: #name) -> Self {
                <#name as variant_set::VariantEnum>::variant(&value)
            }
        }

        impl variant_set::VariantEnum for #name {
            type Variant = #variants_enum_name;

            fn variant(&self) -> Self::Variant {
                match self {
                    #(#variant_cases)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
