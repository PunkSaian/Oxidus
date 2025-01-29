use std::{arch::asm, thread};

#[macro_use]
extern crate log;

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

///# Safety
/// very fucking unsafe
#[no_mangle]
#[allow(unused)]
pub unsafe extern "C" fn self_unload() {
    asm!("int3");
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
