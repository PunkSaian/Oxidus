use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Meta, Type};
use quote::quote;


pub fn vmt(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    //let base_class = attr.into_iter().next();
    let Data::Struct(data) = input.data else {
        panic!("can only be used on structs");
    };

    let mut fields = match data.fields {
        syn::Fields::Named(fields) => {
            let mut fields_with_offsets = Vec::new();
            for field in fields.named {
                assert!(
                    (field.attrs.len() == 1),
                    "Each field must have offset attribute"
                );
                let attr = field.attrs[0].clone();
                assert!(
                    attr.path().is_ident("offset"),
                    "Each field must have offset attribute"
                );
                let Meta::List(offset) = attr.meta else {
                    panic!("Each field must have offset attribute");
                };
                let tokens = offset.tokens.into_iter().collect::<Vec<_>>();
                assert!((tokens.len() == 1), "Invalid offset attribute");
                let proc_macro2::TokenTree::Literal(literal) = tokens.into_iter().next().unwrap()
                else {
                    panic!("Invalid offset attribute");
                };
                let offset = literal.to_string().parse::<isize>().unwrap();

                fields_with_offsets.push((field.ident.clone(), field.ty.clone(), offset));
            }

            fields_with_offsets
        }
        syn::Fields::Unnamed(_) => panic!("Unnamed fields are not supported"),
        syn::Fields::Unit => {
            vec![]
        }
    };

    fields.sort_by_key(|(_, _, offset)| *offset);

    let mut generated_funcs = vec![];

    for (ident, ty, offset) in &fields {
        let Type::BareFn(func) = ty else {
            panic!("Only function pointers are allowed in vmt struct")
        };

        let args_with_types = func.inputs.clone();
        let args = func
            .inputs
            .clone()
            .into_iter()
            .map(|arg| {
                arg.name
                    .expect("this macro doesnt support unnamed arguments")
                    .0
            })
            .collect::<Vec<_>>();
        let ret = func.output.clone();
        let function = if args_with_types.is_empty() {
            quote! {
                pub fn #ident(&self) #ret {
                    unsafe {
                        let vtable = self as *const Self as *const *const extern "C" fn(&Self) #ret;
                        let func = (*vtable).offset(#offset);

                        (*func)(self, #(#args),*)
                    }
                }
            }
        } else {
            quote! {
                pub fn #ident(&self, #args_with_types) #ret {
                    unsafe {
                        let vtable = self as *const Self as *const *const extern "C" fn(&Self,#args_with_types) #ret;
                        let func = (*vtable).offset(#offset);
                        (*func)(self, #(#args),*)
                    }
                }
            }
        };

        generated_funcs.push(function);
    }

    let generated_trait = quote! {
        impl #struct_name {
            #(#generated_funcs)*
        }
    };

    let output = quote! {
        #generated_trait
    };
    output.into()
}
