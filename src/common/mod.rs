use clap::Parser;

/// Genshin Impact Gacha Simulator @LI Runzhong
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Number of times to simulate gacha
    #[arg(short, long, default_value_t = 10000)]
    pub num_sim: u32,

    /// File path of the gacha pool configuration file
    #[arg(short, long, default_value_t = String::from("pool.json"))]
    pub file_path: String,

    /// Interactive mode
    #[arg(short, long, default_value_t = false)]
    pub interactive: bool,
}