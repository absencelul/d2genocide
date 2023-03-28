use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{collections::HashMap, mem::transmute};
use widestring::WideCString;

use super::{
    game::{get_fps, get_ping, get_skip, Difficulty, GameInfo},
    unit::Unit,
};
use crate::hack::CONFIG;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

    #[allow(dead_code)]
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
    if let Some(_player) = player {
        {
            Draw::draw_text(
                5,
                575,
                TextColor::White,
                Alignment::None,
                4,
                &format!("ÿc1{} ÿc;{}", "Off", "Off"),
            );
            Draw::draw_text(
                5,
                588,
                TextColor::White,
                Alignment::None,
                4,
                &format!("ÿc8{} ÿc7{}", "Off", "Off"),
            );
            Draw::draw_text(
                756,
                575,
                TextColor::White,
                Alignment::None,
                4,
                &format!("ÿc3{} ÿc;{}", "Off", "Off"),
            );
            Draw::draw_text(
                756,
                588,
                TextColor::White,
                Alignment::None,
                4,
                &format!("ÿc8{} ÿc7{}", "Off", "Off"),
            );
            // Draw::draw_text(
            //     375,
            //     15,
            //     TextColor::Gold,
            //     Alignment::None,
            //     4,
            //     &format!("{:?}", chrono::offset::Utc::now()),
            // )
        }

        let y_size = Draw::get_text_size("100%", 0)[0] + 15;
        Draw::draw_bordered_box(50, 528, y_size, 15, 4, 0, Transparency::Normal);
        Draw::draw_bordered_box(715, 528, y_size, 15, 4, 0, Transparency::Normal);
        Draw::draw_text(52 + 2, 528 + 4, TextColor::Gold, Alignment::None, 8, "100%");
        Draw::draw_text(
            715 + 2,
            528 + 4,
            TextColor::Gold,
            Alignment::None,
            8,
            "100%",
        );
    }
}

pub extern "C" fn on_draw_automap() {
    let game_info = GameInfo::get();
    if let Some(game_info) = game_info {
        let fps = get_fps().unwrap().to_string();
        let ping = get_ping().unwrap().to_string();
        let skip = get_skip().unwrap().to_string();
        let variables: HashMap<&str, &str> = [
            ("game_name", game_info.get_game_name()),
            ("game_password", game_info.get_game_password()),
            ("fps", &fps),
            ("ping", &ping),
            ("skip", &skip),
            ("game_difficulty", Difficulty::get().unwrap()),
        ]
        .into();

        CONFIG.automap.iter().for_each(|info| {
            let rendered_messaged = info.render(&variables);
            if rendered_messaged.len() > 0 {
                Draw::draw_text(
                    info.x,
                    info.y,
                    info.color,
                    info.alignment,
                    info.font,
                    rendered_messaged.as_str(),
                );
            }
        });
    }
}
