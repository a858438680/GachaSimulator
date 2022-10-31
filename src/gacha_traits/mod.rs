pub mod gacha_enums;

use gacha_enums::*;
use rand::Rng;
use std::cmp::Ordering;

pub trait Gacha5StarInfo {
    const BASE_PROBABILITY_5STAR: f64;
    const THRESHOLD_5STAR: u32;
    const MAX_5STAR: u32;

    fn get_since_last_5star(&self) -> u32;

    fn set_since_last_5star(&mut self, count: u32);

    fn prob_5star(count: u32) -> f64 {
        if count > Self::THRESHOLD_5STAR {
            Self::BASE_PROBABILITY_5STAR * (1. + 10. * (count - Self::THRESHOLD_5STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_5STAR
        }
    }
}

pub trait Gacha4StarInfo {
    const BASE_PROBABILITY_4STAR: f64;
    const THRESHOLD_4STAR: u32;
    const MAX_4STAR: u32;

    fn get_since_last_4star(&self) -> u32;

    fn set_since_last_4star(&mut self, count: u32);

    fn prob_4star(count: u32) -> f64 {
        if count > Self::THRESHOLD_4STAR {
            Self::BASE_PROBABILITY_4STAR * (1. + 10. * (count - Self::THRESHOLD_4STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_4STAR
        }
    }
}

pub trait Up5Star {
    const UP_PROBABILITY_5STAR: f64;
    const UP_5STAR_NUM: u32;

    fn get_last_5star_is_up(&self) -> bool;

    fn set_last_5star_is_up(&mut self, is_up: bool);

    fn get_5star_up_type(&mut self) -> UpType {
        if self.get_last_5star_is_up() {
            if rand::random::<f64>() < Self::UP_PROBABILITY_5STAR {
                self.set_last_5star_is_up(true);
                if Self::UP_5STAR_NUM == 1 {
                    UpType::Up(0)
                } else {
                    UpType::Up(rand::thread_rng().gen_range(0..Self::UP_5STAR_NUM))
                }
            } else {
                self.set_last_5star_is_up(false);
                UpType::NonUp
            }
        } else {
            self.set_last_5star_is_up(true);
            if Self::UP_5STAR_NUM == 1 {
                UpType::Up(0)
            } else {
                UpType::Up(rand::thread_rng().gen_range(0..Self::UP_5STAR_NUM))
            }
        }
    }
}

pub trait Up4Star {
    const UP_PROBABILITY_4STAR: f64;
    const UP_4STAR_NUM: u32;

    fn get_last_4star_is_up(&self) -> bool;

    fn set_last_4star_is_up(&mut self, is_up: bool);

    fn get_4star_up_type(&mut self) -> UpType {
        if self.get_last_4star_is_up() {
            if rand::random::<f64>() < Self::UP_PROBABILITY_4STAR {
                self.set_last_4star_is_up(true);
                if Self::UP_4STAR_NUM == 1 {
                    UpType::Up(0)
                } else {
                    UpType::Up(rand::thread_rng().gen_range(0..Self::UP_4STAR_NUM))
                }
            } else {
                self.set_last_4star_is_up(false);
                UpType::NonUp
            }
        } else {
            self.set_last_4star_is_up(true);
            if Self::UP_4STAR_NUM == 1 {
                UpType::Up(0)
            } else {
                UpType::Up(rand::thread_rng().gen_range(0..Self::UP_4STAR_NUM))
            }
        }
    }
}

pub trait Want5Star: Gacha5StarInfo {
    const WANT_5STAR_MAX: u32;
}

pub trait Balance5Star: Gacha5StarInfo {
    const BALANCE_THRESHOLD_5STAR: u32;

    fn get_since_last_5star_character(&self) -> u32;

    fn get_since_last_5star_weapon(&self) -> u32;

    fn set_since_last_5star_character(&mut self, count: u32);

    fn set_since_last_5star_weapon(&mut self, count: u32);

    fn balance_prob_5star(count: u32) -> f64 {
        if count >  Self::BALANCE_THRESHOLD_5STAR {
            Self::BASE_PROBABILITY_5STAR * 0.5 * (1. + 10. * (count - Self::BALANCE_THRESHOLD_5STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_5STAR * 0.5
        }
    }

    fn get_5star_balance_type(&mut self) -> ItemType {
        let count_character = self.get_since_last_5star_character() + 1;
        let count_weapon = self.get_since_last_5star_weapon() + 1;
        let p_character = Self::balance_prob_5star(count_character);
        let p_weapon = Self::balance_prob_5star(count_weapon);

        let get_character = match p_character.partial_cmp(&p_weapon) {
            Some(Ordering::Equal) => rand::random::<bool>(),
            Some(Ordering::Less) => rand::random::<f64>() * (p_character + p_weapon).min(1.) >= p_weapon,
            Some(Ordering::Greater) => rand::random::<f64>() * (p_character + p_weapon).min(1.) < p_character,
            None => unreachable!()
        };

        match get_character {
            true => {
                self.set_since_last_5star_character(0);
                self.set_since_last_5star_weapon(count_weapon);
                ItemType::Character
            }
            false => {
                self.set_since_last_5star_character(count_character);
                self.set_since_last_5star_weapon(0);
                ItemType::Weapon
            }
        }
    }
}

pub trait Balance4Star: Gacha4StarInfo {
    const BALANCE_THRESHOLD_4STAR: u32;

    fn get_since_last_4star_character(&self) -> u32;

    fn get_since_last_4star_weapon(&self) -> u32;

    fn set_since_last_4star_character(&mut self, count: u32);

    fn set_since_last_4star_weapon(&mut self, count: u32);

    fn balance_prob_4star(count: u32) -> f64 {
        if count >  Self::BALANCE_THRESHOLD_4STAR {
            Self::BASE_PROBABILITY_4STAR * 0.5 * (1. + 10. * (count - Self::BALANCE_THRESHOLD_4STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_4STAR * 0.5
        }
    }

    fn get_4star_balance_type(&mut self) -> ItemType {
        let count_character = self.get_since_last_4star_character() + 1;
        let count_weapon = self.get_since_last_4star_weapon() + 1;
        let p_character = Self::balance_prob_4star(count_character);
        let p_weapon = Self::balance_prob_4star(count_weapon);

        let get_character = match p_character.partial_cmp(&p_weapon) {
            Some(Ordering::Equal) => rand::random::<bool>(),
            Some(Ordering::Less) => rand::random::<f64>() * (p_character + p_weapon).min(1.) >= p_weapon,
            Some(Ordering::Greater) => rand::random::<f64>() * (p_character + p_weapon).min(1.) < p_character,
            None => unreachable!()
        };

        match get_character {
            true => {
                self.set_since_last_4star_character(0);
                self.set_since_last_4star_weapon(count_weapon);
                ItemType::Character
            }
            false => {
                self.set_since_last_4star_character(count_character);
                self.set_since_last_4star_weapon(0);
                ItemType::Weapon
            }
        }
    }
}

pub trait GeneralGachaMechanism: Gacha5StarInfo + Gacha4StarInfo {
    fn get_item_level(&mut self) -> ItemLevel {
        let count_5star = self.get_since_last_5star() + 1;
        let count_4star = self.get_since_last_4star() + 1;
        let p_5star = Self::prob_5star(count_5star);
        let p_4star = Self::prob_4star(count_4star);

        let rnd_num: f64 = rand::random();
        if rnd_num < p_5star {
            self.set_since_last_5star(0);
            self.set_since_last_4star(count_4star);
            ItemLevel::Star5
        } else if rnd_num < p_5star + p_4star {
            self.set_since_last_5star(count_5star);
            self.set_since_last_4star(0);
            ItemLevel::Star4
        } else {
            self.set_since_last_5star(count_5star);
            self.set_since_last_4star(count_4star);
            ItemLevel::Star3
        }
    }
}
