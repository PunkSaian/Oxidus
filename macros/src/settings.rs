use heck::ToPascalCase;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, Expr, Ident, Token, Type};

struct SettingsMacro {
    groups: Vec<Group>,
}

struct Group {
    name: Ident,
    items: Vec<GroupItem>,
}

enum GroupItem {
    Field(FieldDef),
    Group(Group),
}

struct FieldDef {
    field_name: Ident,
    field_type: Type,
    default_value: Expr,
}

impl Parse for SettingsMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut groups = Vec::new();

        while !input.is_empty() {
            groups.push(Group::parse(input)?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(SettingsMacro { groups })
    }
}

impl Parse for Group {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let items = Punctuated::<GroupItem, Token![,]>::parse_terminated(&content)?;

        Ok(Group {
            name,
            items: items.into_iter().collect(),
        })
    }
}

impl Parse for GroupItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let field_type = input.parse()?;
            input.parse::<Token![,]>()?;
            let default_value = input.parse()?;
            Ok(GroupItem::Field(FieldDef {
                field_name: ident,
                field_type,
                default_value,
            }))
        } else if lookahead.peek(syn::token::Brace) {
            let subgroup = Group::parse_subgroup(input, ident)?;
            Ok(GroupItem::Group(subgroup))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Group {
    fn parse_subgroup(input: ParseStream, name: Ident) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let items = Punctuated::<GroupItem, Token![,]>::parse_terminated(&content)?;
        Ok(Group {
            name,
            items: items.into_iter().collect(),
        })
    }
}

