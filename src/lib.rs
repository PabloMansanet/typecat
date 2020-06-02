extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn concatenate(_item: TokenStream) -> TokenStream {
    let text: String = _item
        .into_iter()
        .filter_map(|t| if let TokenTree::Ident(i) = t { Some(i.to_string()) } else { None })
        .collect();
    text.parse().unwrap()
}

#[proc_macro]
pub fn concatenate_upper(_item: TokenStream) -> TokenStream {
    let text: String = _item
        .into_iter()
        .filter_map(|t| if let TokenTree::Ident(i) = t { Some(i.to_string()) } else { None })
        .collect::<String>()
        .to_uppercase();
    text.parse().unwrap()
}

#[proc_macro]
pub fn concatenate_lower(_item: TokenStream) -> TokenStream {
    let text: String = _item
        .into_iter()
        .filter_map(|t| if let TokenTree::Ident(i) = t { Some(i.to_string()) } else { None })
        .collect::<String>()
        .to_lowercase();
    text.parse().unwrap()
}
