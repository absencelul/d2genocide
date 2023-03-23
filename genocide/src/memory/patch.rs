use log::debug;
use winapi::um::{
    libloaderapi::GetModuleHandleA,
    memoryapi::{ReadProcessMemory, VirtualProtect},
    processthreadsapi::GetCurrentProcess,
    winnt::PAGE_EXECUTE_READWRITE,
};

#[derive(Debug)]
#[allow(unused)]
pub enum PatchType {
    Call,
    Jump,
    Fill,
}

#[derive(Debug)]
pub struct Patch {
    patch_type: PatchType,
    offset: usize,
    length: usize,
    function: i32,

    old_bytes: Vec<u8>,
    injected: bool,
}

// Patch::new(PatchType::Call, 0xDEADBEEF, 6, game_loop_hook)

impl Patch {
    pub fn new(patch_type: PatchType, offset: usize, length: usize, function: i32) -> Self {
        Self {
            patch_type,
            offset,
            length,
            function,
            old_bytes: vec![0; length],
            injected: false,
        }
    }

    fn get_address(&self) -> usize {
        unsafe {
            let dll = GetModuleHandleA(0 as *const i8);
            if dll.is_null() {
                debug!("GetModuleHandleA failed");
                return 0;
            }

            (dll as usize) + self.offset
        }
    }

    pub fn inject(&mut self) {
        if self.injected {
            debug!("Patch already injected");
            return;
        }

        let address = self.get_address();
        if address == 0 {
            debug!("Failed to get address");
            return;
        }

        unsafe {
            ReadProcessMemory(
                GetCurrentProcess(),
                address as *const _,
                self.old_bytes.as_mut_ptr() as *mut _,
                self.length,
                std::ptr::null_mut(),
            );

            let mut new_bytes = vec![0u8; self.length];
            let mut protect = 0;

            match self.patch_type {
                PatchType::Fill => {
                    new_bytes = vec![self.function as u8; self.length];
                }
                PatchType::Call => {
                    new_bytes[0] = 0xE8;
                    let relative_address = self.function - (address as i32 + 5);
                    new_bytes[1..5].copy_from_slice(&relative_address.to_le_bytes());
                }
                PatchType::Jump => {
                    new_bytes[0] = 0xE9;
                    let relative_address = self.function - (address as i32 + 5);
                    new_bytes[1..5].copy_from_slice(&relative_address.to_le_bytes());
                }
            }

            VirtualProtect(
                address as *mut _,
                self.length,
                PAGE_EXECUTE_READWRITE,
                &mut protect,
            );

            std::ptr::copy_nonoverlapping(new_bytes.as_ptr(), address as *mut _, self.length);

            VirtualProtect(address as *mut _, self.length, protect, &mut protect);
        }

        self.injected = true;
    }

    pub fn eject(&mut self) {
        if !self.injected {
            debug!("Patch not injected");
            return;
        }

        let address = self.get_address();
        if address == 0 {
            debug!("Failed to get address");
            return;
        }

        unsafe {
            let mut protect = 0;

            VirtualProtect(
                address as *mut _,
                self.length,
                PAGE_EXECUTE_READWRITE,
                &mut protect,
            );

            std::ptr::copy_nonoverlapping(self.old_bytes.as_ptr(), address as *mut _, self.length);

            VirtualProtect(address as *mut _, self.length, protect, &mut protect);
        }
    }
}
