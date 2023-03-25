use smallvec::SmallVec;
use std::mem::transmute;
use widestring::WideCString;

use super::game::get_screen_size_x;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
#[allow(unused)]
pub enum TextColor {
    White,
    Red,
    Green,
    Blue,
    Gold,
    Gray,
    Black,
    Tan,
    Orange,
    Yellow,
    DarkGreen,
    Purple,
    Silver = 15,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
#[allow(unused)]
pub enum Alignment {
    None,
    Center,
    Right,
    Top = 4,
}

pub struct Draw {}

impl Draw {
    fn text_height_by_font(font: u8) -> u32 {
        let height: [u32; 14] = [10, 11, 18, 24, 10, 13, 7, 13, 10, 12, 8, 8, 7, 12];
        return height[font as usize];
    }

    fn get_text_width_file_no(msg: &str, width: &mut u32, file_no: &mut u32) -> u32 {
        type GetTextWidthFileNoFn = extern "fastcall" fn(*const u16, *mut u32, *mut u32) -> u32;
        unsafe {
            let wide_str = WideCString::from_str(msg).unwrap();
            let wide_str = wide_str.into_raw();
            let result =
                transmute::<usize, GetTextWidthFileNoFn>(0x502520)(wide_str, width, file_no);
            let _ = WideCString::from_raw(wide_str);
            result
        }
    }

    fn set_text_size(size: u32) -> u32 {
        type SetTextSizeFn = extern "fastcall" fn(u32) -> u32;
        unsafe { transmute::<usize, SetTextSizeFn>(0x502EF0)(size) }
    }

    pub fn get_text_size(msg: &str, font: u8) -> SmallVec<[u32; 2]> {
        unsafe {
            let old_font = Self::set_text_size(font as u32);
            let mut width: u32 = 0;
            let mut file_no: u32 = 0;
            let wide_str = WideCString::from_str(msg).unwrap();
            let wide_str = wide_str.into_raw();
            Self::get_text_width_file_no(msg, &mut width, &mut file_no);
            let _ = WideCString::from_raw(wide_str);
            Self::set_text_size(old_font);
            return SmallVec::from_buf([width, Self::text_height_by_font(font)]);
        }
    }

    pub fn draw_text(x: i32, y: i32, color: TextColor, alignment: Alignment, font: u8, msg: &str) {
        type DrawTextFn = extern "fastcall" fn(*const u16, i32, i32, i32, i32) -> bool;
        let pos_x = match alignment {
            Alignment::Center => x - Self::get_text_size(msg, font)[0] as i32 / 2,
            Alignment::Right => x - Self::get_text_size(msg, font)[0] as i32,
            _ => x,
        };
        unsafe {
            let wide_str = WideCString::from_str(msg).unwrap();
            let wide_str = wide_str.into_raw();
            let size = Self::set_text_size(font as u32);
            transmute::<usize, DrawTextFn>(0x502320)(
                wide_str,
                pos_x,
                y + Self::text_height_by_font(font) as i32,
                color as i32,
                0,
            );
            let _ = WideCString::from_raw(wide_str);
            Self::set_text_size(size);
        }
    }
}

// TODO: Move these to a plugin
pub extern "C" fn on_draw_interface() {}

pub extern "C" fn on_draw_automap() {
    // Hardcode some game info to the screen
    let info = vec![
        "FPS: 60, Skip: 0, Ping: 0",
        "Game: ",
        "Password: ",
        "Area: ",
        "v 1.14d",
        "Difficulty: ",
        "EXPANSION",
    ];

    let mut y = 0;
    info.iter().for_each(|&msg| {
        y += 16;
        Draw::draw_text(
            (get_screen_size_x().unwrap() - 18).try_into().unwrap(),
            y,
            TextColor::Gold,
            Alignment::Right,
            1,
            msg,
        );
    });
}
