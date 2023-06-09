#![feature(naked_functions)]
#![feature(c_variadic)]
use core::ffi::c_void;
use hack::Hack;
use winapi::{
    shared::minwindef::{BOOL, HINSTANCE, TRUE},
    um::{
        libloaderapi::DisableThreadLibraryCalls,
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

#[cfg(feature = "logging")]
mod logging;

mod config;
mod d2;
mod hack;
mod memory;
mod modules;
mod utils;

#[no_mangle]
extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    let mut hack = Hack::new(hmodule);
    match reason {
        DLL_PROCESS_ATTACH => {
            unsafe { DisableThreadLibraryCalls(hmodule) };
            hack.attach();
        }
        DLL_PROCESS_DETACH => {
            hack.detach();
        }
        _ => {}
    };

    TRUE
}
