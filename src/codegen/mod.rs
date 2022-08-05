use proc_macro::TokenStream;
use quote::quote_spanned;

use crate::model::Microtype;

use self::special_attrs::{strip_special_attrs, SecretAttr};

mod diesel;
mod normal;
mod secret;

mod special_attrs;

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

fn generate_single(Microtype { inner, name, attrs }: Microtype) -> TokenStream {
    let (attrs, special_attrs) = strip_special_attrs(attrs);

    let needs_serialize_impl = matches!(
        special_attrs.secret,
        Some(SecretAttr {
            serialize: true,
            ..
        })
    );

    if !HAS_SERDE {
        if let Some(SecretAttr {
            serialize: true,
            token,
        }) = special_attrs.secret
        {
            return quote_spanned!(token.span());
        }
    }

    match &special_attrs.secret {
        None => normal::generate_normal(inner, name, attrs, special_attrs),
        Some(SecretAttr { serialize }) => {
            secret::generate_secret(inner, name, attrs, *serialize, special_attrs)
        }
    }
}
