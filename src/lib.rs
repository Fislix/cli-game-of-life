mod cli;
mod game;

pub use cli::{print_world, reset_console, Cli, Parser};
pub use game::do_step;
