use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Type};

pub fn vmt_hook(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let sig = &input_fn.sig;
    let unsafety = sig.unsafety;
    let abi = sig.abi.clone();
    let params: Vec<Type> = sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("Receiver not allowed in VMT hook function"),
            syn::FnArg::Typed(pat_type) => *pat_type.ty.clone(),
        })
        .collect();
    let return_type = &sig.output;

    let original_block = &input_fn.block;
    // Generate original retrieval code
    input_fn.block = syn::parse2(quote! {
        {
            let original_function = {
                let hook_fn = unsafe {std::mem::transmute::<_,crate::hook::vmt::FnPtr>(#fn_name as *const ())};
                let registry = crate::hook::vmt::VMT_HOOK_REGISTRY.get().unwrap()
                    .read()
                    .expect("Failed to acquire registry lock");
                let original_ptr = registry.get(&hook_fn)
                    .expect("Original function not found for hook");
                unsafe { std::mem::transmute::<_, #unsafety #abi fn(#(#params),*) #return_type>(*original_ptr) }
            };
            #original_block
        }

    }).expect("Failed to parse modified block");

    TokenStream::from(quote! { #input_fn })
}
