use sdk::{ClientClass, Netvar, NetvarType, RecvTable, SendPropType};
use std::collections::HashSet;
use std::io::Write;
use std::{collections::HashMap, ffi::CStr, fs::File, slice};

use crate::{sdk::interface::base_client::BaseClient, OxidusResult};

pub mod sdk;

#[derive(Debug, Clone)]
pub struct NetvarStruct {
    name: String,
    fields: Vec<(String, Netvar)>,
    baseclass: Option<String>,
    /// `Netvar::Object`
    custom: HashMap<String, Netvar>,
}

#[allow(clippy::too_many_lines)]
pub fn parse_table(table: &RecvTable) -> NetvarStruct {
    let struct_name = unsafe {
        CStr::from_ptr(table.table_name)
            .to_str()
            .unwrap()
            .to_string()
            .replace("DT_", "")
    };
    let mut fields = Vec::new();
    let mut dataclasses = HashMap::new();
    let mut baseclass = None;

    let mut arrays = Vec::new();
    let mut inlined_arrays = HashMap::new();
    unsafe {
        let mut props = slice::from_raw_parts_mut(table.props, table.props_count as usize).to_vec();
        props.sort_by_key(|prop| prop.offset);
        for prop in props {
            let mut name = CStr::from_ptr(prop.var_name).to_str().unwrap().to_string();

            if name.contains('.') {
                name = name.replace('.', "_");
            }
            if name.parse::<i64>().is_ok() {
                name = format!("i{name}");
            }
            let name = name.trim_matches('"').to_owned();

            if name.ends_with("[0]") {
                let name = name.split('[').next().unwrap().to_owned();
                inlined_arrays.insert(name.clone(), (prop, 1));
                continue;
            }
            if name.contains('[') {
                let name = name.split('[').next().unwrap().to_owned();
                let name = name.split('[').next().unwrap().to_owned();
                if let Some(array) = inlined_arrays.get_mut(&name) {
                    array.1 += 1;
                    continue;
                }
                eprintln!("skipping werd vec struct {name} in {struct_name}");
                continue;
            }

            match prop.recv_type {
                SendPropType::Int
                | SendPropType::Float
                | SendPropType::Vector
                | SendPropType::Vector2D => {
                    let r#type = match prop.recv_type {
                        SendPropType::Int => NetvarType::Int,
                        SendPropType::Float => NetvarType::Float,
                        SendPropType::Vector => NetvarType::Vector2,
                        SendPropType::Vector2D => NetvarType::Vector3,
                        _ => unreachable!(),
                    };
                    let netvar = Netvar {
                        r#type,
                        offset: prop.offset as usize,
                        name: name.clone(),
                    };
                    fields.push((name.clone(), netvar));
                }
                SendPropType::Datatable => {
                    let datatable_struct = parse_table(&*prop.data_table);
                    if datatable_struct.name.starts_with('m') {
                        let netvar = Netvar {
                            r#type: NetvarType::Array {
                                r#type: Box::new(
                                    datatable_struct.fields.first().unwrap().1.r#type.clone(),
                                ),
                                length: datatable_struct.fields.len(),
                            },
                            offset: prop.offset as usize,
                            name: name.clone(),
                        };

                        fields.push((name.clone(), netvar));
                        continue;
                    }
                    if name == "baseclass" {
                        baseclass = Some(datatable_struct.name);
                        continue;
                    }
                    let netvar = Netvar {
                        r#type: NetvarType::Datatable(datatable_struct.clone()),
                        offset: prop.offset as usize,
                        name: name.clone(),
                    };
                    if prop.offset == 0 {
                        dataclasses.insert(name.clone(), netvar);
                        continue;
                    }
                    fields.push((name.clone(), netvar));
                }
                SendPropType::String => {
                    let netvar = Netvar {
                        r#type: NetvarType::String {
                            buffer_size: prop.string_buffer_size as usize,
                        },
                        offset: prop.offset as usize,
                        name: name.clone(),
                    };
                    fields.push((name.clone(), netvar));
                }

                SendPropType::Array => {
                    arrays.push((name.clone(), prop));
                }
            }
        }
    }

    for (name, (prop, len)) in inlined_arrays {
        let r#type = match prop.recv_type {
            SendPropType::Int => NetvarType::Int,
            SendPropType::Float => NetvarType::Float,
            SendPropType::Vector => NetvarType::Vector2,
            SendPropType::Vector2D => NetvarType::Vector3,
            SendPropType::String => NetvarType::String {
                buffer_size: prop.string_buffer_size as usize,
            },

            _ => NetvarType::Unknown,
        };
        let netvar = Netvar {
            r#type: NetvarType::Array {
                r#type: Box::new(r#type),
                length: len,
            },
            offset: prop.offset as usize,
            name: name.clone(),
        };
        fields.push((name.clone(), netvar));
    }

    for (name, prop) in arrays {
        let mut element_name = name.clone();
        let mut element_field = fields.iter().find(|x| x.0 == element_name);
        if element_field.is_none() {
            element_name += "_element";
            element_field = fields.iter().find(|x| x.0 == element_name);
        }
        let Some(element_field) = element_field else {
            let netvar = Netvar {
                r#type: NetvarType::Array {
                    r#type: Box::new(NetvarType::Unknown),
                    length: prop.elements as usize,
                },
                offset: prop.offset as usize,
                name: name.clone(),
            };
            fields.push((name.clone(), netvar));
            continue;
        };
        let netvar = Netvar {
            r#type: NetvarType::Array {
                r#type: Box::new(element_field.1.r#type.clone()),
                length: prop.elements as usize,
            },
            offset: element_field.1.offset,
            name: name.clone(),
        };

        fields.retain(|(field_name, _)| field_name != &element_name);
        fields.push((name.clone(), netvar));
    }

    fields.sort_by(|a, b| a.1.offset.cmp(&b.1.offset));

    NetvarStruct {
        name: struct_name,
        fields,
        baseclass,
        custom: dataclasses,
    }
}

