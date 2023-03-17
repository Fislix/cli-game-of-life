pub use clap::Parser;

const ANSI_CLEAR_CONSOLE: &str = "\x1b[2J";
const ANSI_CURSOR_TO_START: &str = "\x1b[1;1H";

pub fn reset_console() {
    print!("{ANSI_CLEAR_CONSOLE}{ANSI_CURSOR_TO_START}");
}

pub fn print_world(world: &[Vec<u8>]) {
    for row in world.iter() {
        for cell in row {
            print!(
                "{} ",
                match cell {
                    0 => " ",
                    1 => "■",
                    n => panic!("Invalid cell value: {}", n),
                }
            );
        }
        println!();
    }
}

#[derive(Parser)]
#[command(about = "Simulates Conway's Game of Life in the terminal.", long_about = None)]
#[command(group(
    clap::ArgGroup::new("duration")
        .args(["seconds", "steps"])
))]
pub struct Cli {
    /// World width in number of characters
    pub width: u32,

    /// World height in number of lines
    pub height: u32,

    /// What portion of the cells should be alive in the randomly generated world
    #[arg(short, long, default_value_t = 0.3)]
    pub portion_alive: f64,

    /// How many seconds the game should run for
    #[arg(long)]
    pub seconds: Option<u32>,

    /// How many simulation steps the game should run for
    #[arg(long)]
    pub steps: Option<u32>,

    /// How many steps per second should (tried to) be computed
    #[arg(short, long = "speed", default_value_t = 2)]
    pub steps_per_second: u32,
}
