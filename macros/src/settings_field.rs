use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprField, Meta, Path};

pub fn settings_field(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute as a path directly
    let field_path = parse_macro_input!(attr as ExprField);

    let item = parse_macro_input!(item as Expr);

    let syn::Member::Named(var_name) = &field_path.member else {
        panic!("Expected named field");
    };

    let expanded = quote! {
        ui.disabled(#field_path.get_mut().is_none(), || {
            if let Some(#var_name) = #field_path.get_mut() {
                {
                    #item
                }
            } else {
                let #var_name = &mut #field_path.get().clone();
                {
                    #item
                }
            }
        })
    };

    TokenStream::from(expanded)
}
