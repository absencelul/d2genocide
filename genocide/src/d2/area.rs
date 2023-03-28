use std::{ffi::c_void, mem::transmute, slice};

use super::unit::UnitType;
use crate::nested;

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct PresetUnit {
    pad_0000: [u8; 4],                  // 0x0000
    pub text_file_no: u32,              // 0x0004
    pub pos_x: u32,                     // 0x0008
    pub preset_next: *const PresetUnit, // 0x000C
    pad_0010: [u8; 4],                  // 0x0010
    pub preset_type: UnitType,          // 0x0014
    pub pos_y: u32,                     // 0x0018
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct CollisionMap {
    pub pos_game_x: u32,       // 0x0000
    pub pos_game_y: u32,       // 0x0004
    pub size_game_x: u32,      // 0x0008
    pub size_game_y: u32,      // 0x000C
    pub pos_room_x: u32,       // 0x0010
    pub pos_room_y: u32,       // 0x0014
    pub size_room_x: u32,      // 0x0018
    pub size_room_y: u32,      // 0x001C
    pub map_start: *const u16, // 0x0020
    pub map_end: *const u16,   // 0x0022
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct RoomTile {
    pub room_ex: *const RoomEx,     // 0x0000
    pub next_tile: *const RoomTile, // 0x0004
    pad_0008: [u8; 8],              // 0x0008
    pub num: *const u32,            // 0x0010
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct RoomEx {
    pad_0000: [u8; 8],                      // 0x0000
    pub room_ex_near: *const *const RoomEx, // 0x0008
    pad_000c: [u8; 20],                     // 0x000C
    type_2_info: *const u32,                // 0x0020
    pub room_ex_next: *const RoomEx,        // 0x0024
    pub room_flags: u32,                    // 0x0028
    pub rooms_near: u32,                    // 0x002C
    pub room: *const Room,                  // 0x0030
    pub pos_x: u32,                         // 0x0034
    pub pos_y: u32,                         // 0x0038
    pub size_x: u32,                        // 0x003C
    pub size_y: u32,                        // 0x0040
    pad_0044: [u8; 4],                      // 0x0044
    pub preset_type: u32,                   // 0x0048
    room_tiles: *const RoomTile,            // 0x004C
    pad_0050: [u8; 8],                      // 0x0050
    pub level: *const Level,                // 0x0058
    pub preset: *const PresetUnit,          // 0x005C
}

#[allow(dead_code)]
impl RoomEx {
    fn init(&self, act: &Act) -> bool {
        if self.room.is_null() {
            let area = nested!(self->level).unwrap().level_no;
            type AddRoomDataFn = extern "stdcall" fn(*const Act, u32, u32, u32, *const Room);
            unsafe {
                transmute::<usize, AddRoomDataFn>(0x61A070)(
                    act, area, self.pos_x, self.pos_y, self.room,
                );
            }
            return true;
        }
        false
    }

    fn cleanup(&self, act: &Act) {
        let area = nested!(self->level).unwrap().level_no;
        type RemoveRoomDataFn = extern "stdcall" fn(*const Act, u32, u32, u32, *const Room);
        unsafe {
            transmute::<usize, RemoveRoomDataFn>(0x61A0C0)(
                act, area, self.pos_x, self.pos_y, self.room,
            );
        }
    }

    fn initialized(&self) -> bool {
        !self.room.is_null()
    }

    pub fn reveal(&self, act: &Act) {
        let reveal_data = self.init(act);
        if let Some(room) = nested!(self->room) {
            if self.initialized() {
                room.reveal();
                if reveal_data {
                    self.cleanup(act);
                }
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct Room {
    pub rooms_near: *const *const Room,     // 0x0000
    pad_0004: [u8; 12],                     // 0x0004
    pub room_ex: *const RoomEx,             // 0x0010
    pad_0014: [u8; 12],                     // 0x0014
    pub collision_map: *const CollisionMap, // 0x0020
    pub rooms: u32,                         // 0x0024
    pad_0028: [u8; 36],                     // 0x0028
    pub start_x: u32,                       // 0x004C
    pub start_y: u32,                       // 0x0050
    pub size_x: u32,                        // 0x0054
    pub size_y: u32,                        // 0x0058
    pad_005c: [u8; 24],                     // 0x005C
    unit_first: *const c_void,              // 0x0074
    pad_0078: [u8; 4],                      // 0x0078
    pub room_next: *const Room,             // 0x007C
}

impl Room {
    fn reveal(&self) {
        type LoactActIFn = extern "stdcall" fn(*const Room) -> *const c_void;
        unsafe {
            transmute::<usize, LoactActIFn>(0x459150)(self);
        }
    }
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct Level {
    pad_0000: [u8; 16],               // 0x0000
    pub room_ex_first: *const RoomEx, // 0x0010
    pad_0014: [u8; 8],                // 0x0014
    pub pos_x: u32,                   // 0x001C
    pub pos_y: u32,                   // 0x0020
    pub size_x: u32,                  // 0x0024
    pub size_y: u32,                  // 0x0028
    pad_002c: [u8; 384],              // 0x002C
    pub level_next: *const Level,     // 0x01AC
    pad_01b0: [u8; 4],                // 0x01B0
    pub act_misc: *const ActMisc,     // 0x01B4
    pad_01b8: [u8; 12],               // 0x01B8
    pub seed: [u32; 2],               // 0x01C4
    pad_01cc: [u8; 4],                // 0x01CC
    pub level_no: u32,                // 0x01D0
    pad_01d4: [u8; 12],               // 0x01D4
}

#[allow(dead_code)]
impl Level {
    fn init(&self) {
        type InitLevelFn = extern "stdcall" fn(*const Level);
        unsafe {
            transmute::<usize, InitLevelFn>(0x6424A0)(self);
        }
    }

    fn initialized(&self) -> bool {
        if self.room_ex_first.is_null() {
            return false;
        }
        true
    }

    pub fn reveal(&self) {
        if self.initialized() {
            let act = nested!(self->act_misc->act).unwrap();
            let mut room_ex_opt = nested!(self->room_ex_first);
            while let Some(room_ex) = room_ex_opt {
                room_ex.reveal(act);

                let near_rooms: &[*const RoomEx] = unsafe {
                    slice::from_raw_parts(room_ex.room_ex_near, room_ex.rooms_near as usize)
                };
                near_rooms.iter().for_each(|near_room| {
                    let room_ex_near = unsafe { &**near_room };
                    let level_near = unsafe { &*room_ex_near.level };
                    if level_near.level_no != self.level_no && level_near.initialized() {
                        room_ex_near.reveal(act);
                    }
                });

                room_ex_opt = nested!(room_ex->room_ex_next);
            }
        }
    }
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct ActMisc {
    pad_0000: [u8; 148],           // 0x0000
    pub staff_tomb_level: u32,     // 0x0094
    pad_0098: [u8; 980],           // 0x0098
    pub act: *const Act,           // 0x046C
    pad_0470: [u8; 12],            // 0x0470
    pub level_first: *const Level, // 0x047C
}

#[derive(Debug)]
#[repr(C)]
#[allow(unused)]
pub struct Act {
    pad_0000: [u8; 12],            // 0x0000
    pub map_seed: u32,             // 0x000C
    pub room: *const Room,         // 0x0010
    pub act: u32,                  // 0x0014
    pad_0018: [u8; 12],            // 0x0018
    pub level_first: *const Level, // 0x0024
    pad_0028: [u8; 32],            // 0x0028
    pub act_misc: *const ActMisc,  // 0x0048
}

#[repr(u32)]
#[allow(unused)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum AreaId {
    None = 0,
    RogueEncampment = 1,
    BloodMoor = 2,
    ColdPlains = 3,
    StonyField = 4,
    DarkWood = 5,
    BlackMarsh = 6,
    TamoeHighland = 7,
    DenOfEvil = 8,
    CaveLevel1 = 9,
    UndergroundPassageLevel1 = 10,
    HoleLevel1 = 11,
    PitLevel1 = 12,
    CaveLevel2 = 13,
    UndergroundPassageLevel2 = 14,
    HoleLevel2 = 15,
    PitLevel2 = 16,
    BurialGrounds = 17,
    Crypt = 18,
    Mausoleum = 19,
    ForgottenTower = 20,
    TowerCellarLevel1 = 21,
    TowerCellarLevel2 = 22,
    TowerCellarLevel3 = 23,
    TowerCellarLevel4 = 24,
    TowerCellarLevel5 = 25,
    MonasteryGate = 26,
    OuterCloister = 27,
    Barracks = 28,
    JailLevel1 = 29,
    JailLevel2 = 30,
    JailLevel3 = 31,
    InnerCloister = 32,
    Cathedral = 33,
    CatacombsLevel1 = 34,
    CatacombsLevel2 = 35,
    CatacombsLevel3 = 36,
    CatacombsLevel4 = 37,
    Tristram = 38,
    MooMooFarm = 39,
    LutGholein = 40,
    RockyWaste = 41,
    DryHills = 42,
    FarOasis = 43,
    LostCity = 44,
    ValleyOfSnakes = 45,
    CanyonOfTheMagi = 46,
    A2SewersLevel1 = 47,
    A2SewersLevel2 = 48,
    A2SewersLevel3 = 49,
    HaremLevel1 = 50,
    HaremLevel2 = 51,
    PalaceCellarLevel1 = 52,
    PalaceCellarLevel2 = 53,
    PalaceCellarLevel3 = 54,
    StonyTombLevel1 = 55,
    HallsOfTheDeadLevel1 = 56,
    HallsOfTheDeadLevel2 = 57,
    ClawViperTempleLevel1 = 58,
    StonyTombLevel2 = 59,
    HallsOfTheDeadLevel3 = 60,
    ClawViperTempleLevel2 = 61,
    MaggotLairLevel1 = 62,
    MaggotLairLevel2 = 63,
    MaggotLairLevel3 = 64,
    AncientTunnels = 65,
    TalRashasTomblevel1 = 66,
    TalRashasTomblevel2 = 67,
    TalRashasTomblevel3 = 68,
    TalRashasTomblevel4 = 69,
    TalRashasTomblevel5 = 70,
    TalRashasTomblevel6 = 71,
    TalRashasTomblevel7 = 72,
    DurielsLair = 73,
    ArcaneSanctuary = 74,
    KurastDocktown = 75,
    SpiderForest = 76,
    GreatMarsh = 77,
    FlayerJungle = 78,
    LowerKurast = 79,
    KurastBazaar = 80,
    UpperKurast = 81,
    KurastCauseway = 82,
    Travincal = 83,
    SpiderCave = 84,
    SpiderCavern = 85,
    SwampyPitLevel1 = 86,
    SwampyPitLevel2 = 87,
    FlayerDungeonLevel1 = 88,
    FlayerDungeonLevel2 = 89,
    SwampyPitLevel3 = 90,
    FlayerDungeonLevel3 = 91,
    A3SewersLevel1 = 92,
    A3SewersLevel2 = 93,
    RuinedTemple = 94,
    DisusedFane = 95,
    ForgottenReliquary = 96,
    ForgottenTemple = 97,
    RuinedFane = 98,
    DisusedReliquary = 99,
    DuranceOfHateLevel1 = 100,
    DuranceOfHateLevel2 = 101,
    DuranceOfHateLevel3 = 102,
    ThePandemoniumFortress = 103,
    OuterSteppes = 104,
    PlainsOfDespair = 105,
    CityOfTheDamned = 106,
    RiverOfFlame = 107,
    ChaosSanctuary = 108,
    Harrogath = 109,
    BloodyFoothills = 110,
    FrigidHighlands = 111,
    ArreatPlateau = 112,
    CrystalizedPassage = 113,
    FrozenRiver = 114,
    GlacialTrail = 115,
    DrifterCavern = 116,
    FrozenTundra = 117,
    AncientsWay = 118,
    IcyCellar = 119,
    ArreatSummit = 120,
    NihlathaksTemple = 121,
    HallsOfAnguish = 122,
    HallsOfPain = 123,
    HallsOfVaught = 124,
    Abaddon = 125,
    PitOfAcheron = 126,
    InfernalPit = 127,
    TheWorldstoneKeepLevel1 = 128,
    TheWorldstoneKeepLevel2 = 129,
    TheWorldstoneKeepLevel3 = 130,
    ThroneOfDestruction = 131,
    TheWorldstoneChamber = 132,
    MatronsDen = 133,
    FogottenSands = 134,
    FurnaceOfPain = 135,
    Tristram2 = 136,
}

#[allow(dead_code)]
impl AreaId {
    pub fn is_town(&self) -> bool {
        matches!(
            self,
            Self::RogueEncampment
                | Self::LutGholein
                | Self::KurastDocktown
                | Self::ThePandemoniumFortress
                | Self::Harrogath
                | Self::None
        )
    }

    // pub fn get_name(&self) -> &str {
    //     type GetLevelNameFn = extern "fastcall" fn(u32) -> *const u16;
    //     let level_name = unsafe {
    //         let level_name = transmute::<usize, GetLevelNameFn>(0x53E70)(self as *const _ as u32);
    //         let mut len = 0;
    //         while *level_name.add(len) != 0 {
    //             len += 1;
    //         }
    //         let level_name = std::slice::from_raw_parts(level_name, len);
    //         String::from_utf16_lossy(level_name)
    //     };
    //     level_name.as_str()
    // }
}
