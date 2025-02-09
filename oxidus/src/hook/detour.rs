use libc::{mprotect, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::{pin::Pin, ptr, sync::RwLock};

const NOP: u8 = 0x90;

const MOVABS_R10: [u8; 2] = [0x49, 0xBA];
const MOVABS_R10_SIZE: usize = MOVABS_R10.len() + 8;

const JMP_INSTRUCTION: [u8; 6] = [0xFF, 0x25, 0x00, 0x00, 0x00, 0x00];

const JMP_SIZE: usize = JMP_INSTRUCTION.len() + 8;

const PATCH_SIZE: usize = JMP_SIZE;

use crate::{prelude::*, util::resolve_fn, HOOKS};

#[allow(clippy::module_name_repetitions)]
pub struct DetourHook {
    pub target_fn: *mut (),
    pub proxy: Option<*mut ()>,
    original_bytes: [u8; PATCH_SIZE],
    pub hooked: bool,
}

unsafe impl Sync for DetourHook {}
unsafe impl Send for DetourHook {}

pub type WrappedDetourHook = Pin<Box<RwLock<DetourHook>>>;

#[allow(unused)]
impl DetourHook {
    pub fn new(target_fn: *mut (), hook_fn: *mut ()) -> OxidusResult<WrappedDetourHook> {
        if target_fn.is_null() || hook_fn.is_null() {
            return Err(OxidusError::Hooking("Null function pointer".to_owned()));
        }

        let original_bytes = unsafe { ptr::read(target_fn as *const [u8; PATCH_SIZE]) };

        let mut hook = Box::pin(RwLock::new(Self {
            target_fn,
            proxy: None,
            original_bytes,
            hooked: false,
        }));

        let proxy = Self::create_proxy(&hook, hook_fn)?;

        let mut hook_locked = hook.write().unwrap();
        hook_locked.remove_protection();
        hook_locked.proxy = Some(proxy);
        drop(hook_locked);

        Ok(hook)
    }

    pub fn new_and_install(
        target_fn: *mut (),
        hook_fn: *mut (),
    ) -> OxidusResult<WrappedDetourHook> {
        let mut hook = Self::new(target_fn, hook_fn)?;
        hook.write().unwrap().install()?;
        Ok(hook)
    }

    fn create_proxy(hook: *const WrappedDetourHook, hook_fn: *mut ()) -> OxidusResult<*mut ()> {
        unsafe {
            let proxy_size = MOVABS_R10_SIZE + JMP_SIZE;
            let proxy = libc::mmap(
                ptr::null_mut(),
                proxy_size,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                libc::MAP_ANON | libc::MAP_PRIVATE,
                -1,
                0,
            )
            .cast::<u8>();

            if proxy.is_null() {
                return Err(OxidusError::Hooking("Proxy allocation failed".to_owned()));
            }

            let mut mov_instr = [NOP; MOVABS_R10_SIZE];
            mov_instr[..2].copy_from_slice(&MOVABS_R10);

            let hook_lock = &*hook; // Dereference the raw pointer to access the Pin<Box<RwLock<...>>>
            let rwlock_ptr = &**hook_lock as *const RwLock<DetourHook>;
            let hook_lock_ptr = rwlock_ptr as usize;

            mov_instr[2..].copy_from_slice(&hook_lock_ptr.to_ne_bytes());

            ptr::copy_nonoverlapping(mov_instr.as_ptr(), proxy, mov_instr.len());

            let jmp_instr = Self::create_jump(hook_fn);
            ptr::copy_nonoverlapping(
                jmp_instr.as_ptr(),
                proxy.add(MOVABS_R10_SIZE),
                jmp_instr.len(),
            );

            Ok(proxy.cast::<()>())
        }
    }

    pub fn remove_protection(&mut self) -> OxidusResult {
        unsafe {
            let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
            let addr = self.target_fn as usize;
            let aligned_addr = addr & !(page_size - 1);
            let protect_size = (addr - aligned_addr) + PATCH_SIZE;

            if mprotect(
                aligned_addr as *mut _,
                protect_size,
                PROT_READ | PROT_WRITE | PROT_EXEC,
            ) != 0
            {
                return Err(OxidusError::Hooking("Memory protection failed".to_owned()));
            }

            Ok(())
        }
    }

    pub fn install(&mut self) -> OxidusResult {
        unsafe {
            if self.hooked {
                return Err(OxidusError::Hooking("Hook already installed".to_owned()));
            }

            let patch = Self::create_patch(self.proxy.unwrap());
            ptr::copy_nonoverlapping(patch.as_ptr(), self.target_fn.cast::<u8>(), PATCH_SIZE);

            self.hooked = true;
            Ok(())
        }
    }

    fn create_jump(target: *mut ()) -> [u8; JMP_SIZE] {
        let mut jump = [0u8; JMP_SIZE];
        jump[0..6].copy_from_slice(&JMP_INSTRUCTION);
        jump[6..].copy_from_slice(&(target as usize).to_ne_bytes());
        jump
    }

    fn create_patch(target: *mut ()) -> [u8; PATCH_SIZE] {
        let mut patch = [NOP; PATCH_SIZE];
        let jump = Self::create_jump(target);
        patch[0..jump.len()].copy_from_slice(&jump);
        patch
    }

    pub fn restore(&mut self) -> OxidusResult {
        if !self.hooked {
            return Err(OxidusError::Hooking("Hook not installed".to_owned()));
        }
        unsafe {
            ptr::copy_nonoverlapping(
                self.original_bytes.as_ptr(),
                self.target_fn.cast::<u8>(),
                PATCH_SIZE,
            );

            self.hooked = false;
        }
        Ok(())
    }
}

impl Drop for DetourHook {
    fn drop(&mut self) {
        if let Err(e) = self.restore() {
            warn!("Hook already restored when dropping: {e}");
        };

        if let Some(proxy_ptr) = self.proxy {
            let proxy_size = MOVABS_R10_SIZE + JMP_SIZE;
            let result = unsafe { libc::munmap(proxy_ptr.cast(), proxy_size) };
            if result != 0 {
                let errno = std::io::Error::last_os_error();
                warn!("Failed to unmap proxy memory: {}", errno);
            }
        }
    }
}

pub fn install_detour(target_fn: *mut (), hook_fn: *mut ()) -> OxidusResult {
    let hook = DetourHook::new_and_install(target_fn, hook_fn)?;
    HOOKS.lock().unwrap().push(hook);
    Ok(())
}
pub fn install_detour_from_symbol(module: &str, symbol: &str, hook_fn: *mut ()) -> OxidusResult {
    let Some(target_fn) = resolve_fn(module, symbol) else {
        return Err(OxidusError::Hooking(format!(
            "Failed to resolve symbol {symbol} in {module}"
        )));
    };
    install_detour(target_fn, hook_fn)?;
    Ok(())
}
