use macros::vmt;

use crate::sdk::client_class::UnparsedClientClass;

pub struct BaseClient;

#[vmt(BaseClient)]
pub struct BaseClient {
    #[offset(6)]
    pub level_init_post_entity: extern "C" fn(),
    #[offset(7)]
    pub level_shutdown: extern "C" fn(),
    #[offset(8)]
    pub get_all_classes: extern "C" fn() -> *const UnparsedClientClass,
}
