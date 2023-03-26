use smallvec::{smallvec, SmallVec};
use std::mem::transmute;
use winapi::ctypes::c_void;

use super::area::{Act, Room};

#[repr(u32)]
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Player,
    Monster,
    Object,
    Missile,
    Item,
    Tile,
    Exit,
    Xy,
}

#[repr(u32)]
#[allow(unused)]
#[derive(Debug)]
pub enum PlayerClass {
    Amazon,
    Sorceress,
    Necromancer,
    Paladin,
    Barbarian,
    Druid,
    Assassin,
}

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct DynamicPathPosition {
    pub offset_x: u16, // 0x0000
    pub x: u16,        // 0x0002
    pub offset_y: u16, // 0x0004
    pub y: u16,        // 0x0006
}

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct DynamicPathPrecision {
    pub x: u32, // 0x000C
    pub y: u32, // 0x0010
}

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub union PathPositionOrPrecision {
    pub position: DynamicPathPosition,
    pub precision: DynamicPathPrecision,
}

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct DynamicPath {
    pub position_or_precision: PathPositionOrPrecision, // 0x0000
    pad_0008: [u8; 8],                                  // 0x0008
    pub target_x: u16,                                  // 0x0010
    pub target_y: u16,                                  // 0x0012
    pad_0014: [u8; 8],                                  // 0x0014
    pub room: *const Room,                              // 0x001C
    room_unk: *const Room,                              // 0x0020
    pad_0024: [u8; 12],                                 // 0x0024
    pub unit: *const Unit,                              // 0x0030
    pub flags: u32,                                     // 0x0034
    pad_0038: [u8; 4],                                  // 0x0038
    pub path_type: u32,                                 // 0x003C
    pub prev_path_type: u32,                            // 0x0040
    pub unit_size: u32,                                 // 0x0044
    pad_0048: [u8; 16],                                 // 0x0048
    pub target_unit: *const Unit,                       // 0x0058
    pub target_type: u32,                               // 0x005C
    pub target_id: u32,                                 // 0x0060
    pub direction: u8,                                  // 0x0064
}

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct StaticPath {
    pub room: *const Room, // 0x0000
    pub target_x: u32,     // 0x0004
    pub target_y: u32,     // 0x0008
    pub x: u32,            // 0x000C
    pub y: u32,            // 0x0010
}

#[repr(C)]
#[allow(unused)]
pub union PathUnion {
    pub dynamic_path: *mut DynamicPath,
    pub static_path: *mut StaticPath,
}

#[repr(C)]
#[allow(unused)]
pub struct Unit {
    pub unit_type: UnitType,     // 0x0000
    pub class_id: u32,           // 0x0004
    pad_0008: [u8; 4],           // 0x0008
    pub unit_id: u32,            // 0x000C
    pub mode: u32,               // 0x0010
    unit_union: usize,           // 0x0014
    pub act_id: u32,             // 0x0018
    pub act: *const Act,         // 0x001C
    pub seed: [u32; 2],          // 0x0020
    pad_0028: [u8; 4],           // 0x0028
    pub path: PathUnion,         // 0x002C
    pad_0030: [u8; 20],          // 0x0030
    pub gfx_frame: u32,          // 0x0044
    pub frame_remain: u32,       // 0x0048
    pub frame_rate: u16,         // 0x004C
    pad_004e: [u8; 2],           // 0x004E
    gfx_unk: *const c_void,      // 0x0050
    gfx_info: *const c_void,     // 0x0054
    pad_0058: [u8; 4],           // 0x0058
    stat_list: *const c_void,    // 0x005C
    inventory: *const c_void,    // 0x0060
    light: *const c_void,        // 0x0064
    pad_0068: [u8; 36],          // 0x0068
    pub x: u16,                  // 0x008C
    pub y: u16,                  // 0x008E
    pad_0090: [u8; 4],           // 0x0090
    pub owner_type: u32,         // 0x0094
    pub owner_id: u32,           // 0x0098
    pad_009c: [u8; 8],           // 0x009C
    overhead_msg: *const c_void, // 0x00A4
    info: *const c_void,         // 0x00A8
    pad_00ac: [u8; 24],          // 0x00AC
    pub flags: u32,              // 0x00C4
    pub flags_2: u32,            // 0x00C8
    pad_00cc: [u8; 20],          // 0x00CC
    changed_next: *const Unit,   // 0x00E0
    pub room_next: *const Unit,  // 0x00E4
    pub list_next: *const Unit,  // 0x00E8
}

impl Unit {
    pub fn get() -> Option<&'static Unit> {
        type GetPlayerUnitFn = extern "stdcall" fn() -> *const Unit;
        unsafe {
            let unit = transmute::<usize, GetPlayerUnitFn>(0x463DD0)();
            if unit.is_null() {
                return None;
            }
            return Some(&*unit);
        }
    }

    pub fn get_player_class(&self) -> Option<PlayerClass> {
        if self.unit_type != UnitType::Player {
            return None;
        }
        return Some(unsafe { std::mem::transmute::<u32, PlayerClass>(self.class_id) });
    }

    fn pos_is_static(&self) -> bool {
        match self.unit_type {
            UnitType::Object | UnitType::Item => true,
            _ => false,
        }
    }

    pub fn pos(&self) -> SmallVec<[u32; 2]> {
        if self.pos_is_static() {
            let path = unsafe { *self.path.static_path };
            return smallvec![path.x, path.y];
        }
        let pos = unsafe { (*self.path.dynamic_path).position_or_precision.position };
        smallvec![pos.x as u32, pos.y as u32]
    }
}
