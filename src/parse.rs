//! Parse the attribute macros into a valid invocation of the `fmt` macro
use ::devise::syn;

#[derive(Debug)]
pub enum Arg {
    Text(syn::LitStr),
    Ident(syn::Ident),
}
#[derive(Debug)]
pub struct Args {
    pub args: Vec<Arg>,
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
