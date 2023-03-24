use cli_game_of_life::*;

fn main() -> Result<(), &'static str> {
    let cli = Cli::parse();
    let mut game = create_game(cli.file, cli.width, cli.height, cli.portion_alive)?;
    let should_stop = get_stop_condition(cli.seconds, cli.steps);

    run(
        &mut game,
        should_stop,
        cli.steps_per_second,
        cli.char_alive,
        cli.char_dead,
    );
    reset_console();

    Ok(())
}

fn create_game(
    file: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    portion_alive: f64,
) -> Result<Game, &'static str> {
    match (file, width, height) {
        (Some(file), None, None) => {
            let file_data = read_file(&file)?;
            Ok(Game::from_starting_data(file_data))
        }
        (None, Some(width), Some(height)) => {
            let (rows, cols) = (height as usize, width as usize);
            Ok(Game::new(rows, cols, portion_alive))
        }
        (_, _, _) => Err("Invalid combination of arguments (or missing). Must either use both <WIDTH> and <HEIGHT> or only <FILE>.")
    }
}
