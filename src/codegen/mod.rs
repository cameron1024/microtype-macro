use proc_macro::TokenStream;

use crate::{model::Microtype, parse::Secrecy};

mod normal;
mod secret;
mod diesel;

const HAS_SERDE: bool = cfg!(feature = "serde_support");
const HAS_TEST_IMPLS: bool = cfg!(feature = "test_impls");
const HAS_DIESEL_IMPLS: bool = cfg!(feature = "diesel_impls");

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

