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
#![feature(inherent_associated_types, generic_const_exprs)]

extern crate thiserror;

use std::ffi::CString;
use std::thread;

use hook::restore_hooks;
use libc::{dlopen, RTLD_NOLOAD, RTLD_NOW};
use log::{error, info};

use modules::init_modules;
use netvar_dumper::dump_netvars;
use overlay::init as init_overlay;
use overlay::unload as unload_overlay;
use prelude::*;
use sdk::interface::interfaces::Interfaces;
use sdk::module_names;

#[macro_use]
extern crate log;

mod hook;
mod math;

mod modules;

mod netvar_dumper;
mod overlay;
mod prelude;
mod sdk;
mod util;

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
    wait_for_client();
    if cfg!(feature = "dump-netvars") {
        info!("Dumping netvars");
        dump_netvars()?;
        return Ok(());
    }
    Interfaces::init();

    //let sig: Signature =
    //    sig!("55 48 89 E5 41 57 41 56 41 55 41 54 53 48 83 EC ?? 0f b7 ?? ?? ?? ?? ?? 81 FB FF FF");
    //let res = sig.scan_module(module_names::CLIENT);

    init_overlay()?;

    init_modules();
    Ok(())
}

#[allow(clippy::missing_panics_doc)]
pub fn cleanup() -> OxidusResult {
    restore_hooks();

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
