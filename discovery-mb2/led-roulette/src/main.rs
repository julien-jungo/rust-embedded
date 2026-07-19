#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut matrix = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut i: usize = 0;
    let mut j: usize = 0;

    let mut dir = Direction::Right;

    loop {
        display.show(&mut timer, matrix, 50);
        display.clear();

        dir = match (i, j) {
            (0, 0) => Direction::Right,
            (0, 4) => Direction::Down,
            (4, 4) => Direction::Left,
            (4, 0) => Direction::Up,
            _ => dir,
        };

        matrix[i][j] = 0;

        (i, j) = match dir {
            Direction::Right => (i, j + 1),
            Direction::Down => (i + 1, j),
            Direction::Left => (i, j - 1),
            Direction::Up => (i - 1, j),
        };

        matrix[i][j] = 1;
    }
}
