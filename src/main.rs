mod game;
mod grid;
use game::Game;
use std::thread;
use std::time::Duration;

fn main() {
    let sleep_time = Duration::from_millis(850);
    let mut sim = Game::new_with_size(10);
    loop {
        thread::sleep(sleep_time);
        sim.tick();
        sim.show_board();
    }
}

