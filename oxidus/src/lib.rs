#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::multiple_crate_versions,
    clippy::cast_precision_loss
)]

extern crate thiserror;

use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    thread,
};

use hook::detour::DetourHook;
use log::{error, info};
use once_cell::sync::Lazy;
use overlay::init as init_overlay;
use prelude::*;

#[macro_use]
extern crate log;

mod hook;
mod overlay;
mod prelude;
mod sdk;
mod util;

#[allow(clippy::type_complexity)]
static HOOKS: Lazy<Arc<Mutex<Vec<Pin<Box<DetourHook>>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub fn main() -> OxidusResult {
    init_overlay()?;
    Ok(())
}

unsafe extern "C" fn load() {
    thread::spawn(|| {
        eprintln!("oxidus: before load");
        env_logger::builder()
            .filter(Some("oxidus"), log::LevelFilter::Trace)
            .try_init()
            .unwrap();
        info!("Loading");
        if let Err(e) = main() {
            error!("Failed to load\n{e}");
            //TODO: propper unloading
            //unsafe {
            //    let handle = dlopen("/tmp/liboxidus.so".as_ptr().cast::<i8>(), 6);
            //    dlclose(handle);
            //    dlclose(handle);
            //}
        } else {
            info!("Loaded sucessfully");
        }
    });
}

extern "C" fn unload() {
    info!("Unloading");
    info!("Unloaded");
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { unload };
