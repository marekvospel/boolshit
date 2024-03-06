#![feature(proc_macro_diagnostic)]

use std::collections::HashMap;

use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use structs::{ImplementTraitData, ImplementVariantData};
use syn::{parse_macro_input, spanned::Spanned, AttrStyle, Data, DeriveInput, Meta, Variant};

mod structs;

struct ImplementTrait {
    data: ImplementTraitData,
    span: Span,
    overrides: HashMap<String, ImplementVariant>,
}

struct ImplementVariant {
    data: ImplementVariantData,
    variant: Variant,
}

#[proc_macro_derive(Boolshit, attributes(boolshit))]
pub fn boolshit_derive(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    // fn name to
    let mut traits = HashMap::<String, ImplementTrait>::new();

    for attr in input.attrs.clone() {
        if matches!(attr.style, AttrStyle::Outer)
            && attr.path().is_ident(&format_ident!("boolshit"))
        {
            if let Meta::List(list) = attr.meta.clone() {
                let tokens = list.tokens.into();
                let gen = parse_macro_input!(tokens as ImplementTraitData);
                if let Some(old) = traits.get(&gen.fn_name.to_string()) {
                    attr.span()
                        .unwrap()
                        .error(format!(
                            "The name `{}` is being implemented multiple times.\nBoolshit can only implement each function once.",
                            old.data.fn_name
                        ))
                        .span_note(
                            old.span.unwrap(),
                            format!("Function {} is already being implemented here", old.data.fn_name),
                        )
                        .emit();
                }
                let gen = ImplementTrait {
                    data: gen,
                    overrides: HashMap::new(),
                    span: attr.span(),
                };
                traits.insert(gen.data.fn_name.to_string(), gen);
            } else {
                attr.span()
                    .unwrap()
                    .error("Expected boolshit attribute to define trait name and a function name")
                    .help("Usage: #[boolshit(TraitName, fn_name)]")
                    .emit();
            }
        }
    }

    if let Data::Enum(en) = input.data {
        for variant in en.variants {
            for attr in variant.attrs.clone() {
                if matches!(attr.style, AttrStyle::Outer)
                    && attr.path().is_ident(&format_ident!("boolshit"))
                {
                    if let Meta::List(list) = attr.meta.clone() {
                        let tokens = list.tokens.into();
                        let var = parse_macro_input!(tokens as ImplementVariantData);

                        if let Some(data) = traits.get_mut(&var.fn_name.to_string()) {
                            data.overrides.insert(
                                variant.ident.to_string(),
                                ImplementVariant {
                                    data: var,
                                    variant: variant.clone(),
                                },
                            );
                        } else {
                            attr.span().unwrap().error("BIG SAD").emit()
                        }
                    } else {
                        panic!("Boolshit attribute requires at least trait name and function name");
                    }
                }
            }
        }
    }

    let ident = input.ident;
    let mut out = quote! {};

    for (_, tr) in traits {
        let trait_name = tr.data.trait_name;
        let fn_name = tr.data.fn_name;
        let val = tr.data.default.to_tokens();
        let retval = tr.data.default.retval();
        let mut options = quote! {};

        tr.overrides.iter().for_each(|(_, v)| {
            let variant = &v.variant.ident;
            let fields = match &v.variant.fields {
                syn::Fields::Named(_) => todo!(),
                syn::Fields::Unnamed(fields) => {
                    let fields = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        if i == 0 {
                            quote!(first)
                        } else {
                            quote!(_)
                        }
                    });
                    quote!((
                        #( #fields ),*
                    ))
                }
                syn::Fields::Unit => quote!(),
            };
            let res = match &v.data.value {
                structs::ImplementValueTransparentable::Any(v) => v.to_tokens(),
                structs::ImplementValueTransparentable::Transparent => quote! {
                    #trait_name::#fn_name(first)
                },
            };
            options = quote_spanned!(tr.span =>
                #options
                #ident::#variant #fields => #res,
            )
        });

        out = quote! {
            #out

            impl #trait_name for #ident {
                fn #fn_name(&self) -> #retval {
                    match self {
                        #options
                        _ => #val
                    }
                }
            }
        };
    }

    out.into()
}
