use std::{io, cmp::Ordering, fs};
use rand::Rng;
use clap::Parser;
use serde::{Deserialize, Serialize};

trait Gacha5StarInfo {
    const BASE_PROBABILITY_5STAR: f64;
    const THRESHOLD_5STAR: u32;
    const MAX_5STAR: u32;

    fn get_since_last_5star(&self) -> u32;

    fn prob_5star(count: u32) -> f64 {
        if count > Self::THRESHOLD_5STAR {
            Self::BASE_PROBABILITY_5STAR * (1. + 10. * (count - Self::THRESHOLD_5STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_5STAR
        }
    }
}

trait Gacha4StarInfo {
    const BASE_PROBABILITY_4STAR: f64;
    const THRESHOLD_4STAR: u32;
    const MAX_4STAR: u32;

    fn get_since_last_4star(&self) -> u32;

    fn prob_4star(count: u32) -> f64 {
        if count > Self::THRESHOLD_4STAR {
            Self::BASE_PROBABILITY_4STAR * (1. + 10. * (count - Self::THRESHOLD_4STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_4STAR
        }
    }
}

trait Up5Star {
    const UP_PROBABILITY_5STAR: f64;
    const UP_5STAR_NUM: u32;

    fn get_last_5star_is_up(&self) -> bool;

    fn get_5star_up_type(&self) -> UpType {
        if self.get_last_5star_is_up() {
            if rand::random::<f64>() < Self::UP_PROBABILITY_5STAR {
                if Self::UP_5STAR_NUM == 1 {
                    UpType::Up(0)
                } else {
                    UpType::Up(rand::thread_rng().gen_range(0..Self::UP_5STAR_NUM))
                }
            } else {
                UpType::NonUp
            }
        } else {
            if Self::UP_5STAR_NUM == 1 {
                UpType::Up(0)
            } else {
                UpType::Up(rand::thread_rng().gen_range(0..Self::UP_5STAR_NUM))
            }
        }
    }
}

trait Up4Star {
    const UP_PROBABILITY_4STAR: f64;
    const UP_4STAR_NUM: u32;

    fn get_last_4star_is_up(&self) -> bool;

    fn get_4star_up_type(&self) -> UpType {
        if self.get_last_4star_is_up() {
            if rand::random::<f64>() < Self::UP_PROBABILITY_4STAR {
                if Self::UP_4STAR_NUM == 1 {
                    UpType::Up(0)
                } else {
                    UpType::Up(rand::thread_rng().gen_range(0..Self::UP_4STAR_NUM))
                }
            } else {
                UpType::NonUp
            }
        } else {
            if Self::UP_4STAR_NUM == 1 {
                UpType::Up(0)
            } else {
                UpType::Up(rand::thread_rng().gen_range(0..Self::UP_4STAR_NUM))
            }
        }
    }
}

trait Want5Star: Gacha5StarInfo {
    const WANT_5STAR_MAX: u32;
}

trait Balance5Star: Gacha5StarInfo {
    const BALANCE_THRESHOLD_5STAR: u32;

    fn get_since_last_5star_character(&self) -> u32;

    fn get_since_last_5star_weapon(&self) -> u32;

    fn balance_prob_5star(count: u32) -> f64 {
        if count >  Self::BALANCE_THRESHOLD_5STAR {
            Self::BASE_PROBABILITY_5STAR * 0.5 * (1. + 10. * (count - Self::BALANCE_THRESHOLD_5STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_5STAR * 0.5
        }
    }

    fn get_5star_balance_type(&self) -> ItemType {
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
            true => ItemType::Character,
            false => ItemType::Weapon,
        }
    }
}

trait Balance4Star: Gacha4StarInfo {
    const BALANCE_THRESHOLD_4STAR: u32;

    fn get_since_last_4star_character(&self) -> u32;

    fn get_since_last_4star_weapon(&self) -> u32;

    fn balance_prob_4star(count: u32) -> f64 {
        if count >  Self::BALANCE_THRESHOLD_4STAR {
            Self::BASE_PROBABILITY_4STAR * 0.5 * (1. + 10. * (count - Self::BALANCE_THRESHOLD_4STAR) as f64)
        } else {
            Self::BASE_PROBABILITY_4STAR * 0.5
        }
    }

    fn get_4star_balance_type(&self) -> ItemType {
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
            true => ItemType::Character,
            false => ItemType::Weapon,
        }
    }
}

trait GeneralGachaMechanism: Gacha5StarInfo + Gacha4StarInfo {
    fn get_item_level(&self) -> ItemLevel {
        let count_5star = self.get_since_last_5star() + 1;
        let count_4star = self.get_since_last_4star() + 1;
        let p_5star = Self::prob_5star(count_5star);
        let p_4star = Self::prob_4star(count_4star);

        let rnd_num: f64 = rand::random();
        if rnd_num < p_5star {
            ItemLevel::Star5
        } else if rnd_num < p_5star + p_4star {
            ItemLevel::Star4
        } else {
            ItemLevel::Star3
        }
    }
}

enum ItemLevel {
    Star3,
    Star4,
    Star5,
}

enum ItemType {
    Character,
    Weapon,
}

enum UpType {
    Up(u32),
    NonUp
}

enum WantState {
    Want(u32, u32),
    None
}

enum NormalGachaType {
    Character5Star,
    Weapon5Star,
    Character4Star,
    Weapon4Star,
    Other3Star,
}

enum CharacterGachaType {
    Up5Star,
    Up4Star(u32),
    Other5Star,
    Other4StarCharacter,
    Other4StarWeapon,
    Other3Star,
}

enum WeaponGachaType {
    Up5Star(u32),
    Up4Star(u32),
    Other5Star,
    Other4StarCharacter,
    Other4StarWeapon,
    Other3Star,
}

struct NormalGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    since_last_5star_character: u32,
    since_last_5star_weapon: u32,
    since_last_4star_character: u32,
    since_last_4star_weapon: u32,
}

struct CharacterGachaState {
    since_last_5star: u32,
    since_last_4star: u32,
    since_last_4star_character: u32,
    since_last_4star_weapon: u32,
    last_5star_is_up: bool,
    last_4star_is_up: bool,
}

struct WeaponGachaState {
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
}

impl Gacha4StarInfo for NormalGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.051;
    const THRESHOLD_4STAR: u32 = 8;
    const MAX_4STAR: u32 = 10;

    fn get_since_last_4star(&self) -> u32 {
        self.since_last_4star
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
}

impl Balance4Star for NormalGachaState {
    const BALANCE_THRESHOLD_4STAR: u32 = 17;

    fn get_since_last_4star_character(&self) -> u32 {
        self.since_last_4star_character
    }
    
    fn get_since_last_4star_weapon(&self) -> u32 {
        self.since_last_4star_weapon
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
}

impl Gacha4StarInfo for CharacterGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.051;
    const THRESHOLD_4STAR: u32 = 8;
    const MAX_4STAR: u32 = 10;

    fn get_since_last_4star(&self) -> u32 {
        self.since_last_4star
    }
}

impl Up5Star for CharacterGachaState {
    const UP_PROBABILITY_5STAR: f64 = 0.5;
    const UP_5STAR_NUM: u32 = 1;

    fn get_last_5star_is_up(&self) -> bool {
        self.last_5star_is_up
    }
}

impl Up4Star for CharacterGachaState {
    const UP_PROBABILITY_4STAR: f64 = 0.5;
    const UP_4STAR_NUM: u32 = 3;

    fn get_last_4star_is_up(&self) -> bool {
        self.last_4star_is_up
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
}

impl GeneralGachaMechanism for CharacterGachaState {}

impl Gacha5StarInfo for WeaponGachaState {
    const BASE_PROBABILITY_5STAR: f64 = 0.007;
    const THRESHOLD_5STAR: u32 = 62;
    const MAX_5STAR: u32 = 80;

    fn get_since_last_5star(&self) -> u32 {
        self.since_last_5star
    }
}

impl Gacha4StarInfo for WeaponGachaState {
    const BASE_PROBABILITY_4STAR: f64 = 0.06;
    const THRESHOLD_4STAR: u32 = 7;
    const MAX_4STAR: u32 = 10;

    fn get_since_last_4star(&self) -> u32 {
        self.since_last_4star
    }
}

impl Up5Star for WeaponGachaState {
    const UP_PROBABILITY_5STAR: f64 = 0.75;
    const UP_5STAR_NUM: u32 = 2;

    fn get_last_5star_is_up(&self) -> bool {
        self.last_5star_is_up
    }
}

impl Up4Star for WeaponGachaState {
    const UP_PROBABILITY_4STAR: f64 = 0.75;
    const UP_4STAR_NUM: u32 = 5;

    fn get_last_4star_is_up(&self) -> bool {
        self.last_4star_is_up
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
}

impl GeneralGachaMechanism for WeaponGachaState {}

impl NormalGachaState {
    fn simulate_normal_gacha(&mut self) -> NormalGachaType {
        match self.get_item_level() {
            ItemLevel::Star3 => {
                self.since_last_5star += 1;
                self.since_last_4star += 1;
                self.since_last_5star_character += 1;
                self.since_last_5star_weapon += 1;
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
                NormalGachaType::Other3Star
            }
            ItemLevel::Star4 => {
                self.since_last_5star += 1;
                self.since_last_4star = 0;
                self.since_last_5star_character += 1;
                self.since_last_5star_weapon += 1;
    
                match self.get_4star_balance_type() {
                    ItemType::Character => {
                        self.since_last_4star_character = 0;
                        self.since_last_4star_weapon += 1;
                        NormalGachaType::Character4Star
                    }
                    ItemType::Weapon => {
                        self.since_last_4star_character += 1;
                        self.since_last_4star_weapon = 0;
                        NormalGachaType::Weapon4Star
                    }
                }
            }
            ItemLevel::Star5 => {
                self.since_last_5star = 0;
                self.since_last_4star += 1;
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
    
                match self.get_5star_balance_type() {
                    ItemType::Character => {
                        self.since_last_5star_character = 0;
                        self.since_last_5star_weapon += 1;
                        NormalGachaType::Character5Star
                    }
                    ItemType::Weapon => {
                        self.since_last_5star_character += 1;
                        self.since_last_5star_weapon = 0;
                        NormalGachaType::Weapon5Star
                    }
                }
            }
        }
    }
}

impl CharacterGachaState {
    fn new() -> CharacterGachaState {
        CharacterGachaState {
            since_last_5star: 0,
            since_last_4star: 0,
            since_last_4star_character: 0,
            since_last_4star_weapon: 0,
            last_5star_is_up: true,
            last_4star_is_up: true,
        }
    }

    fn simulate_character_gacha(&mut self) -> CharacterGachaType {
        match self.get_item_level() {
            ItemLevel::Star3 => {
                self.since_last_5star += 1;
                self.since_last_4star += 1;
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
                CharacterGachaType::Other3Star
            }
            ItemLevel::Star4 => {
                self.since_last_5star += 1;
                self.since_last_4star = 0;

                match self.get_4star_up_type() {
                    UpType::Up(index) => {
                        self.last_4star_is_up = true;
                        self.since_last_4star_character = 0;
                        self.since_last_4star_weapon += 1;
                        CharacterGachaType::Up4Star(index)
                    }
                    UpType::NonUp => {
                        self.last_4star_is_up = false;
                        match self.get_4star_balance_type() {
                            ItemType::Character => {
                                self.since_last_4star_character = 0;
                                self.since_last_4star_weapon += 1;
                                CharacterGachaType::Other4StarCharacter
                            }
                            ItemType::Weapon => {
                                self.since_last_4star_character += 1;
                                self.since_last_4star_weapon = 0;
                                CharacterGachaType::Other4StarWeapon
                            }
                        }
                    }
                }
            }
            ItemLevel::Star5 => {
                self.since_last_5star = 0;
                self.since_last_4star += 1;
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;

                match self.get_5star_up_type() {
                    UpType::Up(_) => {
                        self.last_5star_is_up = true;
                        CharacterGachaType::Up5Star
                    }
                    UpType::NonUp => {
                        self.last_5star_is_up = false;
                        CharacterGachaType::Other5Star
                    }
                }
            }
        }
    }
}

impl WeaponGachaState {
    fn new() -> WeaponGachaState {
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

    fn simulate_weapon_gacha(&mut self) -> WeaponGachaType {
        match self.get_item_level() {
            ItemLevel::Star3 => {
                self.since_last_5star += 1;
                self.since_last_4star += 1;
                self.since_last_4star_character += 1;
                self.since_last_4star_weapon += 1;
                WeaponGachaType::Other3Star
            }
            ItemLevel::Star4 => {
                self.since_last_5star += 1;
                self.since_last_4star = 0;

                match self.get_4star_up_type() {
                    UpType::Up(index) => {
                        self.last_4star_is_up = true;
                        self.since_last_4star_character += 0;
                        self.since_last_4star_weapon = 0;
                        WeaponGachaType::Up4Star(index)
                    }
                    UpType::NonUp => {
                        self.last_4star_is_up = false;
                        match self.get_4star_balance_type() {
                            ItemType::Character => {
                                self.since_last_4star_character = 0;
                                self.since_last_4star_weapon += 1;
                                WeaponGachaType::Other4StarCharacter
                            }
                            ItemType::Weapon => {
                                self.since_last_4star_character += 1;
                                self.since_last_4star_weapon = 0;
                                WeaponGachaType::Other4StarWeapon
                            }
                        }
                    }
                }
            }
            ItemLevel::Star5 => {
                self.since_last_5star = 0;
                self.since_last_4star += 1;
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
                                self.last_5star_is_up = true;
                                self.want_5star_state = WantState::Want(want_index, if index == want_index {0} else {curr + 1});
                                WeaponGachaType::Up5Star(index)
                            }
                            UpType::NonUp => {
                                self.last_5star_is_up = false;
                                self.want_5star_state = WantState::Want(want_index, curr + 1);
                                WeaponGachaType::Other5Star
                            }
                        }
                    }
                    WantState::None => {
                        match self.get_5star_up_type() {
                            UpType::Up(index) => {
                                self.last_5star_is_up = true;
                                WeaponGachaType::Up5Star(index)
                            }
                            UpType::NonUp => {
                                self.last_5star_is_up = false;
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
    fn get_item_name(& self, pool: & CharacterPool) -> String {
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

/// Genshin Impact Gacha Simulator @LI Runzhong
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Number of times to simulate gacha
    #[arg(short, long, default_value_t = 10000)]
    num_sim: u32,

    /// File path of the gacha pool configuration file
    #[arg(short, long, default_value_t = String::from("pool.json"))]
    file_path: String,

    /// Interactive mode
    #[arg(short, long, default_value_t = true)]
    interactive: bool,
}

#[derive(Serialize, Deserialize)]
struct CharacterPool {
    up_5star: String,
    up_4star: Vec<String>,
    other_5star: Vec<String>,
    other_4star_character: Vec<String>,
    other_4star_weapon: Vec<String>,
    other_3star: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Pools {
    character1: CharacterPool,
    character2: CharacterPool,
}

fn interactive_simulate(args: &Arguments) {
    let pool_config = fs::read_to_string(&args.file_path)
        .expect(&format!("Unable to read file: {}", args.file_path));
    let pool_config: Pools = serde_json::from_str(&pool_config)
        .expect("Unable to parse json");
    let mut character_gacha_state = CharacterGachaState::new();
    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num)
            .expect("Unable to read line");
        if num.trim().starts_with("q") {
            break
        }
        let num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(e) => continue
        };
        for _ in 0..num {
            let result = character_gacha_state.simulate_character_gacha();
            let name = result.get_item_name(&pool_config.character2);
            print!("{} ", name);
        }
        println!();
    }
}

fn main() {
    let args = Arguments::parse();

    println!("Genshin Impact Gacha Simulator @LI Runzhong");
    if args.interactive {
        interactive_simulate(&args);
    }
    // println!("Simulating {} times...", args.num_sim);

    // let mut character_gacha_state = CharacterGachaState::new();
    // let mut weapon_gacha_state = WeaponGachaState::new();

    // let mut character_up_5star_count = 0u32;
    // let mut character_5star_count = 0u32;
    // let mut weapon_want_5star_count = 0u32;
    // let mut weapon_up_5star_count = 0u32;
    // let mut weapon_5star_count = 0u32;

    // for _ in 1..args.num_sim {
    //     let result = character_gacha_state.simulate_character_gacha();
    //     match result {
    //         CharacterGachaType::Up5Star => {
    //             character_up_5star_count += 1;
    //             character_5star_count += 1;
    //         }
    //         CharacterGachaType::Other5Star => {
    //             character_5star_count += 1;
    //         }
    //         _ => {}
    //     }
    // }

    // for _ in 1..args.num_sim {
    //     let result = weapon_gacha_state.simulate_weapon_gacha();
    //     match result {
    //         WeaponGachaType::Up5Star(0) => {
    //             weapon_want_5star_count += 1;
    //             weapon_up_5star_count += 1;
    //             weapon_5star_count += 1;
    //         }
    //         WeaponGachaType::Up5Star(1) => {
    //             weapon_up_5star_count += 1;
    //             weapon_5star_count += 1;
    //         }
    //         WeaponGachaType::Other5Star => {
    //             weapon_5star_count += 1;
    //         }
    //         _ => {}
    //     }
    // }

    // println!("Character up 5 star probability: {}%", character_up_5star_count as f64 * 100. / args.num_sim as f64);
    // println!("Character 5 star probability: {}%", character_5star_count as f64 * 100. / args.num_sim as f64);
    // println!("Weapon want 5 star probability: {}%", weapon_want_5star_count as f64 * 100. / args.num_sim as f64);
    // println!("Weapon up 5 star probability: {}%", weapon_up_5star_count as f64 * 100. / args.num_sim as f64);
    // println!("Weapon 5 star probability: {}%", weapon_5star_count as f64 * 100. / args.num_sim as f64);
}
