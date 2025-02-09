#![allow(clippy::cargo_common_metadata)]
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

    let param_vals: Vec<_> = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => panic!("Receiver not allowed in hook function"),
            FnArg::Typed(pat_type) => &pat_type.pat,
        })
        .collect();

    let gateway_type = quote! {
        #unsafety #abi fn(#(#param_types),*) #output
    };

    let before = quote! {
        let hook_ptr = {
            let res: usize;
            unsafe {
                ::std::arch::asm!("mov {}, r10", out(reg) res);
            }
            res as *const std::sync::RwLock<crate::hook::detour::DetourHook>
        };
        let hook_lock = unsafe { &*hook_ptr };
        let mut hook = hook_lock.write().unwrap();

        let original_function = std::mem::transmute::<_, #gateway_type>(hook.target_fn);

        if let Err(e) = hook.restore() {
            return original_function(#(#param_vals),*);
        }
    };

    let after = quote! {

        (*hook).install().unwrap();
    };

    let original_block = &input_fn.block;
    input_fn.block = syn::parse2(quote! {
        {
            #before
            let res = (|| #output  {
                #original_block
            })();
            #after
            res
        }
    })
    .expect("Failed to parse modified block");

    TokenStream::from(quote! { #input_fn })
}
