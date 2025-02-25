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
#![feature(inherent_associated_types, generic_const_exprs, slice_as_array)]

extern crate thiserror;

use std::ffi::CString;
use std::ptr;
use std::thread;

use hook::restore_hooks;
use libc::{dlopen, RTLD_NOLOAD, RTLD_NOW};
use log::{error, info};

use modules::init_modules;
use netvar_dumper::dump_netvars;
use overlay::init as init_overlay;
use overlay::unload as unload_overlay;
use prelude::*;
use sdk::interface::client::Client;
use sdk::interface::client_mode::ClientMode;
use sdk::interface::interface_names;
use sdk::interface::interfaces::Interfaces;
use sdk::module_names;
use config::init_settings;
use config::Config;
use util::create_interface;

#[macro_use]
extern crate log;

mod hook;
mod math;

mod hooks;
mod modules;
mod config;

mod netvar_dumper;
mod overlay;
#[allow(unused)]
mod prelude;
mod sdk;
mod util;

pub fn wait_for_load() {
    let mut logged = false;
    loop {
        let module = CString::new(module_names::CLIENT).unwrap();
        let handle = unsafe { dlopen(module.as_ptr(), RTLD_NOLOAD | RTLD_NOW) };
        if !handle.is_null() {
            let client =
                create_interface::<Client>(module_names::CLIENT, interface_names::CLIENT).unwrap();
            loop {
                #[allow(useless_ptr_null_checks)]
                if ptr::from_ref::<ClientMode>(Interfaces::get_client_mode(client)).is_null() {
                    thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                }
                break;
            }
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
    init_settings();

    wait_for_load();
    if cfg!(feature = "dump-netvars") {
        info!("Dumping netvars");
        dump_netvars()?;
        return Ok(());
    }
    Interfaces::init();

    init_overlay()?;

    info!("Initializing modules");
    init_modules();

    info!("Initializing hooks");
    hooks::init();

    Ok(())
}

#[allow(clippy::missing_panics_doc)]
pub fn cleanup() -> OxidusResult {
    restore_hooks();

    unload_overlay();

    let config = Config::get();
    config.save().unwrap();

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
