use macros::{tf2_struct, vmt};

use crate::sdk::{bindings::BaseEntity, networkable::Networkable};

#[repr(C)]
#[derive(Debug)]
pub struct UtilMemory<T> {
    pub memory: *const T,
    pub allocation_count: i32,
    pub gow_size: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct UtilLinekdListElement<T, I> {
    pub element: T,
    pub previous: I,
    pub netxt: I,
}

#[repr(C)]
#[derive(Debug)]
pub struct UtilLinekdList<T, I> {
    pub memory: UtilMemory<UtilLinekdListElement<T, I>>,
    pub head: I,
    pub tail: I,
    pub first_free: I,
    pub elemetn_count: I,
    pub num_allocated: I,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EntityCacheInfo {
    pub networkable: *const Networkable,
    pub base_entities_index: u8,
}

#[tf2_struct()]
pub struct ClientEntityList {
    #[offset(8)]
    ent_ptr_array: *const (),
    #[offset(40)]
    num_server_entities: i32,
    #[offset(44)]
    max_server_entities: i32,
    #[offset(48)]
    num_clietn_non_networkable: i32,
    #[offset(52)]
    max_used_server_index: i32,
    #[offset(56)]
    cache: [EntityCacheInfo; 4096],
    #[offset(131128)]
    base_entities: UtilLinekdList<*const BaseEntity, u8>,
}

#[vmt]
pub struct ClientEntityList {
    #[offset(3)]
    pub get_client_entity_from_index: extern "C" fn(entindex: i32) -> &'static BaseEntity,
    #[offset(4)]
    pub get_client_entity_from_handle: extern "C" fn(handle: i32) -> &'static BaseEntity,
}

