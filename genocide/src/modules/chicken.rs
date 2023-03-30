use crate::{
    d2::{game::exit_game, unit::Unit},
    hack::SETTINGS,
};

enum CheckType {
    Life,
    Mana,
}

enum SafetyType {
    None,
    Town,
    Exit,
    Potion,
    Rejuv,
}

pub fn run() {
    for check_type in [CheckType::Life, CheckType::Mana] {
        match check(check_type) {
            SafetyType::Town => {}
            SafetyType::Exit => exit_game(),
            SafetyType::Potion => {}
            SafetyType::Rejuv => {}
            SafetyType::None => {}
        }
    }
}

fn check(check_type: CheckType) -> SafetyType {
    let player = Unit::get();
    if let Some(player) = player {
        let (current_value, town_value, exit_value, potion_value, rejuv_value) = match check_type {
            CheckType::Life => (
                player.get_current_hp_percent().unwrap(),
                SETTINGS.chicken.town_life,
                SETTINGS.chicken.exit_life,
                SETTINGS.chicken.potion_life,
                SETTINGS.chicken.rejuv_life,
            ),
            CheckType::Mana => (
                player.get_current_mana_percent().unwrap(),
                SETTINGS.chicken.town_mana,
                SETTINGS.chicken.exit_mana,
                SETTINGS.chicken.potion_mana,
                SETTINGS.chicken.rejuv_mana,
            ),
        };
        if current_value <= exit_value && exit_value > -1 {
            return SafetyType::Exit;
        } else if current_value <= potion_value && potion_value > -1 {
            return SafetyType::Potion;
        } else if current_value <= rejuv_value && rejuv_value > -1 {
            return SafetyType::Rejuv;
        } else if current_value <= town_value && town_value > -1 {
            return SafetyType::Town;
        }
    }
    SafetyType::None
}
