//! This crate provides the `fmt` attribute macro, its usage is simple:
//!
//! ```
//! #[macro_use] extern crate afmt;
//!
//! #[fmt("value: " v)]
//! struct Foo {
//!     v: u32,   
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let f: Foo = "value: 65".parse()?;
//!     assert_eq!(f.v, 65);
//!
//!     let f: Result<Foo,_> = "val: 65".parse();
//!     assert!(f.is_err());
//!     Ok(())
//! }
//! ```

extern crate devise;
extern crate proc_macro;
use ::devise::quote::quote;
use ::devise::syn;
use ::proc_macro::TokenStream;

mod parse;
mod codegen;

#[proc_macro_attribute]
pub fn fmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let fmt = syn::parse_macro_input!(attr as parse::Fmt);

    /* gather everything we need */
    let s = &input;
    let s_ident = &input.ident;
    let s_build = codegen::struct_builder(s);
    let s_parse = codegen::codegen(fmt);

    let code = quote! {
        #s

        impl ::std::str::FromStr for #s_ident {
            type Err = String;
            fn from_str(s: &str) -> Result<#s_ident, String> {
                #s_parse

                Ok(#s_build)
            }
        }
    };

    code.into()
}
