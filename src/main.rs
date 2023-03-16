use cli_game_of_life::{do_step, print_world, reset_console};

fn main() {
    let mut world: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 1, 0, 0, 1],
        vec![0, 1, 1, 0, 0],
    ];

    let rows = world.len();
    let cols = world[0].len();

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
