use syn::{Attribute, Ident};

pub fn strip_special_attrs(attrs: Vec<Attribute>) -> (Vec<Attribute>, SpecialAttrs) {
    todo!()
}

pub struct SpecialAttrs {
    pub secret: Option<SecretAttr>,
}

pub struct SecretAttr {
    pub serialize: bool,
    pub token: Ident,
}
