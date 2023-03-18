use super::Game;

pub fn do_step(game: &mut Game) {
    for y in 0..game.rows() {
        for x in 0..game.cols() {
            let is_alive = game.world.get(x, y) == 1;
            let neighbors_alive = count_alive_neighbors(game, x, y);
            game.buffer.set(x, y, get_new_status(is_alive, neighbors_alive));
        }
    }
}

fn count_alive_neighbors(game: &Game, x: usize, y: usize) -> u8 {
    let mut n_alive = 0;
    for (x, y) in get_neighbor_indices(game, x, y).iter() {
        n_alive += game.world.get(*x, *y);
    }

    n_alive
}

fn get_neighbor_indices(game: &Game, x: usize, y: usize) -> [(usize, usize); 8] {
    let x = x + game.cols();
    let y = y + game.rows();

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

    indices.map(|(x, y)| (x % game.cols(), y % game.rows()))
}

fn get_new_status(is_alive: bool, neighbors_alive: u8) -> u8 {
    match (is_alive, neighbors_alive) {
        (true, 2..=3) => 1,
        (false, 3) => 1,
        (_, n @ 9..) => panic!("Invalid number of alive neighbors: {n}"),
        _ => 0,
    }
}
