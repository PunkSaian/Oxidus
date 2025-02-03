use libc::{mprotect, munmap, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::{os::raw::c_void, ptr};

const NOP: u8 = 0x90;

const MOVABS_R10: [u8; 2] = [0x49, 0xBA];
const MOVABS_R10_SIZE: usize = MOVABS_R10.len() + 8;

const JMP_INSTRUCTION: [u8; 6] = [0xFF, 0x25, 0x00, 0x00, 0x00, 0x00];

const JMP_SIZE: usize = JMP_INSTRUCTION.len() + 8;

const PATCH_SIZE: usize = JMP_SIZE;
const PROXY_SIZE: usize = MOVABS_R10_SIZE + JMP_SIZE;

const GATEWAY_SIZE: usize = PATCH_SIZE + JMP_SIZE;

use crate::prelude::*;

/// # Example
///```rust
///unsafe extern "C" fn original_hokable(arg1: u8, arg2: u32) -> i32 {
///    debug!("original hookable, {}, {}", arg1, arg2);
///    0
///}
///
///#[detour_hook]
///unsafe extern "C" fn my_hook_function(arg1: u8, arg2: u32) -> i32 {
///    println!("Intercepted args: {arg1}, {arg2}");
///    original_function(12, arg2)
///}
///pub fn main() -> OxidusResult {
///    
///    unsafe {
///        let hook_fn = my_hook_function as *mut ();
///        let target_fn = original_hokable as *mut ();
///
///        let mut hook = DetourHook::new_and_install(target_fn, hook_fn)?;
///
///        original_hokable(1, 2);
///
///        hook.restore();
///        original_hokable(1, 2);
///    }
///    Ok(())
///}
///```
#[allow(clippy::module_name_repetitions)]
pub struct DetourHook {
    target_fn: *mut (),
    gateway: *mut (),
    proxy: *mut (),
    original_bytes: [u8; PATCH_SIZE],
}

unsafe impl Sync for DetourHook {}
unsafe impl Send for DetourHook {}

#[allow(unused)]
impl DetourHook {
    pub unsafe fn new(target_fn: *mut (), hook_fn: *mut ()) -> OxidusResult<Self> {
        if target_fn.is_null() || hook_fn.is_null() {
            return OxidusErrorType::Hooking("Null function pointer".to_owned()).into();
        }

        let original_bytes = ptr::read(target_fn as *const [u8; PATCH_SIZE]);

        let gateway = Self::create_gateway(target_fn, &original_bytes)?;
        let proxy = Self::create_proxy(gateway, hook_fn)?;
        Ok(Self {
            target_fn,
            gateway,
            proxy,
            original_bytes,
        })
    }

    pub unsafe fn new_and_install(target_fn: *mut (), hook_fn: *mut ()) -> OxidusResult<Self> {
        let mut hook = Self::new(target_fn, hook_fn)?;
        hook.install_hook()?;
        Ok(hook)
    }

    unsafe fn create_proxy(gateway: *mut (), hook_fn: *mut ()) -> OxidusResult<*mut ()> {
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
            return OxidusErrorType::Hooking("Proxy allocation failed".to_owned()).into();
        }

        let mut mov_instr = [NOP; MOVABS_R10_SIZE];
        mov_instr[..2].copy_from_slice(&MOVABS_R10);

        mov_instr[2..].copy_from_slice(&(gateway as usize).to_ne_bytes());

        ptr::copy_nonoverlapping(mov_instr.as_ptr(), proxy, mov_instr.len());

        let jmp_instr = Self::create_jump(hook_fn);
        ptr::copy_nonoverlapping(
            jmp_instr.as_ptr(),
            proxy.add(MOVABS_R10_SIZE),
            jmp_instr.len(),
        );

        Ok(proxy.cast::<()>())
    }

    unsafe fn create_gateway(
        target_fn: *mut (),
        original_bytes: &[u8; PATCH_SIZE],
    ) -> OxidusResult<*mut ()> {
        let gateway = libc::mmap(
            ptr::null_mut(),
            GATEWAY_SIZE,
            PROT_READ | PROT_WRITE | PROT_EXEC,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0,
        )
        .cast::<()>();

        if gateway.is_null() {
            return OxidusErrorType::Hooking("Gateway allocation failed".to_owned()).into();
        }

        ptr::copy_nonoverlapping(original_bytes.as_ptr(), gateway.cast::<u8>(), PATCH_SIZE);

        let jump_back = Self::create_jump((target_fn as usize + PATCH_SIZE) as *mut ());
        ptr::copy_nonoverlapping(
            jump_back.as_ptr(),
            gateway.cast::<u8>().add(PATCH_SIZE),
            JMP_SIZE,
        );

        Ok(gateway.cast())
    }

    unsafe fn install_hook(&mut self) -> OxidusResult {
        info!("Installing hook at {:p}", self.target_fn);
        #[allow(clippy::cast_sign_loss)]
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
            return OxidusErrorType::Hooking("Memory protection failed".to_owned()).into();
        }

        let patch = Self::create_patch(self.proxy);
        ptr::copy_nonoverlapping(patch.as_ptr(), self.target_fn.cast::<u8>(), PATCH_SIZE);

        mprotect(aligned_addr as *mut _, protect_size, PROT_READ | PROT_EXEC);

        Ok(())
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

    pub unsafe fn restore(&mut self) {
        info!("Restoring original bytes at {:p}", self.target_fn);

        #[allow(clippy::cast_sign_loss)]
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let addr = self.target_fn as usize;
        let aligned_addr = addr & !(page_size - 1);
        let protect_size = (addr - aligned_addr) + PATCH_SIZE;

        mprotect(
            aligned_addr as *mut _,
            protect_size,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        );

        ptr::copy_nonoverlapping(
            self.original_bytes.as_ptr(),
            self.target_fn.cast::<u8>(),
            PATCH_SIZE,
        );

        mprotect(aligned_addr as *mut _, protect_size, PROT_READ | PROT_EXEC);
    }
}

impl Drop for DetourHook {
    fn drop(&mut self) {
        unsafe {
            self.restore();

            if !self.gateway.is_null() {
                munmap(self.gateway.cast::<c_void>(), GATEWAY_SIZE);
            }
            if !self.proxy.is_null() {
                munmap(self.proxy.cast::<c_void>(), PROXY_SIZE);
            }
        }
    }
}
