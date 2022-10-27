use std::{io, cmp::Ordering};
use rand::Rng;

trait Gacha5StarInfo {
    const BASE_PROBABILITY_5STAR: f64;
    const THRESHOLD_5STAR: u32;
    const MAX_5STAR: u32;
    const RATIO_5STAR: f64 = (1. - Self::BASE_PROBABILITY_5STAR) / (Self::MAX_5STAR - Self::THRESHOLD_5STAR) as f64;
}

trait Gacha4StarInfo {
    const BASE_PROBABILITY_4STAR: f64;
    const THRESHOLD_4STAR: u32;
    const MAX_4STAR: u32;
    const RATIO_4STAR: f64 = (1. - Self::BASE_PROBABILITY_4STAR) / (Self::MAX_4STAR - Self::THRESHOLD_4STAR) as f64;
}

enum CharacterGachaType {
    Up5Star,
    Up4Star1,
    Up4Star2,
    Up4Star3,
    Other5Star,
    Other4StarCharacter,
    Other4StarWeapon,
    Other3Star,
}

enum WeaponGachaType {
    Up5Star1,
    Up5Star2,
    Up4Star1,
    Up4Star2,
    Up4Star3,
    Up4Star4,
    Up4Star5,
    Other5Star,
    Other4StarCharacter,
    Other4StarWeapon,
    Other3Star,
}

struct CharacterGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    last_5star_is_up: bool,
    last_4star_is_up: bool,
}

struct WeaponGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    last_5star_is_up: bool,
    last_4star_is_up: bool,
    last_5star_is_wanted: bool,
}

impl Gacha5StarInfo for CharacterGachaState {
    const BASE_PROBABILITY_5STAR: f64 = 0.006;
    const THRESHOLD_5STAR: u32 = 73;
    const MAX_5STAR: u32 = 90;
}

impl Gacha5StarInfo for WeaponGachaState {
    const BASE_PROBABILITY_5STAR: f64 = 0.007;
    const THRESHOLD_5STAR: u32 = 62;
    const MAX_5STAR: u32 = 80;
}

impl Gacha4StarInfo for CharacterGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.51;
    const THRESHOLD_4STAR: u32 = 8;
    const MAX_4STAR: u32 = 10;
}

impl Gacha4StarInfo for WeaponGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.6;
    const THRESHOLD_4STAR: u32 = 7;
    const MAX_4STAR: u32 = 10;
}

impl CharacterGachaState {
    fn simulate_character_gacha(&mut self) -> CharacterGachaType {
        let count_5star = self.since_last_5star + 1;
        let count_4star = self.since_last_4star + 1;

        let p_5star = if count_5star > Self::THRESHOLD_5STAR {
            Self::BASE_PROBABILITY_5STAR + Self::RATIO_5STAR * (count_5star - Self::THRESHOLD_5STAR) as f64
        } else {
            Self::BASE_PROBABILITY_5STAR
        };

        let p_4star = if count_4star > Self::THRESHOLD_4STAR {
            Self::BASE_PROBABILITY_4STAR + Self::RATIO_4STAR * (count_4star - Self::THRESHOLD_4STAR) as f64
        } else {
            Self::BASE_PROBABILITY_4STAR
        };

        let rnd_num: f64 = rand::random();
        if rnd_num < p_5star {
            if self.last_5star_is_up {
                let get_up: bool = rand::random();
                if get_up {
                    self.since_last_5star = 0;
                    self.since_last_4star += 1;
                    self.last_5star_is_up = true;
                    CharacterGachaType::Up5Star
                } else {
                    self.since_last_5star = 0;
                    self.since_last_4star += 1;
                    self.last_5star_is_up = false;
                    CharacterGachaType::Other5Star
                }
            } else {
                self.since_last_5star = 0;
                self.since_last_4star += 1;
                self.last_5star_is_up = true;
                CharacterGachaType::Up5Star
            }
        } else if rnd_num < p_5star + p_4star {
            if self.last_4star_is_up {
                let get_up: bool = rand::random();
                if get_up {
                    self.since_last_5star += 1;
                    self.since_last_4star = 0;
                    self.last_4star_is_up = true;
                    
                }
            }
        } else {
            CharacterGachaType::Other3Star
        }
    }
}

impl WeaponGachaState {
    fn simulate_weapon_gacha(&mut self) -> WeaponGachaType {
        WeaponGachaType::Other3Star
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => { num }
            Err(_) => { continue }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("Too small!") }
            Ordering::Greater => { println!("Too big!") }
            Ordering::Equal => {
                println!("You win!");
                break
            }
        }
    }
}
