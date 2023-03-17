mod simulation_logic;

use crate::cli::{print_world, reset_console};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub use simulation_logic::do_step;

pub fn generate_world(rows: usize, cols: usize, portion_alive: f64) -> Vec<Vec<u8>> {
    let mut world = vec![Vec::<u8>::with_capacity(cols); rows];
    for row in world.iter_mut() {
        for _ in 0..cols {
            row.push(thread_rng().gen_bool(portion_alive) as u8);
        }
    }

    world
}

pub fn get_stop_condition(seconds: Option<u32>) -> Box<dyn Fn() -> bool> {
    match seconds {
        Some(s) => {
            let now = Instant::now();
            let ending_time = now + Duration::from_secs(s.into());
            Box::new(move || Instant::now() > ending_time)
        }
        None => Box::new(|| false),
    }
}

pub fn run<'a>(
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
