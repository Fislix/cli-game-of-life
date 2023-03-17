pub fn do_step(current_world: &[Vec<u8>], next_world: &mut [Vec<u8>], rows: usize, cols: usize) {
    // todo: optimize maybe by storing the number of alive neighbors that are still neighbors of the next cell
    for y in 0..rows {
        for x in 0..cols {
            let is_alive = current_world[y][x] == 1;
            let neighbors_alive = count_alive_neighbors(current_world, x, y, rows, cols);
            next_world[y][x] = get_new_status(is_alive, neighbors_alive);
        }
    }
}

fn count_alive_neighbors(world: &[Vec<u8>], x: usize, y: usize, rows: usize, cols: usize) -> u8 {
    let mut n_alive = 0;
    for (x, y) in get_neighbor_indices(x, y, rows, cols).iter() {
        n_alive += world[*y][*x];
    }

    n_alive
}

fn get_neighbor_indices(x: usize, y: usize, rows: usize, cols: usize) -> [(usize, usize); 8] {
    let x = x + cols;
    let y = y + rows;

    let indices = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    indices.map(|(x, y)| (x % cols, y % rows))
}

fn get_new_status(is_alive: bool, neighbors_alive: u8) -> u8 {
    match (is_alive, neighbors_alive) {
        (true, 2..=3) => 1,
        (false, 3) => 1,
        (_, n @ 9..) => panic!("Invalid number of alive neighbors: {n}"),
        _ => 0,
    }
}
