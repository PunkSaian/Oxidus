#![feature(log_syntax)]
#![allow(clippy::cargo_common_metadata, clippy::missing_panics_doc)]
extern crate proc_macro;

use proc_macro::TokenStream;

mod detour_hook;
mod settings;
mod sig;
mod tf2_struct;
mod vmt;
mod vmt_hook;

#[proc_macro]
pub fn sig(input: TokenStream) -> TokenStream {
    sig::sig(input)
}

#[proc_macro_attribute]
pub fn vmt(attr: TokenStream, item: TokenStream) -> TokenStream {
    vmt::vmt(attr, item)
}

#[proc_macro_attribute]
pub fn tf2_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    tf2_struct::tf2_struct(attr, item)
}

#[proc_macro_attribute]
pub fn detour_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    detour_hook::detour_hook(attr, item)
}

#[proc_macro_attribute]
pub fn vmt_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    vmt_hook::vmt_hook(attr, item)
}

#[proc_macro]
pub fn settings(input: TokenStream) -> TokenStream {
    settings::settings(input)
}
