use smallvec::{smallvec, SmallVec};
use std::mem::transmute;
use winapi::ctypes::c_void;

use crate::nested;

use super::{
    area::{Act, AreaId, Room},
    stats::StatId,
};

#[repr(C)]
#[allow(unused)]
#[derive(Clone, Copy)]
pub enum NpcUnitMode {
    Death,
    Stand,
    Walk,
    BeingHit,
    Attack1,
    Attack2,
    Block,
    Cast,
    UseSkill1,
    UseSkill2,
    UseSkill3,
    UseSkill4,
    Dead,
    KnockedBack,
    Sequence,
    Run,
}

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

    pub fn get_unit_stat(&self, stat: StatId) -> Option<i32> {
        type GetUnitStatFn = extern "stdcall" fn(*const Unit, stat: StatId, stat_2: i32) -> i32;
        let stat = unsafe {
            let stat = transmute::<usize, GetUnitStatFn>(0x625480)(self, stat, 0);
            if stat < 0 {
                return None;
            }
            stat
        };
        Some(stat)
    }

    pub fn get_current_hp_percent(&self) -> Option<i32> {
        if self.unit_type != UnitType::Player {
            return None;
        }
        let life = self.get_unit_stat(StatId::Life);
        let max_life = self.get_unit_stat(StatId::MaxLife);
        if life.is_none() || max_life.is_none() {
            return None;
        }
        let life = life.unwrap();
        let max_life = max_life.unwrap();
        let mut value = ((life as f32 / max_life as f32) * 100.0) as i32;
        if value > 100 {
            value = 100;
        }
        Some(value)
    }

    pub fn get_current_mana_percent(&self) -> Option<i32> {
        if self.unit_type != UnitType::Player {
            return None;
        }
        let mana = self.get_unit_stat(StatId::Mana);
        let max_mana = self.get_unit_stat(StatId::MaxMana);
        if mana.is_none() || max_mana.is_none() {
            return None;
        }
        let mana = mana.unwrap();
        let max_mana = max_mana.unwrap();
        let mut value = ((mana as f32 / max_mana as f32) * 100.0) as i32;
        if value > 100 {
            value = 100;
        }
        Some(value)
    }

    pub fn get_area_id(&self) -> AreaId {
        let level = nested!(self.path->dynamic_path->room->room_ex->level);
        match level {
            Some(level) => level.level_no,
            None => AreaId::None,
        }
    }

    pub fn get_monster_owner_id(unit_id: u32) -> u32 {
        type GetMonsterOwnerIdFn = extern "fastcall" fn(u32) -> u32;
        unsafe {
            let owner_id = transmute::<usize, GetMonsterOwnerIdFn>(0x479150)(unit_id);
            return owner_id;
        }
    }

    pub fn get_player_merc(&self) -> Option<&'static Unit> {
        if self.unit_type != UnitType::Player {
            return None;
        }
        let mut room_opt = nested!(self->act->room);
        while let Some(room) = room_opt {
            let mut unit_opt = nested!(room->unit_first);
            while let Some(unit) = unit_opt {
                match unit.class_id {
                    0x010f | 0x0152 | 0x0167 | 0x0231 => {
                        if Self::get_monster_owner_id(unit.unit_id) == self.unit_id
                            && unit.mode != NpcUnitMode::Dead as u32
                            && unit.mode != NpcUnitMode::Death as u32
                        {
                            return Some(&*unit);
                        }
                    }
                    _ => {}
                }
                unit_opt = nested!(unit->list_next);
            }

            room_opt = nested!(room->room_next);
        }
        None
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
