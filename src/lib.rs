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

    let tokens = quote! {
        #s

        impl ::std::str::FromStr for #s_ident {
            type Err = ();
            fn from_str(s: &str) -> Result<#s_ident, <#s_ident as ::std::str::FromStr>::Err> {
                #s_parse

                Ok(#s_ident {
                    #(#s_fields),*
                })
            }
        }
    };

    tokens.into()
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
            // consume text
            Arg::Text(lit) => {
                let code = quote! {
                    if !s.starts_with(#lit) {
                        return Err(());
                    }
                    let (_,s) = s.split_at(#lit .as_bytes().len());
                };
                parse.push(code);
            }

            // capture value
            Arg::Ident(ident) => {
                let code = if let Some(next) = args.next() {
                    // more elements
                    let lit = match next {
                        Arg::Ident(_) => panic!("bad arg"),
                        Arg::Text(lit) => lit,
                    };
                    quote! {
                        let idx = match s.find(#lit) {
                            Some(idx) => idx,
                            _ => return Err(()),
                        };
                        let (left, s) = s.split_at(idx);
                        let (_, s) = s.split_at(#lit .as_bytes().len());
                        let #ident = match left.parse() {
                            Ok(value) => value,
                            _ => return Err(()),
                        };
                    }
                } else {
                    // final element
                    quote! {
                        let #ident = match s.parse() {
                            Ok(value) => value,
                            _ => return Err(()),
                        };
                    }
                };
                parse.push(code);
            }
        };
    }

    quote! {
        #(#parse)*
    }    
}