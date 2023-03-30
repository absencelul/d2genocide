use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::{collections::HashMap, mem::transmute};
use widestring::WideCString;

use super::{
    game::{get_fps, get_ping, get_skip, Difficulty, GameInfo},
    unit::Unit,
};
use crate::{
    hack::{CONFIG, SETTINGS},
    modules::chicken,
};

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

fn prepare_hashmap() -> Option<HashMap<&'static str, String>> {
    if let Some(player) = Unit::get() {
        let game_info = GameInfo::get().unwrap();
        let fps = get_fps().unwrap().to_string();
        let ping = get_ping().unwrap().to_string();
        let skip = get_skip().unwrap().to_string();
        let variables: HashMap<&str, String> = [
            ("game_name", game_info.get_game_name().to_string()),
            ("game_password", game_info.get_game_password().to_string()),
            ("fps", fps),
            ("ping", ping),
            ("skip", skip),
            ("game_difficulty", Difficulty::get().to_string()),
            ("area_name", player.get_area_id().get_name()),
            (
                "town_life",
                if SETTINGS.chicken.town_life >= 0 {
                    SETTINGS.chicken.town_life.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "town_mana",
                if SETTINGS.chicken.town_mana >= 0 {
                    SETTINGS.chicken.town_mana.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "exit_life",
                if SETTINGS.chicken.exit_life >= 0 {
                    SETTINGS.chicken.exit_life.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "exit_mana",
                if SETTINGS.chicken.exit_mana >= 0 {
                    SETTINGS.chicken.exit_mana.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "potion_life",
                if SETTINGS.chicken.potion_life >= 0 {
                    SETTINGS.chicken.potion_life.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "rejuv_life",
                if SETTINGS.chicken.rejuv_life >= 0 {
                    SETTINGS.chicken.rejuv_life.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "potion_mana",
                if SETTINGS.chicken.potion_mana >= 0 {
                    SETTINGS.chicken.potion_mana.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "rejuv_mana",
                if SETTINGS.chicken.rejuv_mana >= 0 {
                    SETTINGS.chicken.rejuv_mana.to_string()
                } else {
                    "Off".to_string()
                },
            ),
            (
                "life_percent",
                player.get_current_hp_percent().unwrap().to_string(),
            ),
            (
                "mana_percent",
                player.get_current_mana_percent().unwrap().to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        return Some(variables);
    }
    None
}

// TODO: Move these to a plugin
pub extern "C" fn on_draw_interface() {
    // if revealed_areas
    let player = Unit::get();
    if let Some(_player) = player {
        chicken::run();

        let variables = prepare_hashmap().unwrap();
        CONFIG.screen.iter().for_each(|info| {
            let rendered_message = info.render(&variables);
            if !rendered_message.is_empty() {
                Draw::draw_text(
                    info.x,
                    info.y,
                    info.color,
                    info.alignment,
                    info.font,
                    rendered_message.as_str(),
                );
            }
        });
    }
}

pub extern "C" fn on_draw_automap() {
    let game_info = GameInfo::get();
    if let Some(_game_info) = game_info {
        let variables = prepare_hashmap().unwrap();
        CONFIG.automap.iter().for_each(|info| {
            let rendered_message = info.render(&variables);
            if !rendered_message.is_empty() {
                Draw::draw_text(
                    info.x,
                    info.y,
                    info.color,
                    info.alignment,
                    info.font,
                    rendered_message.as_str(),
                );
            }
        });
    }
}
