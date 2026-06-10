mod logger;
mod matrix;
mod pattern;

use clap::Parser;
use matrix::Matrix;

#[derive(Parser)]
#[command(name = "matrix", about = "matrix rain effect")]
pub struct Args {
    /// enable rainbow mode
    #[arg(short, long)]
    pub rainbow: bool,

    /// enable static rainbow mode
    #[arg(long)]
    pub static_rainbow: bool,

    /// set a single color (red, green, yellow, blue, magenta, cyan, white, grey, darkred, darkgreen, darkyellow, darkblue, darkmagenta, darkcyan)
    #[arg(short, long)]
    pub color: Option<String>,

    /// set two colors alternating (e.g. --duo green cyan)
    #[arg(long, num_args = 2)]
    pub duo: Option<Vec<String>>,

    /// streak contains the same characters the entire time
    #[arg(long)]
    pub set_characters: bool,

    /// replaces all characters with lambda symbol
    #[arg(long)]
    pub lambda: bool,

    /// speed in milliseconds between updates
    #[arg(short, long, default_value_t = 35)]
    pub speed: u64,

    /// spawn rate of streaks default is 1
    #[arg(long, default_value_t = 1.)]
    pub spawn: f64,
}

fn main() {
    let args = Args::parse();

    let mut matrix = Matrix::new(args);

    matrix.run();
}
