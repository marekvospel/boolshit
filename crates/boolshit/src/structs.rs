use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, Ident, LitBool, LitInt, Token};

pub(crate) struct ImplementTraitData {
    pub(crate) trait_name: Ident,
    pub(crate) fn_name: Ident,
    pub(crate) default: ImplementValue,
}

impl Parse for ImplementTraitData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let trait_name = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let fn_name = input.parse::<Ident>()?;

        let default = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            input.parse::<ImplementValue>()?
        } else {
            ImplementValue::Boolean(LitBool::new(false, input.span()))
        };

        Ok(Self {
            trait_name,
            fn_name,
            default,
        })
    }
}

#[derive(Clone)]
pub(crate) enum ImplementValue {
    Boolean(LitBool),
    Integer(LitInt),
}

#[derive(Clone)]
pub(crate) enum ImplementValueTransparentable {
    Any(ImplementValue),
    Transparent,
}

impl Parse for ImplementValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitBool) {
            Ok(ImplementValue::Boolean(input.parse::<LitBool>()?))
        } else if input.peek(LitInt) {
            Ok(ImplementValue::Integer(input.parse::<LitInt>()?))
        } else {
            panic!("AAA")
        }
    }
}

impl Parse for ImplementValueTransparentable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) {
            let ident = input.parse::<Ident>()?;
            if ident == "transparent" {
                Ok(ImplementValueTransparentable::Transparent)
            } else {
                // TODO: expression?
                panic!("Implement value not transparent")
            }
        } else {
            Ok(ImplementValueTransparentable::Any(ImplementValue::parse(
                input,
            )?))
        }
    }
}

impl ImplementValue {
    pub(crate) fn to_tokens(&self) -> TokenStream {
        match self {
            ImplementValue::Boolean(val) => {
                quote! {#val}
            }
            ImplementValue::Integer(val) => {
                quote! {#val}
            }
        }
    }

    pub(crate) fn retval(&self) -> TokenStream {
        match self {
            ImplementValue::Boolean(_) => quote!(bool),
            ImplementValue::Integer(_) => quote!(usize),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ImplementVariantData {
    pub(crate) fn_name: Ident,
    pub(crate) value: ImplementValueTransparentable,
}

impl Parse for ImplementVariantData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_name = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<ImplementValueTransparentable>()?;

        Ok(Self { fn_name, value })
    }
}
