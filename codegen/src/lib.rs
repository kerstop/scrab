#![feature(proc_macro_diagnostic)]
use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, ExprUnary, LitInt, Token};

struct CordinateMacro {
    q: i32,
    r: i32,
    s: i32,
    span: Span,
}

impl Parse for CordinateMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let q: i32 = input.parse::<LitInt>()?.base10_parse().unwrap();
        input.parse::<Token![,]>().unwrap();
        let r: i32 = input.parse::<LitInt>()?.base10_parse().unwrap();
        input.parse::<Token![,]>().unwrap();
        let s: i32 = input.parse::<LitInt>()?.base10_parse().unwrap();
        let span = input.span();
        Ok(Self { q, r, s, span })
    }
}

/// Create a `Cordinate` at compile time
///
/// This macro should only be used to make Cordinates using integer literals and should be prefered over the `new()`
/// function as it check to make sure the cordinate is valid at compile time.

#[proc_macro]
pub fn cord(input: TokenStream) -> TokenStream {
    let CordinateMacro { q, r, s, span } = parse_macro_input!(input as CordinateMacro);

    if q + r + s != 0 {
        span.span()
            .unwrap()
            .error("The components must add up to 0")
            .emit();
        return quote!(()).into();
    }

    quote! {
        scrab::hex_grid::Cordinate::new(#q, #r, #s).unwrap()
    }
    .into()
}
