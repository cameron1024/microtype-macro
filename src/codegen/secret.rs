use super::HAS_SERDE;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type};

pub fn generate_secret(
    inner: Type,
    name: Ident,
    extra_attrs: Vec<Attribute>,
    serialize: bool,
) -> TokenStream {
    let wrapper = Ident::new(&format!("__Wrapper{}", name), name.span());

    let mut attrs = quote! {
        #[derive(::std::fmt::Debug, ::std::clone::Clone)]
        #[repr(transparent)]
    };

    if HAS_SERDE {
        attrs.extend(quote! {
            #[derive(::serde::Deserialize)]
        });

        if serialize {
            attrs.extend(quote! {
                #[derive(::serde::Serialize)]
            });
        }

        attrs.extend(quote! {
            #[serde(transparent)]
        });
    }

    let serialize_secret = if serialize && HAS_SERDE {
        Some(quote! { impl ::microtype::secrecy::SerializableSecret for #wrapper {} })
    } else {
        None
    };

    quote! {
        #(#extra_attrs)*
        #attrs
        pub struct #name(::microtype::secrecy::Secret<#wrapper>);

        impl ::microtype::SecretMicrotype for #name {
            type Inner = #inner;

            fn new(inner: Self::Inner) -> Self {
                Self(::microtype::secrecy::Secret::new(#wrapper(inner)))
            }
        }

        impl ::microtype::secrecy::ExposeSecret<#inner> for #name {
            fn expose_secret(&self) -> &#inner {
                use ::microtype::secrecy::ExposeSecret;
                &self.0.expose_secret().0
            }
        }



        #attrs
        struct #wrapper(#inner);

        impl ::microtype::secrecy::CloneableSecret for #wrapper {}
        impl ::microtype::secrecy::DebugSecret for #wrapper {}
        #serialize_secret

        impl ::microtype::secrecy::Zeroize for #wrapper {
            fn zeroize(&mut self) {
                use ::microtype::secrecy::Zeroize;
                self.0.zeroize()
            }
        }

    }
    .into()
}
