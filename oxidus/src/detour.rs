use libc::{mprotect, PROT_EXEC, PROT_READ, PROT_WRITE};
use std::mem;
use std::ptr;

const PATCH_SIZE: usize = 14;
const GATEWAY_SIZE: usize = PATCH_SIZE * 2;
const JUMP_INSTRUCTION: [u8; 6] = [0xff, 0x25, 0x00, 0x00, 0x00, 0x00];

use crate::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct DetourHook {
    original_fn: *mut (),
    gateway: *mut (),
    original_bytes: [u8; 14],
}

impl DetourHook {
    pub unsafe fn new(target_fn: *mut (), hook_fn: *mut ()) -> OxidusResult<Self> {
        if target_fn.is_null() {
            return OxidusErrorType::Hooking("target_fn is null".to_owned()).into();
        }
        if hook_fn.is_null() {
            return OxidusErrorType::Hooking("hook_fn is null".to_owned()).into();
        }

        let original_bytes = ptr::read(target_fn as *const [u8; PATCH_SIZE]);

        let gateway = Self::create_gateway(target_fn, &original_bytes)?;

        Self::install_hook(target_fn, hook_fn)?;

        Ok(Self {
            original_fn: target_fn,
            gateway,
            original_bytes,
        })
    }

    unsafe fn create_gateway(
        target_fn: *mut (),
        original_bytes: &[u8; PATCH_SIZE],
    ) -> OxidusResult<*mut ()> {
        let original_gateway = libc::mmap(
            ptr::null_mut(),
            GATEWAY_SIZE,
            PROT_READ | PROT_WRITE | PROT_EXEC,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0,
        )
        .cast::<()>();

        if original_gateway.is_null() {
            return OxidusErrorType::Hooking("Failed to allocate gateway memory".to_owned()).into();
        }

        ptr::copy_nonoverlapping(
            original_bytes.as_ptr(),
            original_gateway.cast::<u8>(),
            PATCH_SIZE,
        );

        let jump_to_target = Self::create_jump((target_fn as usize + PATCH_SIZE) as *mut ());
        ptr::copy_nonoverlapping(
            jump_to_target.as_ptr(),
            (original_gateway as usize + PATCH_SIZE) as *mut u8,
            PATCH_SIZE,
        );
        dbg!(original_gateway.cast::<[u8; GATEWAY_SIZE]>().read());

        Ok(original_gateway)
    }

    unsafe fn install_hook(target_fn: *mut (), hook_fn: *mut ()) -> OxidusResult {
        debug!("Installing hook {:?}", target_fn);
        #[allow(clippy::cast_sign_loss)]
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;

        let aligned_addr = (target_fn as usize) & !(page_size - 1);
        if mprotect(
            aligned_addr as *mut libc::c_void,
            page_size,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        ) != 0
        {
            return OxidusErrorType::Hooking("Cannot alter memory protection".to_owned()).into();
        }

        let jump = Self::create_jump(hook_fn);
        ptr::copy_nonoverlapping(jump.as_ptr(), target_fn.cast::<u8>(), PATCH_SIZE);

        mprotect(
            aligned_addr as *mut libc::c_void,
            page_size,
            PROT_READ | PROT_EXEC,
        );

        Ok(())
    }

    fn create_jump(target: *mut ()) -> [u8; PATCH_SIZE] {
        let addr = target as usize;
        [
            JUMP_INSTRUCTION[0],
            JUMP_INSTRUCTION[1],
            JUMP_INSTRUCTION[2],
            JUMP_INSTRUCTION[3],
            JUMP_INSTRUCTION[4],
            JUMP_INSTRUCTION[5],
            (addr & 0xFF) as u8,
            ((addr >> 0x8) & 0xFF) as u8,
            ((addr >> 0x10) & 0xFF) as u8,
            ((addr >> 0x18) & 0xFF) as u8,
            ((addr >> 0x20) & 0xFF) as u8,
            ((addr >> 0x28) & 0xFF) as u8,
            ((addr >> 0x30) & 0xFF) as u8,
            ((addr >> 0x38) & 0xFF) as u8,
        ]
    }

    pub unsafe fn call_original<T>(&self, args: T) -> T {
        let original_fn: fn(T) -> T = mem::transmute(self.gateway);
        original_fn(args)
    }

    pub unsafe fn restore(&self) {
        debug!("Restoring hook {:?}", self.original_fn);
        #[allow(clippy::cast_sign_loss)]
        let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        let aligned_addr = (self.original_fn as usize) & !(page_size - 1);
        mprotect(
            aligned_addr as *mut libc::c_void,
            page_size,
            PROT_READ | PROT_WRITE | PROT_EXEC,
        );

        ptr::copy_nonoverlapping(
            self.original_bytes.as_ptr(),
            self.original_fn.cast::<u8>(),
            PATCH_SIZE,
        );

        mprotect(
            aligned_addr as *mut libc::c_void,
            page_size,
            PROT_READ | PROT_EXEC,
        );
    }
}

impl Drop for DetourHook {
    fn drop(&mut self) {
        unsafe {
            //self.restore();
        }
    }
}
