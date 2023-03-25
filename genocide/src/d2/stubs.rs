// use log::info;
use std::arch::asm;

use crate::d2::draw::{on_draw_automap, on_draw_interface};

// pub extern "C" fn on_game_loop() {
//     info!("Game loop");
// }

/// void declspec(naked) OnGameLoop()
/// not why this is crashing after the first call
/// maybe am wrong?
// #[naked]
// pub extern "C" fn game_loop_hook() {
//     unsafe {
//         asm!(
//             "pushad",
//             "call {on_game_loop}",
//             "popad",
//             "push 0x44C990",
//             "ret",
//             on_game_loop = sym on_game_loop,
//             options(noreturn)
//         );
//     }
// }

#[naked]
pub extern "C" fn draw_automap_hook() {
    unsafe {
        asm!(
            "pushad",
            "call {on_draw_automap}",
            "popad",
            "pop edi",
            "pop esi",
            "ret",
            on_draw_automap = sym on_draw_automap,
            options(noreturn)
        );
    }
}

#[naked]
pub extern "C" fn draw_interface_hook() {
    unsafe {
        asm!(
            "call {on_draw_interface}",
            "pop ebx",
            "mov esp, ebp",
            "pop ebp",
            "ret",
            on_draw_interface = sym on_draw_interface,
            options(noreturn),
        )
    }
}
