pub use clap::Parser;

const ANSI_CLEAR_CONSOLE: &str = "\x1b[2J";
const ANSI_CURSOR_TO_START: &str = "\x1b[1;1H";

pub fn reset_console() {
    print!("{ANSI_CLEAR_CONSOLE}{ANSI_CURSOR_TO_START}");
}

pub fn print_world(world: &Vec<Vec<u8>>) {
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
pub struct Cli {
    /// World width in number of characters
    pub width: u32,

    /// World height in number of lines
    pub height: u32,
}
