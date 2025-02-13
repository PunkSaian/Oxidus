use crate::prelude::*;
use libc::{self, mprotect, PROT_READ, PROT_WRITE};
use std::collections::HashMap;
use std::marker::{Send, Sync};
use std::sync::{Mutex, OnceLock, RwLock};

// Newtype wrapper to safely implement Send/Sync for function pointers
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FnPtr(*mut ());

unsafe impl Send for FnPtr {}
unsafe impl Sync for FnPtr {}

pub static VMT_HOOK_REGISTRY: OnceLock<RwLock<HashMap<FnPtr, FnPtr>>> = OnceLock::new();

static VMT_HOOKS: Mutex<Vec<VmtHook>> = const { Mutex::new(Vec::new()) };

fn get_registry() -> &'static RwLock<HashMap<FnPtr, FnPtr>> {
    VMT_HOOK_REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

pub struct VmtHook {
    vmt: *mut *mut (),
    index: usize,
    original: FnPtr,
    hook_fn: FnPtr,
}

// Implement Send/Sync manually since we're managing the pointers safely
unsafe impl Send for VmtHook {}
unsafe impl Sync for VmtHook {}

impl VmtHook {
    pub fn new(vmt: *mut *mut (), index: usize, hook_fn: *mut ()) -> OxidusResult<Self> {
        unsafe {
            let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
            let vmt_ptr = vmt as usize;

            let aligned_addr = vmt_ptr & !(page_size - 1);
            let protect_size =
                (vmt_ptr - aligned_addr) + std::mem::size_of::<*mut ()>() * (index + 1);

            if mprotect(aligned_addr as *mut _, protect_size, PROT_READ | PROT_WRITE) != 0 {
                return Err(OxidusError::Hooking(
                    "Failed to change memory protection".into(),
                ));
            }

            let original = FnPtr(*vmt.add(index));
            let hook_fn = FnPtr(hook_fn);
            *vmt.add(index) = hook_fn.0;

            get_registry()
                .write()
                .map_err(|_| OxidusError::Hooking("Failed to acquire registry lock".into()))?
                .insert(hook_fn, original);

            Ok(Self {
                vmt,
                index,
                original,
                hook_fn,
            })
        }
    }

    pub fn restore(&mut self) -> OxidusResult {
        unsafe {
            *self.vmt.add(self.index) = self.original.0;
            get_registry()
                .write()
                .map_err(|_| OxidusError::Hooking("Failed to acquire registry lock".into()))?
                .remove(&self.hook_fn);

            Ok(())
        }
    }
}

impl Drop for VmtHook {
    fn drop(&mut self) {
        if let Err(e) = self.restore() {
            error!("Failed to restore VMT hook: {}", e);
        }
    }
}

pub fn get_original(hook_fn: *mut ()) -> OxidusResult<*mut ()> {
    let hook_fn = FnPtr(hook_fn);
    get_registry()
        .read()
        .map_err(|_| OxidusError::Hooking("Failed to acquire registry lock".into()))?
        .get(&hook_fn)
        .map(|f| f.0)
        .ok_or_else(|| OxidusError::Hooking("Original function not found".into()))
}

pub fn install_vmt(vmt: *mut *mut (), index: usize, hook_fn: *mut ()) {
    VMT_HOOKS
        .lock()
        .unwrap()
        .push(VmtHook::new(vmt, index, hook_fn).unwrap());
}

pub fn restore_vmt_hooks() {
    VMT_HOOKS.lock().unwrap().clear();
}
