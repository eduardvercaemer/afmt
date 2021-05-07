//! Parse the attribute macros into a valid invocation of the `fmt` macro
use ::devise::syn;

/// Match a string literal section of the string
/// e.g.
/// fmt("hello" v) would make sure the string begins with "hello"
#[derive(Debug)]
pub struct StrMatch {
    pub lit: syn::LitStr,
}

/// Capture a field member by making sure we can match a string literal
/// lookahead
/// e.g
/// fmt(v "xyz") would find where in the string the pattern "xyz" appears
/// and capture the previous section into `v`
#[derive(Debug)]
pub struct CaptureLookahead {
    pub field: syn::Ident,
    pub lookahead: syn::LitStr,
}

/// Capture a field member from the string we are parsing
/// e.g.
/// fmt(v) would consume the whole string into the field `v`
#[derive(Debug)]
pub struct Capture {
    pub field: syn::Ident,
}

/// Each of the variants represent a part of the `fmt` attribute, which will
/// be used to inndependently generate the code to parse tis respective part
/// of the string.
#[derive(Debug)]
pub enum Section {
    StrMatch(StrMatch),
    CaptureLookahead(CaptureLookahead),
    Capture(Capture),
}

/// A complete chain of `fmt` sections defining the fmt attribute
#[derive(Debug)]
pub struct Fmt {
    pub sections: Vec<Section>,
}

impl syn::parse::Parse for Fmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sections = vec![];
        
        loop {
            if input.is_empty() {
                break;
            }

            let next = input.lookahead1();

            if next.peek(syn::Lit) {
                match input.parse().unwrap() {
                    syn::Lit::Str(lit) => {
                        let section = StrMatch { lit };
                        sections.push(Section::StrMatch(section));
                    }
                    _ => return Err(next.error()),
                }
            } else if next.peek(syn::Ident) {
                let field: syn::Ident = input.parse().unwrap();
                if input.is_empty() {
                    let section = Capture { field };
                    sections.push(Section::Capture(section));
                    continue;
                }

                let lookahead = input.lookahead1();
                if lookahead.peek(syn::Lit) {
                    match input.parse().unwrap() {
                        syn::Lit::Str(lit) => {
                            let section = CaptureLookahead {
                                field,
                                lookahead: lit,
                            };
                            sections.push(Section::CaptureLookahead(section));
                        }
                        _ => return Err(lookahead.error()),
                    }
                } else {
                    return Err(lookahead.error());
                }
            } else {
                return Err(next.error());
            }
        }

        Ok(Self { sections })
    }
}
