#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::multiple_crate_versions,
    clippy::cast_precision_loss
)]

extern crate thiserror;

use std::{sync::Mutex, thread};

use hook::detour::WrappedDetourHook;
use log::{error, info};
use once_cell::sync::Lazy;
//use overlay::unload as unload_overlay;
//use overlay::{init as init_overlay, IMGUI_STATE};
use prelude::*;

#[macro_use]
extern crate log;

mod hook;
//mod overlay;
mod prelude;
mod sdk;
mod util;

//#[allow(clippy::type_complexity)]
//static HOOKS: Lazy<Mutex<Vec<WrappedDetourHook>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn main() -> OxidusResult {
    //init_overlay()?;
    Ok(())
}

///
/// # Panics
/// panics if hooks in use
pub fn cleanup() -> OxidusResult {
    //let mut hooks = HOOKS.lock().unwrap();
    //for hook in hooks.iter() {
    //    dbg!("locking");
    //    if let Err(e) = hook.write().unwrap().restore() {
    //        warn!("Hook already resotred when dropping: {e}");
    //    };
    //    dbg!("locked");
    //}
    //hooks.clear();
    //FIXME: figure out htis locking issue
    //sleep(Duration::from_secs(1));
    //IMGUI_STATE.with(|state| {
    //    let state = state.read().unwrap();
    //    dbg!(state);
    //});
    //unload_overlay();
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

/// cleanup function to remove all thread_local storage instances and restore all hooks
#[allow(unused)]
#[no_mangle]
extern "C" fn oxidus_cleanup() {
    info!("Unloading");
    if let Err(e) = cleanup() {
        error!("Failed to unload\n{e}");
    } else {
        info!("Unloaded");
    }
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

extern "C" fn fini() {
    info!("fini");
}

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { fini };
