#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

use proc_macro2::{Ident, Span, TokenStream, TokenTree};

fn fn_name(item: TokenStream) -> Ident {
    let mut tokens = item.into_iter();
    let mut fn_found = false;
    while let Some(tok) = tokens.next() {
        if let TokenTree::Ident(word) = tok {
            if word == "fn" {
                fn_found = true;
                break;
            }
        }
    }

    match tokens.next() {
        Some(TokenTree::Ident(word)) => word,
        _ => if fn_found {
            panic!("failed to find function name")
        } else {
            panic!("the macro attribute applies only to functions")
        },
    }
}

#[proc_macro_attribute]
pub fn interpolate_name(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(attr).into_iter().collect::<Vec<_>>();
    if tokens.len() != 1 {
        panic!("expected #[interpolate_name(specifier)]");
    }

    let specifier = match &tokens[0] {
        TokenTree::Ident(tt) => tt.to_string(),
        _ => panic!("expected #[interpolate_name(specifier)]"),
    };

    let item = TokenStream::from(item);
    let name = fn_name(item.clone());
    let interpolated_name = Ident::new(&format!("{}_{}", name.to_string(), specifier), Span::call_site());

    let ret: TokenStream = quote_spanned! {
        proc_macro2::Span::call_site() =>
        fn #interpolated_name() {
            #item

            return #name ();
        }
    }.into();

    ret.into()
}

use std::iter::FromIterator;

#[proc_macro_attribute]
pub fn interpolate_test(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let tokens = TokenStream::from(attr).into_iter().collect::<Vec<_>>();
    if tokens.len() < 3  {
        panic!("expected #[interpolate_test(specifier, arguments)]");
    }

    let specifier = match &tokens[0] {
        TokenTree::Ident(tt) => tt.to_string(),
        _ => panic!("expected #[interpolate_name(specifier)]"),
    };

    match &tokens[1] {
        TokenTree::Punct(p) if p.as_char() == ',' => {}
        _ => panic!("expected #[interpolate_test(specifier, arguments)]")
    }

    let args = TokenStream::from_iter(tokens.into_iter().skip(2));

    let item = TokenStream::from(item);
    let name = fn_name(item.clone());
    let interpolated_name = Ident::new(&format!("{}_{}", name.to_string(), specifier), Span::call_site());

    let ret: TokenStream = quote_spanned! {
        proc_macro2::Span::call_site() =>
        #item

        #[test]
        fn #interpolated_name() {
            return #name (#args);
        }
    }.into();

    ret.into()
}
