use winapi::um::{
    consoleapi::AllocConsole,
    wincon::{FreeConsole, SetConsoleTitleA},
};

pub fn alloc_console(title: &str) -> bool {
    unsafe {
        let alloc_console = AllocConsole();
        if alloc_console == 0 {
            return false;
        }

        let console_title = SetConsoleTitleA(title.as_ptr() as *const i8);
        console_title != 0
    }
}

pub fn free_console() -> bool {
    unsafe {
        let free_console = FreeConsole();
        free_console != 0
    }
}
