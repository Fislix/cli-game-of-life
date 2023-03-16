const ANSI_CLEAR_CONSOLE: &str = "\x1b[2J";
const ANSI_CURSOR_TO_START: &str = "\x1b[1;1H";

pub fn reset_console() {
    print!("{ANSI_CLEAR_CONSOLE}{ANSI_CURSOR_TO_START}");
}

pub fn print_world(world: &Vec<Vec<u8>>) {
    for row in world.iter() {
        for cell in row {
            print!("{} ", match cell {
                0 => " ",
                1 => "■",
                n => panic!("Invalid cell value: {}", n),
            });
        }
        println!();
    }
}