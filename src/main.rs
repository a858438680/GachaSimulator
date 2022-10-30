mod gacha_traits;
mod gacha_impl;

use std::{io, fs};
use clap::Parser;

use gacha_traits::*;
use gacha_impl::*;

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
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
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
    } else {
        println!("Simulating {} times...", args.num_sim);

        let mut character_gacha_state = CharacterGachaState::new();
        let mut weapon_gacha_state = WeaponGachaState::new();

        let mut character_up_5star_count = 0u32;
        let mut character_5star_count = 0u32;
        let mut weapon_want_5star_count = 0u32;
        let mut weapon_up_5star_count = 0u32;
        let mut weapon_5star_count = 0u32;

        for _ in 1..args.num_sim {
            let result = character_gacha_state.simulate_character_gacha();
            match result {
                CharacterGachaType::Up5Star => {
                    character_up_5star_count += 1;
                    character_5star_count += 1;
                }
                CharacterGachaType::Other5Star => {
                    character_5star_count += 1;
                }
                _ => {}
            }
        }

        for _ in 1..args.num_sim {
            let result = weapon_gacha_state.simulate_weapon_gacha();
            match result {
                WeaponGachaType::Up5Star(0) => {
                    weapon_want_5star_count += 1;
                    weapon_up_5star_count += 1;
                    weapon_5star_count += 1;
                }
                WeaponGachaType::Up5Star(1) => {
                    weapon_up_5star_count += 1;
                    weapon_5star_count += 1;
                }
                WeaponGachaType::Other5Star => {
                    weapon_5star_count += 1;
                }
                _ => {}
            }
        }

        println!("Character up 5 star probability: {}%", character_up_5star_count as f64 * 100. / args.num_sim as f64);
        println!("Character 5 star probability: {}%", character_5star_count as f64 * 100. / args.num_sim as f64);
        println!("Weapon want 5 star probability: {}%", weapon_want_5star_count as f64 * 100. / args.num_sim as f64);
        println!("Weapon up 5 star probability: {}%", weapon_up_5star_count as f64 * 100. / args.num_sim as f64);
        println!("Weapon 5 star probability: {}%", weapon_5star_count as f64 * 100. / args.num_sim as f64);
    }
}
