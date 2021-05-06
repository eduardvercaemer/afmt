extern crate proc_macro;
extern crate devise;
use ::proc_macro::{TokenStream};
use ::devise::quote::{quote};
use ::devise::syn;
#[derive(Debug)]
enum Arg {
    Text(syn::LitStr),
    Ident(syn::Ident),
}

#[derive(Debug)]
struct Args {
    args: Vec<Arg>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ret = Args {
            args: vec![],
        };
        
        let mut prev = false;
        loop {
            if input.is_empty() {
                return Ok(ret);
            }

            let lookahead = input.lookahead1();

            if lookahead.peek(syn::Lit) { // literal
                let lit: syn::Lit = input.parse().unwrap();
                match lit {
                    syn::Lit::Str(s) => {
                        ret.args.push(Arg::Text(s));
                    }
                    _ => return Err(lookahead.error()),
                }
                prev = false;
            } else if lookahead.peek(syn::Ident) {
                if prev {
                    // cannot have two idents next to each other
                    return Err(lookahead.error());
                }
                let ident: syn::Ident = input.parse().unwrap();
                ret.args.push(Arg::Ident(ident));
                prev = true;
            } else {
                return Err(lookahead.error());
            }
        }
    }
}

#[proc_macro_attribute]
pub fn fmt(attr: TokenStream, item: TokenStream) -> TokenStream {    
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let args = syn::parse_macro_input!(attr as Args);

    let s = &input;
    let s_ident = &input.ident;

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

    let mut build = vec![];
    for field in s.fields.iter() {
        let i = field.ident.clone();
        build.push(i.unwrap());
    }

    let tokens = quote! {
        #s

        impl ::std::str::FromStr for #s_ident {
            type Err = ();
            fn from_str(s: &str) -> Result<#s_ident, <#s_ident as ::std::str::FromStr>::Err> {
                #(#parse)*

                Ok(#s_ident {
                    #(#build),*
                })
            }
        }
    };

    tokens.into()
}