use super::gacha_traits::*;
use super::common::Arguments;
use super::gacha_impl::*;

use std::{fs, io};

enum InteractiveStage {
    Start,
    RealModeCharacterPool2,
}

struct InteractiveState {
    stage: InteractiveStage,
}

impl InteractiveState {
    fn new() -> InteractiveState {
        InteractiveState {
            stage: InteractiveStage::Start,
        }
    }
}

pub fn interactive_simulate(args: &Arguments) {
    let pool_config = fs::read_to_string(&args.file_path)
        .expect(&format!("Unable to read file: {}", args.file_path));
    let pool_config: Pools = serde_json::from_str(&pool_config)
        .expect("Unable to parse json");
    let mut interactive_state = InteractiveState::new();
    let mut character_gacha_state = CharacterGachaState::new();
    loop {
        match interactive_state.stage {
            InteractiveStage::Start => {
                println!("请选择模拟模式：(1) 真实模式 (2) 概率分析 (q) 退出");
                let mut mode = String::new();
                io::stdin().read_line(&mut mode)
                    .expect("Unable to read line from stdin");
                if mode.trim().starts_with("q") {
                    break;
                }
                match mode.trim().parse() {
                    Ok(1) => {
                        interactive_state.stage = InteractiveStage::RealModeCharacterPool2;
                    }
                    _ => continue
                };
            }
            InteractiveStage::RealModeCharacterPool2 => {
                let mut num = String::new();
                io::stdin().read_line(&mut num)
                    .expect("Unable to read line");
                if num.trim().starts_with("q") {
                    break
                }
                let num: u32 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue
                };
                for _ in 0..num {
                    let result = character_gacha_state.simulate_character_gacha();
                    let name = result.get_item_name(&pool_config.character2);
                    print!("{} ", name);
                }
                println!();
            }
        }
    }
}