use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Ident, Result, Token, Type,
};

mod kw {
    syn::custom_keyword!(secret);
    syn::custom_keyword!(out);
}

/// The entire invocation of the macro
pub struct MicrotypeMacro(pub Vec<MicrotypeDecl>);

impl Parse for MicrotypeMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut result = vec![];
        while !input.is_empty() {
            result.push(input.parse()?);
        }
        Ok(Self(result))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Secrecy {
    Normal,
    OutSecret,
    Secret,
}

impl Parse for Secrecy {
    fn parse(input: ParseStream) -> Result<Self> {
        let out: Option<kw::out> = input.parse()?;
        let secret: Option<kw::secret> = input.parse()?;
        Ok(match (out, secret) {
            (Some(_), Some(_)) => Secrecy::OutSecret,
            (None, Some(_)) => Secrecy::Secret,
            (None, None) => Secrecy::Normal,
            (Some(out), None) => {
                return Err(Error::new(
                    out.span(),
                    "Cannot be `out` without also being `secret`",
                ))
            }
        })
    }
}

/// A one-to-many mapping of inner type to any number of microtypes
pub struct MicrotypeDecl {
    pub attrs: Vec<Attribute>,
    pub secrecy: Secrecy,
    pub inner: Type,
    pub idents: Vec<AttrIdent>,
}

impl Parse for MicrotypeDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let secrecy: Secrecy = input.parse()?;
        let inner: Type = input.parse()?;
        let content;
        let _ = braced!(content in input);
        let idents = Punctuated::<AttrIdent, Token![,]>::parse_terminated(&content)?;
        let idents = idents.into_iter().collect();

        Ok(Self {
            attrs,
            secrecy,
            inner,
            idents,
        })
    }
}

/// Identifier with 0 or more attributes
pub struct AttrIdent {
    pub attributes: Vec<Attribute>,
    pub ident: Ident,
}

impl Parse for AttrIdent {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let ident = input.parse()?;
        Ok(Self { attributes, ident })
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::*;

    #[test]
    fn parse_attr_ident() {
        let attr_ident: AttrIdent = parse_str("#[foo] asdf").unwrap();
        assert_eq!(attr_ident.attributes.len(), 1);
        assert_eq!(attr_ident.ident.to_string(), "asdf");
    }

    #[test]
    fn parse_secrecy() {
        let secret: Secrecy = parse_str("secret").unwrap();
        assert_eq!(secret, Secrecy::Secret);

        let out_secret: Secrecy = parse_str("out secret").unwrap();
        assert_eq!(out_secret, Secrecy::OutSecret);

        let normal: Secrecy = parse_str("").unwrap();
        assert_eq!(normal, Secrecy::Normal);

        let err: Result<Secrecy> = parse_str("out");
        assert!(err.is_err());
    }

    #[test]
    fn parse_microtype_decl() {
        let microtype_decl: MicrotypeDecl =
            parse_str("out secret String { #[foo] Email }").unwrap();
        assert!(microtype_decl.attrs.is_empty());
        assert_eq!(microtype_decl.secrecy, Secrecy::OutSecret);
        assert_eq!(microtype_decl.idents[0].attributes.len(), 1);
        assert_eq!(microtype_decl.idents[0].ident.to_string(), "Email");
    }

    #[test]
    fn parse_full_macro() {
        let microtype: MicrotypeMacro = parse_str(
            r#"
#[foo]
out secret String {
    Email
}
i64 {
    Age
}
"#,
        )
        .unwrap();

        assert_eq!(microtype.0.len(), 2);
        let first = &microtype.0[0];
        assert_eq!(first.secrecy, Secrecy::OutSecret);
        assert_eq!(first.attrs.len(), 1);
        let ty = &first.inner;
        let ty = quote::quote! {#ty};
        assert_eq!(ty.to_string(), "String");
    }
}
