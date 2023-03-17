use cli_game_of_life::{do_step, generate_world, print_world, reset_console, Cli, Parser};

fn main() {
    let cli = Cli::parse();

    let (rows, cols) = (cli.height as usize, cli.width as usize);
    let mut world = generate_world(rows, cols, cli.portion_alive);
    let mut buffer: Vec<Vec<u8>> = vec![vec![0; cols]; rows];

    for _ in 0..3 {
        reset_console();
        print_world(&world);
        do_step(&mut world, &mut buffer, rows, cols);
        (world, buffer) = (buffer, world);
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    reset_console();
}
