use crate::sdk::client_class::{
    ClientClass, Netvar, NetvarStruct, NetvarType, RecvTable, SendPropType,
};
use std::collections::HashSet;
use std::io::Write;
use std::{collections::HashMap, ffi::CStr, fs::File, slice};

use crate::sdk::interface::{client::Client, interface_names};
use crate::sdk::module_names;
use crate::util::create_interface;
use crate::util::error::OxidusResult;

const IGNORE: &[&str] = &[
    "m_flMvMLastDamageTime",
    "entindex",
    "m_pszCustomUpgradesFile",
    "m_bShowCompetitiveMatchSummary",
    "m_iRawValue32",
    "m_flModelWidthScale",
    "m_iStartAttachment",
    "m_iEndAttachment",
    "m_chCurrentSlideLists",
    "m_hProps",
    "m_bShielded",
    "m_uchFrontColor",
    "m_uchBackColor",
    "m_chPoseIndex",
    "m_iControlPointParents",
    "m_LightStyle",
    "m_LightStyle",
    //wtf
    "m_iParentAttachment",
];
const CUSTOM_OVERRIDE: &[&str] = &[
    "team_object_array",
    "healing_array",
    "player_array",
    "HDRColorScale",
    "movetype",
    "movecollide",
    "moveparent",
];
const BOOL_TYPE_OVERRIDES: &[&str] = &[
    "m_skybox3d_fog_enable",
    "m_skybox3d_fog_blend",
    "m_triggerBloat",
    "m_nSurroundType",
    "m_nSolidType",
    "m_nColor",
    "m_fog_enable",
    "m_fog_blend",
    "m_lifeState",
    "m_Flags",
];

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
    let mut custom = HashMap::new();
    let mut baseclass = None;

    let mut arrays = Vec::new();
    let mut inlined_arrays = HashMap::new();
    unsafe {
        let mut props = slice::from_raw_parts_mut(table.props, table.props_count as usize).to_vec();
        props.sort_by_key(|prop| prop.offset);
        for prop in props {
            let mut name = CStr::from_ptr(prop.var_name).to_str().unwrap().to_string();
            if name.parse::<i64>().is_ok() {
                name = format!("i{name}");
            }
            if name.contains('.') {
                name = name.replace('.', "_");
            }
            let name = name.trim_matches('"').to_owned();
            if IGNORE.contains(&name.as_str()) {
                continue;
            }
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
                        SendPropType::Int
                            if name.starts_with("m_b")
                                | BOOL_TYPE_OVERRIDES.contains(&name.as_str()) =>
                        {
                            NetvarType::Bool
                        }
                        SendPropType::Int => NetvarType::Int,
                        SendPropType::Float => NetvarType::Float,
                        SendPropType::Vector2D => NetvarType::Vector2,
                        SendPropType::Vector => NetvarType::Vector3,
                        _ => unreachable!(),
                    };
                    let netvar = Netvar {
                        r#type,
                        offset: prop.offset as usize,
                        name: name.clone(),
                    };

                    if CUSTOM_OVERRIDE.contains(&name.as_str()) {
                        custom.insert(name.clone(), netvar);
                        continue;
                    }
                    fields.push((name.clone(), netvar));
                }
                SendPropType::Datatable => {
                    let datatable_struct = parse_table(&*prop.data_table);
                    if datatable_struct.name.starts_with('m') {
                        let mut length = datatable_struct.fields.len();
                        if name.to_lowercase().contains("bits") {
                            length = (length as f32 / 32f32).ceil() as usize;
                        }
                        let mut r#type = datatable_struct.fields.first().unwrap().1.r#type.clone();
                        if name.starts_with("m_b") {
                            r#type = NetvarType::Bool;
                        }

                        if IGNORE.contains(&name.as_str()) {
                            //warn!("Ignoring field: {}", name);
                            continue;
                        }
                        let netvar = Netvar {
                            r#type: NetvarType::Array {
                                r#type: Box::new(r#type),
                                length,
                            },
                            offset: prop.offset as usize,
                            name: name.clone(),
                        };

                        if CUSTOM_OVERRIDE.contains(&name.as_str()) {
                            custom.insert(name.clone(), netvar);
                            continue;
                        }
                        fields.push((name.clone(), netvar));
                        continue;
                    }

                    if let Some(field) = datatable_struct.fields.first() {
                        if field.1.name == "i000" {
                            if IGNORE.contains(&name.as_str()) {
                                warn!("Ignoring field: {}", name);
                                continue;
                            }
                            let netvar = Netvar {
                                r#type: NetvarType::Array {
                                    r#type: Box::new(field.1.r#type.clone()),
                                    length: datatable_struct.fields.len(),
                                },
                                offset: prop.offset as usize,
                                name: name.clone(),
                            };
                            if CUSTOM_OVERRIDE.contains(&name.as_str()) {
                                custom.insert(name.clone(), netvar);
                                continue;
                            }
                            fields.push((name.clone(), netvar));
                            continue;
                        }
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
                        custom.insert(name.clone(), netvar);
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
                    if CUSTOM_OVERRIDE.contains(&name.as_str()) {
                        custom.insert(name.clone(), netvar);
                        continue;
                    }
                    fields.push((name.clone(), netvar));
                }

                SendPropType::Array => {
                    if IGNORE.contains(&name.as_str()) {
                        warn!("Ignoring field: {}", name);
                        continue;
                    }
                    arrays.push((name.clone(), prop));
                }
            }
        }
    }

    for (name, (prop, length)) in inlined_arrays {
        let r#type = match prop.recv_type {
            SendPropType::Int if name.starts_with("m_b") => NetvarType::Bool,
            SendPropType::Int => NetvarType::Int,
            SendPropType::Float => NetvarType::Float,
            SendPropType::Vector => NetvarType::Vector2,
            SendPropType::Vector2D => NetvarType::Vector3,
            SendPropType::String => NetvarType::String {
                buffer_size: prop.string_buffer_size as usize,
            },
            _ => unreachable!(),
        };

        if IGNORE.contains(&name.as_str()) {
            warn!("Ignoring field: {}", name);
            continue;
        }

        let netvar = Netvar {
            r#type: NetvarType::Array {
                r#type: Box::new(r#type),
                length,
            },
            offset: prop.offset as usize,
            name: name.clone(),
        };

        if CUSTOM_OVERRIDE.contains(&name.as_str()) {
            custom.insert(name.clone(), netvar);
            continue;
        }
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
            if IGNORE.contains(&name.as_str()) {
                warn!("Ignoring field: {}", name);
                continue;
            }
            let netvar = Netvar {
                r#type: NetvarType::Array {
                    r#type: Box::new(NetvarType::Int),
                    length: prop.elements as usize,
                },
                offset: prop.offset as usize,
                name: name.clone(),
            };
            if CUSTOM_OVERRIDE.contains(&name.as_str()) {
                custom.insert(name.clone(), netvar);
                continue;
            }
            fields.push((name.clone(), netvar));
            continue;
        };
        if IGNORE.contains(&name.as_str()) {
            warn!("Ignoring field: {}", name);
            continue;
        }
        let netvar = Netvar {
            r#type: NetvarType::Array {
                r#type: Box::new(element_field.1.r#type.clone()),
                length: prop.elements as usize,
            },
            offset: element_field.1.offset,
            name: name.clone(),
        };

        fields.retain(|(field_name, _)| field_name != &element_name);
        if CUSTOM_OVERRIDE.contains(&name.as_str()) {
            custom.insert(name.clone(), netvar);
            continue;
        }
        fields.push((name.clone(), netvar));
    }

    fields.sort_by(|a, b| a.1.offset.cmp(&b.1.offset));

    NetvarStruct {
        name: struct_name,
        fields,
        baseclass,
        custom,
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

pub fn dump_netvars() -> OxidusResult {
    let base_client =
        create_interface::<Client>(module_names::CLIENT, interface_names::CLIENT).unwrap();
    let mut client_class: ClientClass = unsafe { base_client.get_all_classes().read().into() };

    let mut classes = Vec::new();
    loop {
        let table = client_class.recv_table;
        let netvar_struct = parse_table(table);

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
        "use crate::math::{{Vector2, Vector3}};\nuse macros::tf2_struct;"
    )?;

    writeln!(&mut file)?;

    for netvar_struct in &unique_classes {
        writeln!(
            &mut file,
            "#[tf2_struct({})]",
            netvar_struct.baseclass.clone().unwrap_or_else(String::new),
        )?;

        write!(&mut file, "pub struct {}", netvar_struct.name)?;
        if netvar_struct.fields.is_empty() {
            writeln!(&mut file, ";\n")?;
        } else {
            writeln!(&mut file, " {{")?;
            for (name, netvar) in &netvar_struct.fields {
                if netvar.offset == 0 {
                    writeln!(file, "    //probably invalid")?;
                }
                writeln!(file, "    #[offset({})]", netvar.offset)?;
                match &netvar.r#type {
                    NetvarType::Datatable(NetvarStruct {
                        name: struct_name, ..
                    }) => {
                        let struct_name = if class_names.contains(struct_name) {
                            struct_name.to_owned()
                        } else {
                            panic!("struct_name not found");
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
