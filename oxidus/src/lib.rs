#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::multiple_crate_versions,
    clippy::cast_precision_loss,
    clippy::module_name_repetitions,
    clippy::cast_possible_wrap,
    clippy::cargo_common_metadata,
    clippy::cast_sign_loss
)]

extern crate thiserror;

use std::sync::atomic::AtomicBool;
use std::{sync::Mutex, thread};

use hook::detour::WrappedDetourHook;
use log::{error, info};
use overlay::init as init_overlay;
use overlay::unload as unload_overlay;
use prelude::*;

#[macro_use]
extern crate log;

mod hook;
mod overlay;
mod prelude;
mod sdk;
mod util;

#[allow(clippy::type_complexity)]
static HOOKS: Mutex<Vec<WrappedDetourHook>> = const { Mutex::new(Vec::new()) };
static UNLOADING: AtomicBool = const { AtomicBool::new(false) };

pub fn main() -> OxidusResult {
    init_overlay()?;
    Ok(())
}

#[allow(clippy::missing_panics_doc)]
pub fn cleanup() -> OxidusResult {
    let mut hooks = HOOKS.lock().unwrap();
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
        if UNLOADING.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }

        UNLOADING.store(true, std::sync::atomic::Ordering::SeqCst);
        info!("Unloading");
        info!("Cleanup started");
        if let Err(e) = cleanup() {
            error!("Failed to cleanup\n{e}");
        } else {
            info!("Cleanup finished");
        }
    });
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

extern "C" fn unload() {
    info!("Unloaded");
}

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { unload };
