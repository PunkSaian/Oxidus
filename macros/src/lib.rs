#![feature(log_syntax)]
#![allow(clippy::cargo_common_metadata, clippy::missing_panics_doc)]
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Data, DeriveInput, Expr, FnArg, Ident, ItemFn, LitStr, Meta, Token, Type,
};

struct SignatureInput {
    pattern: LitStr,
}

impl Parse for SignatureInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pattern: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn sig(input: TokenStream) -> TokenStream {
    let sig_input = syn::parse_macro_input!(input as SignatureInput);
    let pattern_str = sig_input.pattern.value();

    let mut pattern = Vec::new();
    let mut mask = Vec::new();

    for part in pattern_str.split_whitespace() {
        match part {
            "?" | "??" => {
                pattern.push(0x0);
                mask.push(b'?');
            }
            _ => {
                let byte = u8::from_str_radix(part, 16)
                    .unwrap_or_else(|_| panic!("Invalid hex byte: {part}"));
                pattern.push(byte);
                mask.push(b'x');
            }
        }
    }

    TokenStream::from(quote! {
        {
            crate::util::signature_scanner::Signature::new(
                vec![#(#pattern),*],
                vec![#(#mask),*]
            )
        }
    })
}

#[allow(clippy::too_many_lines)]
#[proc_macro_attribute]
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

#[allow(clippy::too_many_lines)]
#[proc_macro_attribute]
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
        drop(hook);
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

#[proc_macro_attribute]
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
struct SettingsMacro {
    groups: Vec<Group>,
}

struct Group {
    name: Ident,
    entries: Vec<Entry>,
}

enum Entry {
    Value {
        name: Ident,
        ty: Ident,
        default: Expr,
    },
    Nested(Group),
}

impl Parse for SettingsMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut groups = Vec::new();

        while !input.is_empty() {
            let group = Group::parse(input)?;
            groups.push(group);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(SettingsMacro { groups })
    }
}

impl Group {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let content;
        braced!(content in input);

        let mut entries = Vec::new();
        while !content.is_empty() {
            if content.peek(Ident) && content.peek2(Token![:]) {
                entries.push(Entry::parse(&content)?);
            } else if content.peek(Ident) && !content.peek2(Token![:]) {
                entries.push(Entry::parse_nested(&content)?);
            }

            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        Ok(Group { name, entries })
    }
}

impl Entry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse::<Ident>()?;
        input.parse::<Token![,]>()?;
        let default = input.parse()?;

        Ok(Entry::Value { name, ty, default })
    }

    fn parse_nested(input: ParseStream) -> syn::Result<Self> {
        let group = Group::parse(input)?;
        Ok(Entry::Nested(group))
    }
}

#[proc_macro]
pub fn settings(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as SettingsMacro);

    let mut map_builders = Vec::new();

    for group in &config.groups {
        let group_name = group.name.to_string();
        let entries = build_entries(&group.entries);

        map_builders.push(quote! {
            {
                let mut group_map = ::std::collections::HashMap::new();
                #(#entries)*
                (
                    #group_name.to_string(),
                    Entry::Group(group_map)
                )
            }
        });
    }

    let expanded = quote! {
        {
            let mut settings_map = ::std::collections::HashMap::new();
            #(
                let (key, value) = #map_builders;
                settings_map.insert(key, value);
            )*
            settings_map
        }
    };

    TokenStream::from(expanded)
}

fn build_entries(entries: &[Entry]) -> Vec<proc_macro2::TokenStream> {
    let mut tokens = Vec::new();

    for entry in entries {
        match entry {
            Entry::Value { name, ty, default } => {
                let key = name.to_string();
                tokens.push(quote! {
                    group_map.insert(
                        #key.to_string(),
                        Entry::Value(
                            EntryValue::#ty(#default),
                            EntryValue::#ty(#default)
                        )
                    );
                });
            }
            Entry::Nested(group) => {
                let group_name = group.name.to_string();
                let nested_entries = build_entries(&group.entries);

                tokens.push(quote! {
                    {
                        let mut nested_map = ::std::collections::HashMap::new();
                        #(#nested_entries)*
                        group_map.insert(
                            #group_name.to_string(),
                            Entry::Group(nested_map.clone()),
                        );
                    }
                });
            }
        }
    }

    tokens
}
