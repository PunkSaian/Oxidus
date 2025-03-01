use std::sync::OnceLock;

use crate::{sdk::module_names, util::create_interface};

use super::{
    client::Client, client_entity_list::ClientEntityList, client_mode::ClientMode, engine::Engine, engine_cvar::CVar, engine_render_view::EngineRenderView, engine_trace::EngineTrace, global_vars::GlobalVars, gui_surface::GuiSurface, interface_names, material_system::MaterialSystem, model_info::ModelInfo, model_render::ModelRender
};

pub struct Interfaces {
    pub entity_list: &'static ClientEntityList,
    pub client: &'static Client,
    pub engine: &'static Engine,
    pub engine_render_view: &'static EngineRenderView,
    pub gui_surface: &'static GuiSurface,
    pub client_mode: &'static ClientMode,
    pub global_vars: &'static GlobalVars,
    pub model_info: &'static ModelInfo,
    pub engine_trace: &'static EngineTrace,
    pub engine_cvar: &'static CVar,
    pub model_render: &'static ModelRender,
    pub material_system: &'static MaterialSystem,
}

unsafe impl Sync for Interfaces {}
unsafe impl Send for Interfaces {}

static INTERFACES: OnceLock<Interfaces> = OnceLock::new();

impl Interfaces {
    pub fn init() {
        INTERFACES.get_or_init(|| {
            let client =
                create_interface::<Client>(module_names::CLIENT, interface_names::CLIENT).unwrap();

            let client_mode = Self::get_client_mode(client);
            let global_vars = Self::get_global_vars(client);

            Interfaces {
                entity_list: create_interface::<ClientEntityList>(
                    module_names::CLIENT,
                    interface_names::CLIENT_ENTITY_LIST,
                )
                .unwrap(),

                client,

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
                client_mode,
                global_vars,
                model_info: create_interface::<ModelInfo>(
                    module_names::ENGINE,
                    interface_names::MODEL_INFO,
                )
                .unwrap(),
                engine_trace: create_interface::<EngineTrace>(
                    module_names::ENGINE,
                    interface_names::ENGINE_TRACE,
                )
                .unwrap(),
                engine_cvar: create_interface::<CVar>(
                    module_names::VSTDLIB,
                    interface_names::ENGINE_CVAR,
                )
                .unwrap(),
                model_render: create_interface::<ModelRender>(
                    module_names::ENGINE,
                    interface_names::ENGINE_MODEL,
                ).unwrap(),
                material_system: create_interface::<MaterialSystem>(
                    module_names::MATERIAL_SYSTEM,
                    interface_names::MATERIAL_SYSTEM,
                ).unwrap()
            }
        });
    }

    pub fn get_client_mode(client: &'static Client) -> &'static ClientMode {
        unsafe {
            let vmt_ptr = *std::ptr::from_ref(client).cast::<*const *const ()>();
            let hud_process_input = *vmt_ptr.offset(10);
            let client_mode_relative_offset =
                hud_process_input.byte_add(3).cast::<u32>().read_unaligned() as usize;

            let client_mode = hud_process_input
                .byte_add(3 + 4 + client_mode_relative_offset)
                .cast::<&'static ClientMode>();
            *client_mode
        }
    }

    pub fn get_global_vars(client: &'static Client) -> &'static GlobalVars {
        unsafe {
            let vmt_ptr = *std::ptr::from_ref(client).cast::<*const *const ()>();
            let hud_update = *vmt_ptr.offset(11);
            let global_vars_relative_offset =
                hud_update.byte_add(0x16).cast::<u32>().read_unaligned() as usize;

            let global_vars = hud_update
                .byte_add(0x16 + 4 + global_vars_relative_offset)
                .cast::<&'static GlobalVars>();
            *global_vars
        }
    }

    pub fn get() -> &'static Self {
        INTERFACES.get().expect("Interfaces not initialized")
    }
}

#[macro_export]
macro_rules! i {
    () => {
        $crate::sdk::interface::interfaces::Interfaces::get()
    };
}
