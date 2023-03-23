#![feature(asm_const)]
#![feature(naked_functions)]
use core::ffi::c_void;
use hack::Hack;
use winapi::{
    shared::minwindef::{BOOL, HINSTANCE, TRUE},
    um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

#[cfg(feature = "logging")]
mod logging;

mod d2;
mod hack;
mod memory;
mod utils;

#[no_mangle]
extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    let mut hack = Hack::new(hmodule);
    match reason {
        DLL_PROCESS_ATTACH => {
            hack.attach();
        }
        DLL_PROCESS_DETACH => {
            hack.detach();
        }
        _ => {}
    };

    TRUE
}
