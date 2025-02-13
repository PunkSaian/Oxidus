use std::sync::OnceLock;

use crate::{sdk::module_names, util::create_interface};

use super::{
    client::Client, client_entity_list::ClientEntityList, engine::Engine,
    engine_render_view::EngineRenderView, interface_names,
};

pub struct Interfaces {
    pub entity_list: &'static ClientEntityList,
    pub client: &'static Client,
    pub engine: &'static Engine,
    pub engine_render_view: &'static EngineRenderView,
}

unsafe impl Sync for Interfaces {}
unsafe impl Send for Interfaces {}

pub static INTERFACES: OnceLock<Interfaces> = OnceLock::new();

impl Interfaces {
    pub fn init() {
        INTERFACES.get_or_init(|| Interfaces {
            entity_list: create_interface::<ClientEntityList>(
                module_names::CLIENT,
                interface_names::CLIENT_ENTITY_LIST,
            )
            .unwrap(),

            client: create_interface::<Client>(module_names::CLIENT, interface_names::CLIENT)
                .unwrap(),

            engine: create_interface::<Engine>(module_names::ENGINE, interface_names::ENGINE)
                .unwrap(),

            engine_render_view: create_interface::<EngineRenderView>(
                module_names::ENGINE,
                interface_names::ENGINE_RENDER_VIEW,
            )
            .unwrap(),
        });
    }

    fn get() -> &'static Self {
        INTERFACES.get().expect("Interfaces not initialized")
    }
}
