use proc_macro::TokenStream;
use quote::quote;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, Token,
};

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

//TODO(nullptr): propper ping getting

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
                            EntryValue::#ty(#default),
                            None
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
