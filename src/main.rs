mod gacha_traits;
mod gacha_impl;
mod user_interface;
mod common;

use clap::Parser;

use gacha_traits::gacha_enums::*;
use gacha_impl::*;
use common::Arguments;
use user_interface::*;

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
