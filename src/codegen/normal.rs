use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Type};

pub fn generate_normal(inner: Type, name: Ident, attrs: Vec<Attribute>) -> TokenStream {
    quote! {
        #(#attrs)*
        #[repr(transparent)]
        pub struct #name (#inner);

        impl ::microtype_core::Microtype for #name {
           type Inner = String;

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


           fn transmute<T: ::microtype_core::Microtype<Inner = Self::Inner>>(self) -> T {
               T::new(self.0)
           }
        }

        impl From<#inner> for #name {
            fn from(inner: #inner) -> Self {
                Self(inner)
            }
        }
    }
    .into()
}
