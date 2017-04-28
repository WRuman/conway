mod game;
mod grid;
use game::Game;
use std::thread;
use std::time::Duration;

fn main() {
    let sleep_time = Duration::from_millis(100);
    let mut sim = Game::new_with_size(50);
    sim.add_glider();
    sim.show_board();
    loop {
        thread::sleep(sleep_time);
        sim.tick();
        sim.show_board();
    }
}

