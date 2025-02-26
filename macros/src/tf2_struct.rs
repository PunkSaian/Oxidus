use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident, Meta};

pub fn tf2_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let generics = &input.generics;

    //#[tf2_struct(base_class = <base class>, vmt = <vmt>)]

    //group attrs into touples 3
    let base_class = proc_macro2::TokenStream::from(attr).into_iter().next();

    let Data::Struct(data) = input.data else {
        panic!("tf2_struct can only be used on structs");
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
                let offset = literal.to_string().parse::<usize>().unwrap();

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

    let mut generated_fields = vec![];
    let mut last = None;

    for (i, (ident, ty, offset)) in fields.iter().enumerate() {
        let padding_ident = Ident::new(format!("_padding_{i}").as_str(), Span::call_site());

        if let Some((last_offset, last_type)) = last {
            generated_fields.push(quote! {
                #padding_ident: [u8; (#offset) - (::std::mem::size_of::<#last_type>() + #last_offset)],
            });
        } else {
            generated_fields.push(quote! {
                #padding_ident: [u8; #offset],
            });
        }
        generated_fields.push(quote! {
            pub #ident: #ty,
        });
        last = Some((offset, ty));
    }

    let generated_impl = base_class.map(|base_class: proc_macro2::TokenTree| {
        let base_class = Ident::new(base_class.to_string().as_str(), Span::call_site());
        quote! {
            impl ::std::ops::Deref for #struct_name {
                type Target = #base_class;

                fn deref(&self) -> &Self::Target {
                    unsafe { &*(self as *const Self as *const Self::Target) }
                }
            }
            impl ::std::ops::DerefMut for #struct_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { &mut *(self as *mut Self as *mut Self::Target) }
                }
            }
        }
    });

    let generated_struct = quote! {
        #[repr(C)]
        pub struct #struct_name #generics {
            #(#generated_fields)*
        }
    };

    let output = quote! {
        #generated_struct
        #generated_impl
    };

    output.into()
}
