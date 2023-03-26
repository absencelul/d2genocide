use smallvec::SmallVec;
use std::mem::transmute;
use widestring::WideCString;

use super::{
    game::{get_screen_size_x, GameInfo},
    unit::Unit,
};

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
#[allow(unused)]
pub enum Transparency {
    ThreeFourths,
    Half,
    OneFourth,
    White,
    Black,
    Normal,
    Screen,
    Highlight,
    Full,
}

pub struct Draw {}

#[allow(dead_code)]
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

    fn get_text_width(msg: &str) -> u32 {
        type GetTextWidthFn = extern "fastcall" fn(*const u16) -> u32;
        unsafe {
            let wide_str = WideCString::from_str(msg).unwrap();
            let wide_str = wide_str.into_raw();
            let result = transmute::<usize, GetTextWidthFn>(0x501820)(wide_str);
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

    pub fn draw_box(x: u32, y: u32, width: u32, height: u32, color: u32, trans: Transparency) {
        type DrawBoxFn = extern "stdcall" fn(u32, u32, u32, u32, u32, u32) -> bool;
        unsafe {
            transmute::<usize, DrawBoxFn>(0x4F6300)(
                x,
                y,
                x + width,
                y + height,
                color,
                trans as u32,
            );
        }
    }

    pub fn draw_line(x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
        type DrawLineFn = extern "stdcall" fn(u32, u32, u32, u32, u32, i32);
        unsafe { transmute::<usize, DrawLineFn>(0x4F6380)(x1, y1, x2, y2, color, -1) };
    }

    pub fn draw_bordered_box(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        border_color: u32,
        background_color: u32,
        trans: Transparency,
    ) {
        Self::draw_box(x, y, width, height, background_color, trans);
        Self::draw_line(x, y, x + width, y, border_color);
        Self::draw_line(x, y, x, y + height, border_color);
        Self::draw_line(x + width, y, x + width, y + height, border_color);
        Self::draw_line(x, y + height, x + width, y + height, border_color);
    }
}

// TODO: Move these to a plugin
pub extern "C" fn on_draw_interface() {
    // if revealed_areas
    let player = Unit::get();
    if let Some(player) = player {
        let player_class = player.get_player_class().unwrap();
        Draw::draw_text(
            150,
            100,
            TextColor::Gold,
            Alignment::Center,
            1,
            &format!(
                "Player class: {:?}, type: {:?}",
                player_class, player.unit_type
            ),
        );

        let player_pos = player.pos();
        Draw::draw_text(
            150,
            120,
            TextColor::Gold,
            Alignment::Center,
            1,
            &format!("Player pos: {:?}", player_pos),
        );
    }
}

pub extern "C" fn on_draw_automap() {
    let game_info = GameInfo::get();
    if let Some(game_info) = game_info {
        let mut info = Vec::new();
        // Hardcode some game info to the screen
        info.push("FPS: 200, Skip: 0, Ping: 15");
        let game_name = format!("Game: {}", game_info.get_game_name());
        if game_info.get_game_name().len() > 0 {
            info.push(&game_name);
        }
        let game_password = format!("Password: {}", game_info.get_game_password());
        if game_info.get_game_password().len() > 0 {
            info.push(&game_password);
        }
        info.push("Area: ");
        info.push("v 1.14d");
        info.push("Difficulty: ");
        info.push("EXPANSION");

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

        // Draw::draw_box(100, 100, 200, 200, TextColor::Gold, Transparency::Normal);
        // Draw::draw_line(100, 100, 200, 200, TextColor::White);
        // Draw::draw_bordered_box(100, 100, 100, 100, 4, 0, Transparency::ThreeFourths);
    }
}
