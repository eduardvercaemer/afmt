use crate::parse::{Fmt, Section};
use ::devise::proc_macro2::TokenStream as TokenStream2;
use ::devise::quote::quote;
use ::devise::syn;

/// Codegen for a matching a string literal
///
/// The code generated makes sure the string we are parsing matches the given
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
fn str_match(lit: &syn::LitStr) -> TokenStream2 {
    quote! {
        if !s.starts_with(#lit) {
            return Err(format!("string literal match for \"{}\" failed", #lit));
        }

        let (_, s) = s.split_at(#lit .as_bytes().len());
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
fn capture(field: &syn::Ident) -> TokenStream2 {
    quote! {
        let #field = match s.parse() {
            Ok(v) => v,
            Err(e) => return Err(format!("{}", e)),
        };
    }
}

/// Codegen for field member capture with string literal lookahead
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
fn capture_w_lookahead(field: &syn::Ident, lookahead: &syn::LitStr) -> TokenStream2 {
    quote! {
        let idx = match s.find(#lookahead) {
            Some(idx) => idx,
            None => return Err(format!("lookahead for \"{}\" not matched", #lookahead)),
        };
        let (left, s) = s.split_at(idx);
        let (_, s) = s.split_at(#lookahead .as_bytes().len());
        let #field = match left.parse() {
            Ok(value) => value,
            Err(e) => return Err(format!("{}", e)),
        };
    }
}

/// Codegen for a full chain of arguments
pub fn codegen(fmt: Fmt) -> TokenStream2 {
    let mut code = vec![];
    for s in fmt.sections.iter() {
        code.push(match s {
            Section::StrMatch(s) => str_match(&s.lit),
            Section::CaptureLookahead(s) => capture_w_lookahead(&s.field, &s.lookahead),
            Section::Capture(s) => capture(&s.field),
        });
    }

    quote! {
        #(#code)*
    }
}

/// Codegen for building a struct with default field names from a struct item
///
/// Example --
///
/// CODEGEN ARGS:
///   s = "struct Foo { v: u32, f: f64 }"
/// CODE OUT:
///   Foo { v, f, }
pub fn struct_builder(s: &syn::ItemStruct) -> TokenStream2 {
    let ident = &s.ident;
    let mut fields = vec![];
    for field in s.fields.iter() {
        let field = field.ident.clone();
        fields.push(field.unwrap());
    }
    quote! {
        #ident {
            #(#fields),*
        }
    }
}
