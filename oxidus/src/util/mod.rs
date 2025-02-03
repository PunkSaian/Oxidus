#[allow(deprecated)]
use std::{
    env::home_dir,
    ffi::{CStr, CString},
    fs::{File, OpenOptions},
    io::{Read, Write},
    ptr,
};

use goblin::elf::Elf;
use libc::{dl_iterate_phdr, dl_phdr_info, dlclose, dlopen, dlsym, RTLD_NOLOAD, RTLD_NOW};

#[allow(unused)]
pub unsafe fn resolve_fn(module: &str, name: &str) -> Option<*mut ()> {
    let module = CString::new(module).unwrap();
    let handle = dlopen(module.as_ptr(), RTLD_NOLOAD | RTLD_NOW);
    if handle.is_null() {
        return None;
    }
    let name = CString::new(name).unwrap();
    let res = dlsym(handle, name.as_ptr()).cast::<()>();
    dlclose(handle);
    if res.is_null() {
        return None;
    }
    Some(res)
}

#[allow(unused)]
extern "C" fn callback(info: *mut dl_phdr_info, _size: usize, _data: *mut std::ffi::c_void) -> i32 {
    unsafe {
        let info = &*info;
        let filename = CStr::from_ptr(info.dlpi_name).to_string_lossy();
        #[allow(deprecated)]
        let mut path = home_dir().unwrap();
        path.push("symboldump.txt");
        let mut dump = OpenOptions::new()
            .append(true)
            .create_new(!path.exists())
            .open(path)
            .unwrap();
        if !filename.is_empty() {
            if [
                "/usr/lib/libstdc++.so.6",
                "/usr/lib/libgcc_s.so.1",
                "bin/linux64/launcher.so",
                "bin/linux64/libtier0.so",
                "bin/linux64/libvstdlib.so",
                "bin/linux64/libsteam_api.so",
                "bin/linux64/libtogl.so",
                "/tmp/liboxidus.so",
                "bin/linux64/libmimalloc.so",
            ]
            .contains(&&*filename)
            {
                return 0;
            }
            if let Ok(mut file) = File::open(&*filename) {
                let mut buffer = Vec::new();
                if file.read_to_end(&mut buffer).is_ok() {
                    if let Ok(elf) = Elf::parse(&buffer) {
                        for sym in &elf.syms {
                            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                                if !name.ends_with("@GLIBC_2.2.5") && !name.is_empty() {
                                    dump.write_fmt(format_args!("{filename}/{name}\n")).unwrap();
                                }
                            }
                        }
                        for sym in &elf.dynsyms {
                            if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                                if !name.ends_with("@GLIBC_2.2.5") && !name.is_empty() {
                                    dump.write_fmt(format_args!("{filename}/{name}\n")).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}
#[allow(unused)]
pub fn print_resolvable_names() {
    #[allow(deprecated)]
    let mut path = home_dir().unwrap();
    path.push("symboldump.txt");
    std::fs::remove_file(path).unwrap();
    unsafe {
        dl_iterate_phdr(Some(callback), ptr::null_mut());
    }
}
