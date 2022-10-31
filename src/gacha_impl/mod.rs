use super::gacha_traits::*;
use super::gacha_traits::gacha_enums::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

pub struct NormalGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    since_last_5star_character: u32,
    since_last_5star_weapon: u32,
    since_last_4star_character: u32,
    since_last_4star_weapon: u32,
}

pub struct CharacterGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    since_last_4star_character: u32,
    since_last_4star_weapon: u32,
    last_5star_is_up: bool,
    last_4star_is_up: bool,
}

pub struct WeaponGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    since_last_4star_character: u32,
    since_last_4star_weapon: u32,
    last_5star_is_up: bool,
    last_4star_is_up: bool,
    want_5star_state: WantState,
}

impl Gacha5StarInfo for NormalGachaState {
    const BASE_PROBABILITY_5STAR: f64 = 0.006;
    const THRESHOLD_5STAR: u32 = 73;
    const MAX_5STAR: u32 = 90;

    fn get_since_last_5star(&self) -> u32 {
        self.since_last_5star
    }

    fn set_since_last_5star(&mut self, count: u32) {
        self.since_last_5star = count;
    }
}

impl Gacha4StarInfo for NormalGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.051;
    const THRESHOLD_4STAR: u32 = 8;
    const MAX_4STAR: u32 = 10;

    fn get_since_last_4star(&self) -> u32 {
        self.since_last_4star
    }

    fn set_since_last_4star(&mut self, count: u32) {
        self.since_last_4star = count;
    }
}

impl Balance5Star for NormalGachaState {
    const BALANCE_THRESHOLD_5STAR: u32 = 146;

    fn get_since_last_5star_character(&self) -> u32 {
        self.since_last_5star_character
    }

    fn get_since_last_5star_weapon(&self) -> u32 {
        self.since_last_5star_weapon
    }

    fn set_since_last_5star_character(&mut self, count: u32) {
        self.since_last_5star_character = count;
    }

    fn set_since_last_5star_weapon(&mut self, count: u32) {
        self.since_last_5star_weapon = count;
    }
}

impl Balance4Star for NormalGachaState {
    const BALANCE_THRESHOLD_4STAR: u32 = 17;

    fn get_since_last_4star_character(&self) -> u32 {
        self.since_last_4star_character
    }
    
    fn get_since_last_4star_weapon(&self) -> u32 {
        self.since_last_4star_weapon
    }

    fn set_since_last_4star_character(&mut self, count: u32) {
        self.since_last_4star_character = count;
    }

    fn set_since_last_4star_weapon(&mut self, count: u32) {
        self.since_last_4star_weapon = count;
    }
}

impl GeneralGachaMechanism for NormalGachaState {}

impl Gacha5StarInfo for CharacterGachaState {
    const BASE_PROBABILITY_5STAR: f64 = 0.006;
    const THRESHOLD_5STAR: u32 = 73;
    const MAX_5STAR: u32 = 90;

    fn get_since_last_5star(&self) -> u32 {
        self.since_last_5star
    }

    fn set_since_last_5star(&mut self, count: u32) {
        self.since_last_5star = count;
    }
}

impl Gacha4StarInfo for CharacterGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.051;
    const THRESHOLD_4STAR: u32 = 8;
    const MAX_4STAR: u32 = 10;

    fn get_since_last_4star(&self) -> u32 {
        self.since_last_4star
    }

    fn set_since_last_4star(&mut self, count: u32) {
        self.since_last_4star = count;
    }
}

impl Up5Star for CharacterGachaState {
    const UP_PROBABILITY_5STAR: f64 = 0.5;
    const UP_5STAR_NUM: u32 = 1;

    fn get_last_5star_is_up(&self) -> bool {
        self.last_5star_is_up
    }

    fn set_last_5star_is_up(&mut self, is_up: bool) {
        self.last_5star_is_up = is_up;
    }
}

impl Up4Star for CharacterGachaState {
    const UP_PROBABILITY_4STAR: f64 = 0.5;
    const UP_4STAR_NUM: u32 = 3;

    fn get_last_4star_is_up(&self) -> bool {
        self.last_4star_is_up
    }

    fn set_last_4star_is_up(&mut self, is_up: bool) {
        self.last_4star_is_up = is_up;
    }
}

impl Balance4Star for CharacterGachaState {
    const BALANCE_THRESHOLD_4STAR: u32 = 17;

    fn get_since_last_4star_character(&self) -> u32 {
        self.since_last_4star_character
    }
    
