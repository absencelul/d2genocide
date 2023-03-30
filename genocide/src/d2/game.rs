use std::{
    ffi::c_char,
    fmt::{Display, Formatter},
    mem::transmute,
};
use widestring::WideCString;

use super::draw::TextColor;

pub fn exit_game() {
    type ExitGameFn = extern "fastcall" fn();
    unsafe { transmute::<usize, ExitGameFn>(0x44DD60)() };
}

pub fn get_ping() -> Option<u32> {
    let ping: *const u32 = 0x7A04A4 as *const u32;
    if ping.is_null() {
        return None;
    }
    Some(unsafe { *ping })
}

pub fn get_fps() -> Option<u32> {
    let fps: *const u32 = 0x7BB390 as *const u32;
    if fps.is_null() {
        return None;
    }
    Some(unsafe { *fps })
}

pub fn get_skip() -> Option<u32> {
    let skip: *const u32 = 0x7A04B0 as *const u32;
    if skip.is_null() {
        return None;
    }
    Some(unsafe { *skip })
}

#[allow(dead_code)]
pub fn get_screen_size_x() -> Option<u32> {
    let x: *const u32 = 0x71146C as *const u32;
    if x.is_null() {
        return None;
    }
    Some(unsafe { *x })
}

#[allow(dead_code)]
pub fn get_screen_size_y() -> Option<u32> {
    let y: *const u32 = 0x711470 as *const u32;
    if y.is_null() {
        return None;
    }
    Some(unsafe { *y })
}

// pub fn get_level_name(level_id)

fn print_game_string(msg: &str, color: TextColor) {
    type PrintGameStringFn = extern "fastcall" fn(*const u16, i32);
    unsafe {
        let wide_str = WideCString::from_str(msg).unwrap();
        let wide_str = wide_str.into_raw();
        transmute::<usize, PrintGameStringFn>(0x49E3A0)(wide_str, color as i32);
        let _ = WideCString::from_raw(wide_str);
    }
}

fn print_party_string(msg: &str, color: TextColor) {
    type PrintPartyStringFn = extern "fastcall" fn(*const u16, i32);
    unsafe {
        let wide_str = WideCString::from_str(msg).unwrap();
        let wide_str = wide_str.into_raw();
        transmute::<usize, PrintPartyStringFn>(0x49E5C0)(wide_str, color as i32);
        let _ = WideCString::from_raw(wide_str);
    }
}

#[allow(dead_code)]
pub fn print(bottom: bool, color: TextColor, msg: &str) {
    if msg.len() == 0 {
        return;
    }
    match bottom {
        true => print_party_string(msg, color),
        false => print_game_string(msg, color),
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct GameInfo {
    pad_0000: [u8; 31],               // 0x0000
    pub game_name: [c_char; 24],      // 0x001F
    pub server_ip: [c_char; 86],      // 0x0037
    pub account_name: [c_char; 48],   // 0x008D
    pub character_name: [c_char; 24], // 0x00BD
    pub realm_name: [c_char; 24],     // 0x00D5
    pad_00ed: [u8; 344],              // 0x00ED
    pub game_password: [c_char; 18],  // 0x0245
}

#[allow(dead_code)]
impl GameInfo {
    pub fn get() -> Option<GameInfo> {
        unsafe {
            let game_info =
                transmute::<usize, extern "stdcall" fn() -> *const GameInfo>(0x44B7A0)();
            if game_info.is_null() {
                return None;
            }
            Some(*game_info)
        }
    }

    pub fn get_game_name(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.game_name.as_ptr()) }
            .to_str()
            .unwrap()
    }

    pub fn get_server_ip(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.server_ip.as_ptr()) }
            .to_str()
            .unwrap()
    }

    pub fn get_account_name(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.account_name.as_ptr()) }
            .to_str()
            .unwrap()
    }

    pub fn get_realm_name(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.realm_name.as_ptr()) }
            .to_str()
            .unwrap()
    }

    pub fn get_game_password(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.game_password.as_ptr()) }
            .to_str()
            .unwrap()
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Difficulty {
    Normal = 0,
    Nightmare,
    Hell,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Difficulty::Normal => "Normal",
            Difficulty::Nightmare => "Nightmare",
            Difficulty::Hell => "Hell",
        };
        write!(f, "{}", s)
    }
}

impl Difficulty {
    pub fn get() -> Self {
        unsafe { transmute::<usize, fn() -> Difficulty>(0x44DCD0)() }
    }
}
