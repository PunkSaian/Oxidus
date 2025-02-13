use macros::vmt;

use crate::{netvar_dumper::sdk::UnparsedClientClass, sdk::vmts::VMTBaseEntity};

#[vmt]
pub struct BaseClient {
    #[offset(7)]
    pub level_init_post_entity: extern "C" fn(),
    pub level_shutdown: extern "C" fn(),
    pub get_all_classes: extern "C" fn() -> *const UnparsedClientClass,
}

pub struct BaseClient;

impl VMTBaseClient for BaseClient {
}
