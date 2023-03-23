use log::info;
use std::arch::asm;

pub fn on_game_loop() {
    info!("Game loop");
}

// void declspec(naked) OnGameLoop()
// not sure if this is the correct way to do this
// it's crashing after the first call
#[naked]
pub extern "C" fn game_loop_hook() {
    unsafe {
        asm!(
            "pushad",
            "call {on_game_loop}",
            "popad",
            "push 0x44C990",
            "ret",
            on_game_loop = sym on_game_loop,
            options(noreturn)
        );
    }
}
