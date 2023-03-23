use log::info;
use winapi::shared::minwindef::{BOOL, HINSTANCE, TRUE};

use crate::{
    d2::hooks,
    memory::patch::{Patch, PatchType},
    utils::console,
};

#[allow(dead_code)]
pub struct Hack {
    patches: Vec<Patch>,
    hmodule: HINSTANCE,
}

impl Hack {
    pub fn new(hmodule: HINSTANCE) -> Self {
        Self {
            patches: vec![Patch::new(
                PatchType::Call,
                0x4F28B,
                6,
                hooks::game_loop_hook as i32,
            )],
            hmodule,
        }
    }

    fn install_patches(&mut self) {
        for patch in self.patches.iter_mut() {
            patch.inject();
            info!("Installed patch: {:?}", patch);
        }
    }

    fn eject_patches(&mut self) {
        for patch in self.patches.iter_mut() {
            patch.eject();
            info!("Ejected patch: {:?}", patch);
        }
    }

    pub fn attach(&mut self) -> BOOL {
        #[cfg(feature = "logging")]
        crate::logging::init();

        console::alloc_console("genocide");

        self.install_patches();

        TRUE
    }

    pub fn detach(&mut self) -> BOOL {
        self.eject_patches();

        console::free_console();

        TRUE
    }
}
