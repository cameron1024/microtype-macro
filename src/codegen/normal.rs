use super::{
    diesel::generate_diesel_impls, special_attrs::SpecialAttrs, HAS_DIESEL_IMPLS, HAS_SERDE,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type};

pub fn generate_normal(
    inner: Type,
    name: Ident,
    attrs: Vec<Attribute>,
    _special_attrs: SpecialAttrs,
) -> TokenStream {
    let serde_attrs = if HAS_SERDE {
        Some(quote! {
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            #[serde(transparent)]
        })
    } else {
        None
    };

    let diesel_impls = if HAS_DIESEL_IMPLS {
        generate_diesel_impls(inner.clone(), name.clone(), &attrs, false)
    } else {
        quote! {}
    };

    quote! {
        #(#attrs)*
        #[repr(transparent)]
        #serde_attrs
        pub struct #name (#inner);

        impl ::microtype::Microtype for #name {
           type Inner = #inner;

           fn new(inner: Self::Inner) -> Self {
               Self(inner)
           }


           fn into_inner(self) -> Self::Inner {
               self.0
           }

           fn inner(&self) -> &Self::Inner {
               &self.0
           }

           fn inner_mut(&mut self) -> &mut Self::Inner {
               &mut self.0
           }


           fn transmute<T: ::microtype::Microtype<Inner = Self::Inner>>(self) -> T {
               T::new(self.0)
           }
        }

        impl ::std::convert::From<#inner> for #name {
            fn from(inner: #inner) -> Self {
                Self(inner)
            }
        }

        impl ::std::ops::Deref for #name {
            type Target = #inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #diesel_impls
    }
    .into()
}
