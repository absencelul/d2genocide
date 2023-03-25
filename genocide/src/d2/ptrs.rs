use std::mem::transmute;

use widestring::WideCString;

pub fn print_game_string(message: &str, color: i32) {
    type PrintGameStringFn = extern "fastcall" fn(*const u16, i32);
    unsafe {
        let wide_str = WideCString::from_str(message).unwrap();
        let wide_str = wide_str.into_raw();
        transmute::<usize, PrintGameStringFn>(0x49E3A0)(wide_str, color);
        let _ = WideCString::from_raw(wide_str);
    }
}
