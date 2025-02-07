extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn};

/// # Panics
/// `self` argument is not allowed
#[proc_macro_attribute]
pub fn detour_hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let sig = &input_fn.sig;
    let unsafety = &sig.unsafety;
    let abi = &sig.abi;
    let output = &sig.output;

    let param_types: Vec<_> = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => panic!("Receiver not allowed in hook function"),
            FnArg::Typed(pat_type) => &pat_type.ty,
        })
        .collect();

    let gateway_type = quote! {
        #unsafety #abi fn(#(#param_types),*) #output
    };

    let before = quote! {
        let mut hook: Box<DetourHook> = {
            let res: *mut DetourHook;
            unsafe {
                ::std::arch::asm!("mov {}, r10", out(reg) res);
            }
                Box::from_raw(res)
        };

        let original_function  = std::mem::transmute::<_,#gateway_type>(hook.target_fn);

        hook.restore();
    };

    let after = quote! {
        hook.install().unwrap();
        std::mem::forget(hook);
    };

    let original_block = &input_fn.block;
    input_fn.block = syn::parse2(quote! {
        {
            #before
            let res = {
                #original_block
            };
            #after
            res
        }
    })
    .expect("Failed to parse modified block");

    TokenStream::from(quote! { #input_fn })
}
