use std::sync::OnceLock;

use macros::sig;

use crate::{sdk::module_names, util::create_interface};

use super::{
    client::Client, client_entity_list::ClientEntityList, client_mode::ClientMode, engine::Engine,
    engine_render_view::EngineRenderView, gui_surface::GuiSurface, interface_names,
};

pub struct Interfaces {
    pub entity_list: &'static ClientEntityList,
    pub client: &'static Client,
    pub engine: &'static Engine,
    pub engine_render_view: &'static EngineRenderView,
    pub gui_surface: &'static GuiSurface,
    pub client_mode: &'static ClientMode,
}

unsafe impl Sync for Interfaces {}
unsafe impl Send for Interfaces {}

pub static INTERFACES: OnceLock<Interfaces> = OnceLock::new();

impl Interfaces {
    pub fn init() {
        INTERFACES.get_or_init(|| {
            type GetClientModeFn = extern "C" fn() -> &'static ClientMode;
            let client_mode_sig =
                //sig!("0f b6 05 ? ? ? ? 84 c0 74 0d 48 8d 05 ? ? ? ? c3 ? ? ? ? ? 55 48 8d 3d ? ? ? ? 48 89 e5 41 54 48 83 ec ? e8 e2 3b ? ? 85 c0 75 0e 4c 8b 65 f8 48 8d 05 ? ? ? ? c9 c3 ? 48 8d 3d ? ? ? ? 38 94 fe ? ? 48 8d 15 ? ? ? ? 48 8d 35 ? ? ? ? 48 8d 3d ? ? ? ? e8 fa 3f ? ? 48 8d 3d ? ? ? ? e8 ee 6c ? ? 4c 8b 65 f8 48 8d 05 ? ? ? ? c9 c3 49 89 c4 e9 39 b4 ? ? 90 0f 1f 84 00 00 00 00 00 55 48 89 e5");

                sig!("0F B6 05 D9 16 30 01 84 C0 ? ? 48 8D 05 EE 16 30 01 ? 55 48 8D 3D C0 16 30 01 48 89 E5 41 54 48 83 EC 08 ? ? ? ? ? 85 C0 ? ? 4C 8B 65 F8 48 8D 05 C3 16 30 01 C9 ? 48 8D 3D B9 16 30 01 ? ? ? ? ? 48 8D 15 0D CB 1D 01 48 8D 35 A6 16 30 01 48 8D 3D 9F F4 FF FF ? ? ? ? ? 48 8D 3D 73 16 30 01 ? ? ? ? ? 4C 8B 65 F8 48 8D 05 83 16 30 01 C9 ? ");

            let get_client_mode_fn = unsafe {
                (std::mem::transmute::<_,GetClientModeFn>(dbg!(client_mode_sig.scan_module(module_names::CLIENT).unwrap()))
                    )
            };
            dbg!(get_client_mode_fn);
            dbg!(get_client_mode_fn());

            Interfaces {
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
                gui_surface: create_interface::<GuiSurface>(
                    module_names::GUIMATSURFACE,
                    interface_names::GUI_SURFACE,
                )
                .unwrap(),
                client_mode: get_client_mode_fn(),
            }
        });
    }

    pub fn get() -> &'static Self {
        INTERFACES.get().expect("Interfaces not initialized")
    }
}
