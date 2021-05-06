extern crate proc_macro;
extern crate devise;
use ::proc_macro::{TokenStream};
use ::devise::quote::{quote};
use ::devise::proc_macro2::{TokenStream as TokenStream2};
use ::devise::syn;

mod parse;
use parse::{Args, Arg};

#[proc_macro_attribute]
pub fn fmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let args = syn::parse_macro_input!(attr as Args);

    /* gather everything we need */
    let s = &input;                         // struct item
    let s_ident = &input.ident;                 // struct ident
    let s_fields = struct_field_idents(s);   // struct field idents
    let s_parse = argument_parsing(args);   // code that parses the struct

    let code = quote! {
        #s

        impl ::std::str::FromStr for #s_ident {
            type Err = String;
            fn from_str(s: &str) -> Result<#s_ident, String> {
                #s_parse

                Ok(#s_ident {
                    #(#s_fields),*
                })
            }
        }
    };

    code.into()
}

/// From a struct item, get the vector of field idents
///  in: struct { v: u32, f: f32 }
///  out: [v, f]
fn struct_field_idents(s: &syn::ItemStruct) -> Vec<syn::Ident> {
    let mut build = vec![];
    for field in s.fields.iter() {
        let i = field.ident.clone();
        build.push(i.unwrap());
    }
    build
}

/// From a vector of `fmt` arguments, generate the code that
/// correctly parses a string in scope `s` into the appropiate fields
fn argument_parsing(args: Args) -> TokenStream2 {
    let mut parse = vec![];
    let mut args = args.args.into_iter();

    while let Some(arg) = args.next() {
        match arg {
            Arg::Text(lit) => {
                parse.push(parse_lit_str(lit));
            }

            Arg::Ident(ident) => {
                if let Some(next) = args.next() {
                    // capture with lookahead
                    let lookahead = match next {
                        Arg::Ident(_) => panic!("bad format string"),
                        Arg::Text(lit) => lit,
                    };
                    parse.push(parse_capture_w_lookahead(ident, lookahead));
                } else {
                    // capture w/o lookahead
                    parse.push(parse_capture(ident));
                }
            }
        };
    }

    quote! {
        #(#parse)*
    }    
}

/// Codegen for a literal string argument
///
/// The code generated makes sure the string we are parsing matches the
/// string literal, in which case it consumes it and continues, otherwise
/// it returns a parse error.
///
/// Example --
///
/// CODEGEN ARGS:
///   lit_str = "abc"
/// SCOPE IN:
///   s = "abcxyz"
/// SCOPE OUT:
///   s = "xyz"
fn parse_lit_str(lit_str: syn::LitStr) -> TokenStream2 {
    quote! {
        if !s.starts_with(#lit_str) {
            return Err("parse_lit_str".to_owned());
        }
        
        let (_, s) = s.split_at(#lit_str .as_bytes().len());
    }
}

/// Codegen for field member capture
///
/// The code generated attempts to parse a field value from the whole remainings
/// of the string in scope.
///
/// Example --
///
/// CODEGEN ARGS:
///   capture = 'v' (an integer)
/// SCOPE IN:
///   s = "65"
/// SCOPE OUT:
///   s = "65"
///   v = 65
fn parse_capture(capture: syn::Ident) -> TokenStream2 {
    quote! {
        let #capture = match s.parse() {
            Ok(v) => v,
            Err(e) => return Err(format!("{}", e)),
        };
    }
}

/// Codegen for field member capture with lookahead
///
/// The code generated makes sure a certain string literal lookahead is valid,
/// then captures the string before it into a capture field.
///
/// Example --
///
/// CODEGEN ARGS:
///   capture = 'v' (an integer)
///   lookahead = "abc"
/// SCOPE IN:
///   s = "65abcxyz"
/// SCOPE OUT:
///   s = "xyz"
///   v = 65
fn parse_capture_w_lookahead(capture: syn::Ident, lookahead: syn::LitStr) -> TokenStream2 {
    quote! {
        let idx = match s.find(#lookahead) {
            Some(idx) => idx,
            None => return Err(format!("lookahead for \"{}\" not matched", #lookahead)),
        };
        let (left, s) = s.split_at(idx);
        let (_, s) = s.split_at(#lookahead .as_bytes().len());
        let #capture = match left.parse() {
            Ok(value) => value,
            Err(e) => return Err(format!("{}", e)),
        };
    }
}