use cli_game_of_life::*;

fn main() {
    let cli = Cli::parse();
    let (rows, cols) = (cli.height as usize, cli.width as usize);

    let mut game = Game::new(rows, cols, cli.portion_alive);
    let should_stop = get_stop_condition(cli.seconds, cli.steps);

    run(&mut game, should_stop);

    reset_console();
}
