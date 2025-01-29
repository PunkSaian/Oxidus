use std::thread;

use prelude::*;

#[macro_use]
extern crate log;

mod error;
mod prelude;

pub fn main() -> OxidusResult{
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
    });
}

extern "C" fn unload() {
    info!("Unloading");
    eprintln!("oxidus: after unload");
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { unload };
