use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, LitStr};
use quote::quote;


struct SignatureInput {
    pattern: LitStr,
}

impl Parse for SignatureInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pattern: input.parse()?,
        })
    }
}

pub fn sig(input: TokenStream) -> TokenStream {
    let sig_input = syn::parse_macro_input!(input as SignatureInput);
    let pattern_str = sig_input.pattern.value();

    let mut pattern = Vec::new();
    let mut mask = Vec::new();

    for part in pattern_str.split_whitespace() {
        match part {
            "?" | "??" => {
                pattern.push(0x0);
                mask.push(b'?');
            }
            _ => {
                let byte = u8::from_str_radix(part, 16)
                    .unwrap_or_else(|_| panic!("Invalid hex byte: {part}"));
                pattern.push(byte);
                mask.push(b'x');
            }
        }
    }

    TokenStream::from(quote! {
        {
            crate::util::signature_scanner::Signature::new(
                vec![#(#pattern),*],
                vec![#(#mask),*]
            )
        }
    })
}
