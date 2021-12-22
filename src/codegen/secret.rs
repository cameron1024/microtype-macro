use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type};

use super::{serde_deserialize, serde_serialize, serde_transparent};


pub fn generate_secret(
    inner: Type,
    name: Ident,
    attrs: Vec<Attribute>,
    serialize: bool,
) -> TokenStream {
    let wrapper = Ident::new(&format!("__Wrapper{}", name), name.span());

    eprintln!("{}", cfg!(feature = "serde_support"));
    let serialize_attr = if serialize { serde_serialize() } else { None };

    let deserialize_attr = serde_deserialize();
    let transparent = serde_transparent();

    let default_attrs = quote::quote! {
        #[derive(::std::fmt::Debug, ::std::clone::Clone)]
        #[repr(transparent)]
        #serialize_attr
        #deserialize_attr
        #transparent
    };

    let serializable_secret = if serialize {
        Some(quote! {
            impl ::microtype_core::secrecy::SerializableSecret for #wrapper {}
        })
    } else {
        None
    };

    quote! {
        #(#attrs)*
        #default_attrs
        pub struct #name(::microtype_core::secrecy::Secret<#wrapper>);

        #default_attrs
        struct #wrapper(#inner);

        impl ::microtype_core::secrecy::CloneableSecret for #wrapper {}
        impl ::microtype_core::secrecy::DebugSecret for #wrapper {}
        #serializable_secret

        impl ::microtype_core::secrecy::Zeroize for #wrapper {
            fn zeroize(&mut self) {
                self.0.zeroize()
            }
        }

        impl ::microtype_core::secrecy::ExposeSecret<#inner> for #name {
            fn expose_secret(&self) -> &#inner {
                use ::microtype_core::secrecy::ExposeSecret;
                &self.0.expose_secret().0
            }
        }

        impl ::microtype_core::SecretMicrotype for #name {
            type Inner = #inner;

            fn new(inner: Self::Inner) -> Self {
                Self(::microtype_core::secrecy::Secret::new(#wrapper(inner)))
            }
        }

    }
    .into()
}