fn process_netvar_struct(
    netvar_struct: &NetvarStruct,
    class_names: &mut Vec<String>,
    classes: &mut Vec<NetvarStruct>,
) {
    for (_, netvar) in &netvar_struct.fields {
        if let NetvarType::Datatable(
            inner @ NetvarStruct {
                name,
                fields: _,
                baseclass: _,
                custom: _,
            },
        ) = &netvar.r#type
        {
            if !class_names.contains(name) {
                class_names.push(name.clone());
                classes.push(inner.clone());
                process_netvar_struct(inner, class_names, classes);
            }
        }
    }
    for netvar in netvar_struct.custom.values() {
        if let NetvarType::Datatable(
            inner @ NetvarStruct {
                name,
                fields: _,
                baseclass: _,
                custom: _,
            },
        ) = &netvar.r#type
        {
            if !class_names.contains(name) {
                class_names.push(name.clone());
                classes.push(inner.clone());
                process_netvar_struct(inner, class_names, classes);
            }
        }
    }
}

pub fn dump_netvars(base_client: *const BaseClient) -> OxidusResult {
    let mut client_class: ClientClass = unsafe {
        (base_client.read().vtable.get_all_classes)(base_client)
            .read()
            .into()
    };

    let mut classes = Vec::new();
    loop {
        let table = unsafe { client_class.recv_table.read() };
        let netvar_struct = parse_table(&table);

        classes.push(netvar_struct);

        if client_class.next.is_null() {
            break;
        }
        client_class = unsafe { client_class.next.read().into() };
    }
    let file_path = format!("{}/src/sdk/bindings.rs", env!("CARGO_MANIFEST_DIR"));
    let mut file = File::create(&file_path)?;

    let mut class_names = classes.iter().map(|x| &x.name).cloned().collect::<Vec<_>>();

    for netvar_struct in classes.clone() {
        process_netvar_struct(&netvar_struct, &mut class_names, &mut classes);
    }

    let mut unique_classes = Vec::new();
    let mut seen_names = HashSet::new();
    for class in &classes {
        if seen_names.insert(class.name.clone()) {
            unique_classes.push(class);
        }
    }

    writeln!(
        &mut file,
        "use libc::c_void;\npub type Vector2 = [f32;2];\npub type Vector3 = [f32;3];\npub type Unknown = [u8;0];\npub type Unknown2 = [u8;0];"
    )?;

    writeln!(&mut file)?;

    for netvar_struct in &unique_classes {
        writeln!(
            &mut file,
            "//#[tf2_struct(baseclass = {})]",
            netvar_struct
                .baseclass
                .clone()
                .unwrap_or_else(|| { "None".to_owned() }),
        )?;
        write!(&mut file, "pub struct {}", netvar_struct.name)?;
        if netvar_struct.fields.is_empty() {
            writeln!(&mut file, ";")?;
        } else {
            writeln!(&mut file, " {{")?;
            for (name, netvar) in &netvar_struct.fields {
                if netvar.offset == 0 {
                    writeln!(file, "    //probably invalid")?;
                }
                writeln!(file, "    //#[offset({})]", netvar.offset)?;
                match &netvar.r#type {
                    NetvarType::Datatable(NetvarStruct {
                        name: struct_name, ..
                    }) => {
                        let struct_name = if class_names.contains(struct_name) {
                            struct_name.to_owned()
                        } else {
                            writeln!(&mut file, "    /// unknown struct {struct_name}")?;
                            "*const c_void".to_owned()
                        };
                        writeln!(file, "    pub {name}: {struct_name},")?;
                    }
                    _ => {
                        writeln!(
                            file,
                            "    pub {}: {},",
                            netvar.name,
                            netvar.r#type.to_rust_type()
                        )?;
                    }
                }
            }
            writeln!(&mut file, "}}\n")?;
        }

        if !netvar_struct.custom.is_empty() {
            writeln!(&mut file, "impl {} {{", netvar_struct.name)?;
            for (name, netvar) in &netvar_struct.custom {
                writeln!(
                    &mut file,
                    "    pub type {} = {};",
                    name,
                    netvar.r#type.to_rust_type()
                )?;
            }
            writeln!(&mut file, "}}\n")?;
        }
    }
    Ok(())
}
