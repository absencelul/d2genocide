use core::ffi::c_void;
use windows::{
    s,
    Win32::{
        Foundation::{BOOL, HINSTANCE, HWND, TRUE},
        System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        UI::WindowsAndMessaging::MessageBoxA,
    },
};

fn attach() -> BOOL {
    unsafe {
        // do some stuff
        MessageBoxA(
            HWND(0),
            s!("Injected"),
            s!("genocide.dll"),
            Default::default(),
        );
        TRUE
    }
}

fn detach() -> BOOL {
    unsafe {
        // do some stuff
        MessageBoxA(
            HWND(0),
            s!("Detaching"),
            s!("genocide.dll"),
            Default::default(),
        );
        TRUE
    }
}

#[no_mangle]
extern "stdcall" fn DllMain(_hmodule: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => detach(),
        _ => TRUE,
    }
}
