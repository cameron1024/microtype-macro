use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type};
use super::HAS_SERDE;

pub fn generate_normal(inner: Type, name: Ident, attrs: Vec<Attribute>) -> TokenStream {
    let serde_attrs = if HAS_SERDE {
        Some(quote! {
            #[derive(::serde::Deserialize, ::serde::Serialize)]
            #[serde(transparent)]
        })
    } else {
        None
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

        impl ::std::convert::From<#inner> for #name {
            fn from (inner: #inner) -> #name {
                <#name as ::microtype::Microtype>::new(inner)
            }
        }
    }
    .into()
}
