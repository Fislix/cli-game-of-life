use cli_game_of_life::*;

fn main() {
    let cli = Cli::parse();
    let (rows, cols) = (cli.height as usize, cli.width as usize);

    let mut world = generate_world(rows, cols, cli.portion_alive);
    let mut buffer: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    let should_stop = get_stop_condition(cli.seconds);

    run(&mut world, &mut buffer, rows, cols, should_stop);

    reset_console();
}
