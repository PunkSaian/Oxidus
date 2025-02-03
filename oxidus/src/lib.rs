#![allow(
    clippy::missing_errors_doc,
    clippy::cast_possible_truncation,
    clippy::multiple_crate_versions
)]

extern crate thiserror;

use std::thread;

use detour::DetourHook;
use prelude::*;
use util::resolve_fn;

#[macro_use]
extern crate log;

mod detour;
mod error;
mod prelude;
mod sdk;
mod util;

unsafe extern "C" fn original_hokable() {
    debug!("original hookable");
}

unsafe extern "C" fn my_hook_function() {
    debug!("Hooked function called!");
}

pub fn main() -> OxidusResult {
    // Initialize overlay
    unsafe {
        //let target_fn = resolve_fn("/usr/lib/libvulkan.so.1", "vkQueuePresentKHR").expect("test");
        let hook_fn = my_hook_function as *mut ();
        let target_fn = original_hokable as *mut ();

        let hook = DetourHook::new(target_fn, hook_fn)?;

        original_hokable();
        original_hokable();
        original_hokable();
        original_hokable();
        hook.call_original(());


        //hook.restore();
    }
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
            //unsafe {
            //    let handle = dlopen("/tmp/liboxidus.so".as_ptr().cast::<i8>(), 6);
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
