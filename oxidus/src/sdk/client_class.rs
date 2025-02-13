use std::{
    collections::HashMap,
    ffi::{c_void, CStr},
};

use crate::sdk::class_id::ClassId;

#[derive(Debug, Clone)]
pub struct NetvarStruct {
    pub name: String,
    pub fields: Vec<(String, Netvar)>,
    pub baseclass: Option<String>,
    pub custom: HashMap<String, Netvar>,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum SendPropType {
    Int,
    Float,
    Vector,
    Vector2D,
    String,
    Array,
    Datatable,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RecvProp {
    pub var_name: *const i8,
    pub recv_type: SendPropType,
    pub flags: i32,
    pub string_buffer_size: i32,
    pub inside_array: bool,
    pub extra_data: *const c_void,
    pub array_prop: *const RecvProp,
    pub array_length_proxy_fn: extern "C" fn(),
    pub proxy_fn: extern "C" fn(),
    pub data_table_proxy_fn: extern "C" fn(),
    pub data_table: *const RecvTable,
    pub offset: i32,
    pub element_tride: i32,
    pub elements: i32,
    pub parent_array_prop_name: *const i8,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RecvTable {
    pub props: *mut RecvProp,
    pub props_count: i32,
    pub decoder: *const c_void,
    pub table_name: *const i8,
    pub initialized: bool,
    pub in_main_list: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UnparsedClientClass {
    _pad1: [[u8; 8]; 2],
    pub network_name: *const char,
    pub recv_table: &'static RecvTable,
    pub next: *const UnparsedClientClass,
    pub class_id: ClassId,
}

#[derive(Debug, Clone)]
pub struct Netvar {
    pub r#type: NetvarType,
    pub offset: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum NetvarType {
    Int,
    Bool,
    Float,
    Vector3,
    Vector2,
    String {
        buffer_size: usize,
    },
    Array {
        r#type: Box<NetvarType>,
        length: usize,
    },
    Datatable(NetvarStruct),
}

impl NetvarType {
    pub fn to_rust_type(&self) -> String {
        match self {
            Self::Int => "i32".to_owned(),
            Self::Bool => "bool".to_owned(),
            Self::Float => "f32".to_owned(),
            Self::Vector3 => "Vector3".to_owned(),
            Self::Vector2 => "Vector2".to_owned(),
            Self::String { buffer_size } => format!("[i8; {buffer_size}]"),
            //Self::Bool => "bool".to_owned(),
            Self::Array { r#type, length } => {
                format!("[{}; {length}]", r#type.to_rust_type())
            }
            Self::Datatable(NetvarStruct { name, .. }) => name.clone(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClientClass {
    pub network_name: String,
    pub next: *const UnparsedClientClass,
    pub recv_table: &'static RecvTable,
    pub class_id: ClassId,
}
impl From<UnparsedClientClass> for ClientClass {
    fn from(value: UnparsedClientClass) -> Self {
        let network_name = unsafe {
            CStr::from_ptr(value.network_name.cast::<i8>())
                .to_str()
                .unwrap_or("")
                .to_owned()
        };
        ClientClass {
            network_name,
            next: value.next,
            recv_table: value.recv_table,
            class_id: ClassId::CSun,
        }
    }
}
