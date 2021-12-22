use proc_macro::TokenStream;
use quote::quote;

use crate::{model::Microtype, parse::Secrecy};

mod normal;
mod secret;

pub fn codegen(microtypes: Vec<Microtype>) -> TokenStream {
    let mut stream = TokenStream::new();

    for microtype in microtypes {
        let tokens = generate_single(microtype);
        stream.extend(tokens);
    }

    stream
}

fn generate_single(
    Microtype {
        inner,
        name,
        secrecy,
        attrs,
    }: Microtype,
) -> TokenStream {
    match secrecy {
        Secrecy::Normal => normal::generate_normal(inner, name, attrs),
        Secrecy::Secret => secret::generate_secret(inner, name, attrs, false),
        Secrecy::OutSecret => secret::generate_secret(inner, name, attrs, true),
    }
}

fn serde_transparent() -> Option<proc_macro2::TokenStream> {
    let serde = cfg!(feature = "serde_support");
    if serde {
        Some(quote! {#[serde(transparent)]})
    } else {
        None
    }
}

fn serde_serialize() -> Option<proc_macro2::TokenStream> {
    if cfg!(feature = "serde_support") {
        Some(quote! { #[derive(::serde::Serialize)] })
    } else {
        None
    }
}

fn serde_deserialize() -> Option<proc_macro2::TokenStream> {
    if cfg!(feature = "serde_support") {
        Some(quote! { #[derive(::serde::Deserialize)] })
    } else {
        None
    }
}



#[cfg(test)]
mod tests {}
