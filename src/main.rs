use cli_game_of_life::{do_step, generate_world, print_world, reset_console, Cli, Parser};
use std::time::{Duration, Instant};

fn main() {
    let cli = Cli::parse();
    let (rows, cols) = (cli.height as usize, cli.width as usize);

    let mut world = generate_world(rows, cols, cli.portion_alive);
    let mut buffer: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    let should_stop = get_stop_condition(cli.seconds);

    run(&mut world, &mut buffer, rows, cols, should_stop);

    reset_console();
}

fn run<'a>(
    mut world: &'a mut [Vec<u8>],
    mut buffer: &'a mut [Vec<u8>],
    rows: usize,
    cols: usize,
    should_stop: Box<dyn Fn() -> bool>,
) {
    while !should_stop() {
        reset_console();
        print_world(world);
        do_step(world, buffer, rows, cols);
        (world, buffer) = (buffer, world);
        std::thread::sleep(std::time::Duration::from_secs_f64(0.5));
    }
}

fn get_stop_condition(seconds: Option<u32>) -> Box<dyn Fn() -> bool> {
    match seconds {
        Some(s) => {
            let now = Instant::now();
            let ending_time = now + Duration::from_secs(s.into());
            Box::new(move || Instant::now() > ending_time)
        }
        None => Box::new(|| false),
    }
}
