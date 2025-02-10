use crate::netvar_dumper::sdk::UnparsedClientClass;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VMTBaseClient {
    _pad1: [usize; 6],
    pub level_init_post_entity: extern "C" fn(*const BaseClient) -> (),
    pub level_shutdown: extern "C" fn(*const BaseClient) -> (),
    pub get_all_classes: extern "C" fn(*const BaseClient) -> *const UnparsedClientClass,
}

pub struct BaseClient {
    pub(crate) vtable: &'static VMTBaseClient,
}