pub fn settings(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as SettingsMacro);

    let group_code = config.groups.iter().map(generate_group_code);

    let settings_fields = config.groups.iter().map(|group| {
        let group_name = &group.name;
        let struct_name = group_struct_name(group_name);
        quote! { pub #group_name: #struct_name }
    });

    let settings_defaults = config.groups.iter().map(|group| {
        let group_name = &group.name;
        let struct_name = group_struct_name(group_name);
        quote! { #group_name: #struct_name::default() }
    });

    let settings_impl = {
        let merge_statements = config.groups.iter().map(|group| {
            let group_name = &group.name;
            quote! {
                self.#group_name.merge(&old.#group_name);
            }
        });
        let apply_overwrites_statements = config.groups.iter().map(|group| {
            let group_name = &group.name;
            quote! {
                self.#group_name.apply_overwrites(&bind.#group_name);
            }
        });
        let apply_overwrites_permanent_statements = config.groups.iter().map(|group| {
            let group_name = &group.name;
            quote! {
                self.#group_name.apply_overwrites_permanent(&bind.#group_name);
            }
        });
        let clear_overwrites_statements = config.groups.iter().map(|group| {
            let group_name = &group.name;
            quote! {
                self.#group_name.clear_overwrites();
            }
        });
        let generate_overwrites_statements = config.groups.iter().map(|group| {
            let group_name = &group.name;
            quote! {
                self.#group_name.generate_overwrites(&new.#group_name);
            }
        });

        quote! {
            impl Settings {
                pub fn merge(&mut self, old: &Self) {
                    #(#merge_statements)*
                }
                pub fn apply_overwrites(&mut self, bind: &Self) {
                    #(#apply_overwrites_statements)*
                }
                pub fn apply_overwrites_permanent(&mut self, bind: &Self) {
                    #(#apply_overwrites_permanent_statements)*
                }
                pub fn clear_overwrites(&mut self) {
                    #(#clear_overwrites_statements)*
                }
                pub fn generate_overwrites(&mut self, new: &Self) {
                    #(#generate_overwrites_statements)*
                }

            }
        }
    };

    let expanded = quote! {
        #(#group_code)*

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Settings {
            #(#settings_fields),*
        }

        impl Default for Settings {
            fn default() -> Self {
                Self {
                    #(#settings_defaults),*
                }
            }
        }

        #settings_impl
    };

    TokenStream::from(expanded)
}

#[allow(clippy::too_many_lines)]
fn generate_group_code(group: &Group) -> TokenStream2 {
    let subgroup_code = group.items.iter().filter_map(|item| {
        if let GroupItem::Group(subgroup) = item {
            Some(generate_group_code(subgroup))
        } else {
            None
        }
    });

    let struct_name = group_struct_name(&group.name);

    let fields = group.items.iter().map(|item| match item {
        GroupItem::Field(field) => {
            let field_name = &field.field_name;
            let field_type = &field.field_type;
            quote! { pub #field_name: SettingsField<#field_type> }
        }
        GroupItem::Group(subgroup) => {
            let subgroup_name = &subgroup.name;
            let subgroup_struct = group_struct_name(subgroup_name);
            quote! { pub #subgroup_name: #subgroup_struct }
        }
    });

    let default_fields = group.items.iter().map(|item| match item {
        GroupItem::Field(field) => {
            let field_name = &field.field_name;
            let default_value = &field.default_value;
            quote! {
                #field_name: SettingsField {
                    value: #default_value.clone(),
                    default: #default_value.clone(),
                    overwrite: None,
                }
            }
        }
        GroupItem::Group(subgroup) => {
            let subgroup_name = &subgroup.name;
            let subgroup_struct = group_struct_name(subgroup_name);
            quote! { #subgroup_name: #subgroup_struct::default() }
        }
    });

    let struct_impl = {
        let merge_statements = group.items.iter().map(|item| match item {
            GroupItem::Field(field) => {
                let field_name = &field.field_name;
                quote! {
                    self.#field_name.merge(&old.#field_name);
                }
            }
            GroupItem::Group(subgroup) => {
                let subgroup_name = &subgroup.name;
                quote! {
                    self.#subgroup_name.merge(&old.#subgroup_name);
                }
            }
        });

        let clear_overwrites_statements = group.items.iter().map(|item| match item {
            GroupItem::Field(field) => {
                let field_name = &field.field_name;
                quote! {
                    self.#field_name.clear_overwrites();
                }
            }
            GroupItem::Group(subgroup) => {
                let subgroup_name = &subgroup.name;
                quote! {
                    self.#subgroup_name.clear_overwrites();
                }
            }
        });

        let apply_overwrites_statements = group.items.iter().map(|item| match item {
            GroupItem::Field(field) => {
                let field_name = &field.field_name;
                quote! {
                    self.#field_name.apply_overwrite(&bind.#field_name);
                }
            }
            GroupItem::Group(subgroup) => {
                let subgroup_name = &subgroup.name;
                quote! {
                    self.#subgroup_name.apply_overwrites();
                }
            }
        });
        let apply_overwrites_permanent_statements = group.items.iter().map(|item| match item {
            GroupItem::Field(field) => {
                let field_name = &field.field_name;
                quote! {
                    self.#field_name.apply_overwrite_permanent(&bind.#field_name);
                }
            }
            GroupItem::Group(subgroup) => {
                let subgroup_name = &subgroup.name;
                quote! {
                    self.#subgroup_name.apply_overwrites();
                }
            }
        });

        let generate_overwrites_statements = group.items.iter().map(|item| match item {
            GroupItem::Field(field) => {
                let field_name = &field.field_name;
                quote! {
                    self.#field_name.generate_overwrites(&new.#field_name);
                }
            }
            GroupItem::Group(subgroup) => {
                let subgroup_name = &subgroup.name;
                quote! {
                    self.#subgroup_name.generate_overwrites(&new.#subgroup_name);
                }
            }
        });

        quote! {
            impl #struct_name {
                pub fn merge(&mut self, old: &Self) {
                    #(#merge_statements)*
                }
                pub fn apply_overwrites(&mut self, bind: &Self) {
                    #(#apply_overwrites_statements)*
                }
                pub fn apply_overwrites_permanent(&mut self, bind: &Self) {
                    #(#apply_overwrites_permanent_statements)*
                }
                pub fn clear_overwrites(&mut self) {
                    #(#clear_overwrites_statements)*
                }
                pub fn generate_overwrites(&mut self, new: &Self) {
                    #(#generate_overwrites_statements)*
                }
            }
        }
    };

    quote! {
        #(#subgroup_code)*

        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct #struct_name {
            #(#fields),*
        }

        impl Default for #struct_name {
            fn default() -> Self {
                Self {
                    #(#default_fields),*
                }
            }
        }

        #struct_impl
    }
}

fn group_struct_name(name: &Ident) -> Ident {
    Ident::new(
        &format!("{}Settings", name.to_string().to_pascal_case()),
        Span::call_site(),
    )
}
