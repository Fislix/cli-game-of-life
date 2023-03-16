use cli_game_of_life::{do_step, print_world, reset_console, Cli, Parser};

fn main() {
    let cli = Cli::parse();

    let (rows, cols) = (cli.height as usize, cli.width as usize);
    let mut world = vec![vec![0; cols]; rows];

    let mut buffer: Vec<Vec<u8>> = Vec::with_capacity(rows);
    for _ in 0..rows {
        buffer.push(vec![0; cols]);
    }

    for _ in 0..3 {
        reset_console();
        print_world(&world);
        do_step(&mut world, &mut buffer, rows, cols);
        (world, buffer) = (buffer, world);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    reset_console();
}