    fn get_since_last_4star_weapon(&self) -> u32 {
        self.since_last_4star_weapon
    }

    fn set_since_last_4star_character(&mut self, count: u32) {
        self.since_last_4star_character = count;
    }

    fn set_since_last_4star_weapon(&mut self, count: u32) {
        self.since_last_4star_weapon = count;
    }
}

impl GeneralGachaMechanism for CharacterGachaState {}

impl Gacha5StarInfo for WeaponGachaState {
    const BASE_PROBABILITY_5STAR: f64 = 0.007;
    const THRESHOLD_5STAR: u32 = 62;
    const MAX_5STAR: u32 = 80;

    fn get_since_last_5star(&self) -> u32 {
        self.since_last_5star
    }

    fn set_since_last_5star(&mut self, count: u32) {
        self.since_last_5star = count;
    }
}

impl Gacha4StarInfo for WeaponGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.06;
    const THRESHOLD_4STAR: u32 = 7;
    const MAX_4STAR: u32 = 10;

    fn get_since_last_4star(&self) -> u32 {
        self.since_last_4star
    }

    fn set_since_last_4star(&mut self, count: u32) {
        self.since_last_4star = count;
    }
}

impl Up5Star for WeaponGachaState {
    const UP_PROBABILITY_5STAR: f64 = 0.75;
    const UP_5STAR_NUM: u32 = 2;

    fn get_last_5star_is_up(&self) -> bool {
        self.last_5star_is_up
    }

    fn set_last_5star_is_up(&mut self, is_up: bool) {
        self.last_5star_is_up = is_up;
    }
}

impl Up4Star for WeaponGachaState {
    const UP_PROBABILITY_4STAR: f64 = 0.75;
    const UP_4STAR_NUM: u32 = 5;

    fn get_last_4star_is_up(&self) -> bool {
        self.last_4star_is_up
    }

    fn set_last_4star_is_up(&mut self, is_up: bool) {
        self.last_4star_is_up = is_up;
    }
}

impl Want5Star for WeaponGachaState {
    const WANT_5STAR_MAX: u32 = 2;
}

impl Balance4Star for WeaponGachaState {
    const BALANCE_THRESHOLD_4STAR: u32 = 14;

    fn get_since_last_4star_character(&self) -> u32 {
        self.since_last_4star_character
    }
    
    fn get_since_last_4star_weapon(&self) -> u32 {
        self.since_last_4star_weapon
    }

    fn set_since_last_4star_character(&mut self, count: u32) {
        self.since_last_4star_character = count;
    }

    fn set_since_last_4star_weapon(&mut self, count: u32) {
        self.since_last_4star_weapon = count;
    }
}

impl GeneralGachaMechanism for WeaponGachaState {}

impl NormalGachaState {
    pub fn simulate_normal_gacha(&mut self) -> NormalGachaType {
        match self.get_item_level() {
            ItemLevel::Star3 => {
                self.since_last_5star_character += 1;
                self.since_last_5star_weapon += 1;
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
                NormalGachaType::Other3Star
            }
            ItemLevel::Star4 => {
                self.since_last_5star_character += 1;
                self.since_last_5star_weapon += 1;
    
                match self.get_4star_balance_type() {
                    ItemType::Character => {
                        NormalGachaType::Character4Star
                    }
                    ItemType::Weapon => {
                        NormalGachaType::Weapon4Star
                    }
                }
            }
            ItemLevel::Star5 => {
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
    
                match self.get_5star_balance_type() {
                    ItemType::Character => {
                        NormalGachaType::Character5Star
                    }
                    ItemType::Weapon => {
                        NormalGachaType::Weapon5Star
                    }
                }
            }
        }
    }
}

impl CharacterGachaState {
    pub fn new() -> CharacterGachaState {
        CharacterGachaState {
            since_last_5star: 0,
            since_last_4star: 0,
            since_last_4star_character: 0,
            since_last_4star_weapon: 0,
            last_5star_is_up: true,
            last_4star_is_up: true,
        }
    }

