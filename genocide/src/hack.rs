use log::info;
use once_cell::sync::Lazy;
use winapi::shared::minwindef::{BOOL, HINSTANCE, TRUE};

use crate::{
    config::{settings::Settings, ui::UiConfig},
    d2::stubs,
    memory::patch::{Patch, PatchType},
    utils::console,
};

pub static CONFIG: Lazy<UiConfig> = Lazy::new(|| {
    let mut config = UiConfig::new("C:\\Users\\zmeye\\Documents\\d2genocide\\config\\ui.json");
    let _ = config.parse();
    config
});

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    let mut settings =
        Settings::new("C:\\Users\\zmeye\\Documents\\d2genocide\\config\\settings.json");
    let _ = settings.parse();
    settings
});

#[allow(dead_code)]
pub struct Hack {
    patches: Vec<Patch>,
    hmodule: HINSTANCE,
}

impl Hack {
    pub fn new(hmodule: HINSTANCE) -> Self {
        Self {
            patches: vec![
                Patch::new(PatchType::Jump, 0x5ADB3, stubs::draw_automap_hook as i32, 5),
                Patch::new(
                    PatchType::Jump,
                    0x572D8,
                    stubs::draw_interface_hook as i32,
                    6,
                ),
            ],
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
