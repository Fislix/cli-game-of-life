mod simulation_logic;

use crate::cli::{print_world, reset_console};
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};

pub use simulation_logic::do_step;

pub struct Board {
    board: Vec<u8>,
    cols: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Board { board: vec![0; rows * cols], cols }
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.board[y * self.cols + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.board[y * self.cols + x] = value;
    }

    pub fn fill_random(&mut self, portion_alive: f64) {
        for i in 0..self.board.len() {
            self.board[i] = thread_rng().gen_bool(portion_alive) as u8;
        }
    }
}

pub struct Game {
    pub world: Board,
    pub buffer: Board,
    rows: usize,
    cols: usize,
}

impl Game {
    pub fn new(rows: usize, cols: usize, portion_alive: f64) -> Game {
        let mut world = Board::new(rows, cols);
        world.fill_random(portion_alive);
        let buffer = Board::new(rows, cols);

        Game {
            world,
            buffer,
            rows,
            cols,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn swap_world_and_buffer(&mut self) {
        std::mem::swap(&mut self.world, &mut self.buffer);
    }
}

pub fn get_stop_condition(seconds: Option<u32>, steps: Option<u32>) -> Box<dyn Fn(u32) -> bool> {
    match (seconds, steps) {
        (Some(max_seconds), None) => {
            let now = Instant::now();
            let ending_time = now + Duration::from_secs(max_seconds.into());
            Box::new(move |_| Instant::now() > ending_time)
        }
        (None, Some(max_steps)) => Box::new(move |steps: u32| steps >= max_steps),
        (None, None) => Box::new(|_| false),
        (Some(_), Some(_)) => {
            panic!("Failed to restrict argument group: Got both \"seconds\" and \"steps\"")
        }
    }
}

pub fn run(game: &mut Game, should_stop: Box<dyn Fn(u32) -> bool>, steps_per_second: f64) {
    let mut steps: u32 = 0;

    let time_step = Duration::from_secs_f64(1.0 / steps_per_second);
    let mut t0 = Instant::now();

    while !should_stop(steps) {
        reset_console();
        print_world(game);
        do_step(game);
        game.swap_world_and_buffer();

        steps += 1;
        enforce_speed(&mut t0, time_step);
    }
}

fn enforce_speed(t0: &mut Instant, time_step: Duration) {
    let t1 = Instant::now();
    let delta_t = t1 - *t0;
    if delta_t < time_step {
        std::thread::sleep(time_step - delta_t);
    }
    *t0 = t1;
}