    pub fn simulate_character_gacha(&mut self) -> CharacterGachaType {
        match self.get_item_level() {
            ItemLevel::Star3 => {
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
                CharacterGachaType::Other3Star
            }
            ItemLevel::Star4 => {
                match self.get_4star_up_type() {
                    UpType::Up(index) => {
                        self.since_last_4star_character = 0;
                        self.since_last_4star_weapon += 1;
                        CharacterGachaType::Up4Star(index)
                    }
                    UpType::NonUp => {
                        match self.get_4star_balance_type() {
                            ItemType::Character => {
                                CharacterGachaType::Other4StarCharacter
                            }
                            ItemType::Weapon => {
                                CharacterGachaType::Other4StarWeapon
                            }
                        }
                    }
                }
            }
            ItemLevel::Star5 => {
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;

                match self.get_5star_up_type() {
                    UpType::Up(_) => {
                        CharacterGachaType::Up5Star
                    }
                    UpType::NonUp => {
                        CharacterGachaType::Other5Star
                    }
                }
            }
        }
    }
}

impl WeaponGachaState {
    pub fn new() -> WeaponGachaState {
        WeaponGachaState {
            since_last_5star: 0,
            since_last_4star: 0,
            since_last_4star_character: 0,
            since_last_4star_weapon: 0,
            last_5star_is_up: true,
            last_4star_is_up: true,
            want_5star_state: WantState::Want(0, 0),
        }
    }

    pub fn simulate_weapon_gacha(&mut self) -> WeaponGachaType {
        match self.get_item_level() {
            ItemLevel::Star3 => {
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
                WeaponGachaType::Other3Star
            }
            ItemLevel::Star4 => {
                match self.get_4star_up_type() {
                    UpType::Up(index) => {
                        self.since_last_4star_character += 1;
                        self.since_last_4star_weapon = 0;
                        WeaponGachaType::Up4Star(index)
                    }
                    UpType::NonUp => {
                        match self.get_4star_balance_type() {
                            ItemType::Character => {
                                WeaponGachaType::Other4StarCharacter
                            }
                            ItemType::Weapon => {
                                WeaponGachaType::Other4StarWeapon
                            }
                        }
                    }
                }
            }
            ItemLevel::Star5 => {
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;

                match self.want_5star_state {
                    WantState::Want(index, Self::WANT_5STAR_MAX) => {
                        self.last_5star_is_up = true;
                        self.want_5star_state = WantState::Want(index, 0);
                        WeaponGachaType::Up5Star(index)
                    }
                    WantState::Want(want_index, curr) => {
                        match self.get_5star_up_type() {
                            UpType::Up(index) => {
                                self.want_5star_state = WantState::Want(want_index, if index == want_index {0} else {curr + 1});
                                WeaponGachaType::Up5Star(index)
                            }
                            UpType::NonUp => {
                                self.want_5star_state = WantState::Want(want_index, curr + 1);
                                WeaponGachaType::Other5Star
                            }
                        }
                    }
                    WantState::None => {
                        match self.get_5star_up_type() {
                            UpType::Up(index) => {
                                WeaponGachaType::Up5Star(index)
                            }
                            UpType::NonUp => {
                                WeaponGachaType::Other5Star
                            }
                        }
                    }
                }
            }
        }
    }
}

impl CharacterGachaType {
    pub fn get_item_name(& self, pool: & CharacterPool) -> String {
        match self {
            Self::Up5Star => format!("\x1b[01m\x1b[38;2;186;106;53m{}\x1b[0m\x1b[0m", pool.up_5star),
            Self::Up4Star(index) => format!("\x1b[01m\x1b[38;2;160;90;215m{}\x1b[0m\x1b[0m\x1b[0m", pool.up_4star[*index as usize]),
            Self::Other5Star => format!("\x1b[01m\x1b[38;2;186;106;53m{}\x1b[0m\x1b[0m", pool.other_5star[rand::thread_rng().gen_range(0..pool.other_5star.len())]),
            Self::Other4StarCharacter => format!("\x1b[01m\x1b[38;2;160;90;215m{}\x1b[0m\x1b[0m", pool.other_4star_character[rand::thread_rng().gen_range(0..pool.other_4star_character.len())]),
            Self::Other4StarWeapon => format!("\x1b[01m\x1b[38;2;160;90;215m{}\x1b[0m\x1b[0m", pool.other_4star_weapon[rand::thread_rng().gen_range(0..pool.other_4star_weapon.len())]),
            Self::Other3Star => format!("{}", pool.other_3star[rand::thread_rng().gen_range(0..pool.other_3star.len())]),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharacterPool {
    up_5star: String,
    up_4star: Vec<String>,
    other_5star: Vec<String>,
    other_4star_character: Vec<String>,
    other_4star_weapon: Vec<String>,
    other_3star: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Pools {
    pub character1: CharacterPool,
    pub character2: CharacterPool,
}
