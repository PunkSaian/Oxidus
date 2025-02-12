#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::multiple_crate_versions,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::cast_possible_wrap,
    clippy::cargo_common_metadata,
    clippy::cast_sign_loss,
    clippy::missing_panics_doc,
    incomplete_features,
    dead_code
)]
#![feature(inherent_associated_types,generic_const_exprs)]

extern crate thiserror;

use std::ffi::CString;
use std::{sync::Mutex, thread};

use hook::detour::WrappedDetourHook;
use libc::{dlopen, RTLD_NOLOAD, RTLD_NOW};
use log::{error, info};
use modules::init_modules;
use netvar_dumper::dump_netvars;
use overlay::init as init_overlay;
use overlay::unload as unload_overlay;
use prelude::*;
use sdk::interface::base_client::BaseClient;
use sdk::module_names;

#[macro_use]
extern crate log;

mod hook;
mod netvar_dumper;
mod overlay;
mod prelude;
mod sdk;
mod util;
mod modules;

#[allow(clippy::type_complexity)]
static HOOKS: Mutex<Vec<WrappedDetourHook>> = const { Mutex::new(Vec::new()) };

pub fn wait_for_client() {
    let mut logged = false;
    loop {
        let module = CString::new(module_names::CLIENT).unwrap();
        let handle = unsafe { dlopen(module.as_ptr(), RTLD_NOLOAD | RTLD_NOW) };
        if !handle.is_null() {
            break;
        }
        if !logged {
            info!("Waiting for tf2 to load");
            logged = true;
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
    if logged {
        info!("tf2 loaded");
    }
}

pub fn main() -> OxidusResult {
    if cfg!(feature = "dump-netvars") {
        wait_for_client();
        info!("Dumping netvars");
        let create_interface: extern "C" fn(*const i8, *const isize) -> *const () = unsafe {
            std::mem::transmute(util::resolve_fn(module_names::CLIENT, "CreateInterface").unwrap())
        };
        let base_client: *mut BaseClient =
            create_interface(c"VClient017".as_ptr(), std::ptr::null()) as *mut BaseClient;
        dump_netvars(base_client)?;
        return Ok(());
    }

    wait_for_client();
    init_overlay()?;
    init_modules();

    Ok(())
}

#[allow(clippy::missing_panics_doc)]
pub fn cleanup() -> OxidusResult {
    let mut hooks = HOOKS.lock().unwrap();
    for hook in hooks.iter_mut() {
        let mut hook = hook.write().unwrap();
        if let Err(e) = hook.restore() {
            warn!("Failed to restore hook: {:?}", e);
        }
    }
    hooks.clear();

    unload_overlay();
    Ok(())
}

unsafe extern "C" fn load() {
    thread::spawn(|| {
        env_logger::builder()
            .filter(Some("oxidus"), log::LevelFilter::Trace)
            .try_init()
            .unwrap();
        info!("Loading");
        if let Err(e) = main() {
            error!("Failed to load\n{e}");
            oxidus_cleanup();
        } else {
            info!("Loaded");
        }
    });
}

#[allow(unused)]
#[no_mangle]
extern "C" fn oxidus_cleanup() {
    thread::spawn(|| {
        info!("Unloading");
        if let Err(e) = cleanup() {
            error!("Failed to unload \n{e}");
        } else {
            info!("Unloaded");
        }
    });
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

extern "C" fn unload() {
    info!("Fini");
}

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { unload };
