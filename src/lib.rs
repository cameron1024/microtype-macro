#![warn(clippy::all)]

use codegen::codegen;
use parse::MicrotypeMacro;
use syn::parse_macro_input;

use crate::model::flatten;

extern crate proc_macro;

mod parse;
mod model;
mod codegen;


#[proc_macro]
pub fn microtype(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let microtype = parse_macro_input!(tokens as MicrotypeMacro);
    let microtypes = flatten(microtype);
    codegen(microtypes)
}




#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui/fail/*.rs");
        t.pass("tests/ui/pass/*.rs");
    }

}




