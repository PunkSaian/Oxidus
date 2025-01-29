use std::arch::asm;

#[macro_use]
extern crate log;

unsafe extern "C" fn load() {
    eprintln!("oxidus: before load");
    libc::atexit(unload);

    env_logger::builder()
        .filter(Some("oxidus"), log::LevelFilter::Trace)
        .try_init()
        .unwrap();

    info!("Loading");
}

///# Safety
/// very fucking unsafe
pub unsafe fn print_all_keys() {
    for i in 0..libc::KEY_MAX {
        let info = libc::pthread_getspecific(i.into());
        if !info.is_null() {
            dbg!(i, info);
        }
    }
}

///# Safety
/// very fucking unsafe
#[no_mangle]
#[allow(unused)]
pub unsafe extern "C" fn self_unload() {
    asm!("int3");
}

extern "C" fn unload() {
    eprintln!("oxidus: unloaded");
}

#[link_section = ".init_array"]
#[allow(unused)]
static LOAD: unsafe extern "C" fn() = { load };

#[link_section = ".fini_array"]
#[allow(unused)]
static UNLOAD: unsafe extern "C" fn() = { unload };
