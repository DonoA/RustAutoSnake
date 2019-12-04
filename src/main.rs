mod game;
mod snake;
mod hamiltonian_matrix;
mod point;
mod direction;

extern crate ncurses;
extern crate num;

use ncurses::*;
use crate::game::Game;

fn setup_ncurses() {
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
}

fn main() {
    // setup_ncurses();

    /* Get the screen bounds. */
    // let mut max_x = 0;
    // let mut max_y = 0;
    // getmaxyx(stdscr(), &mut max_y, &mut max_x);

    // let mut game = Game::new(1, 2, max_x-2, max_y-2);
    // let mut game = Game::new(1, 2, 30, 15);
    let mut game = Game::new(0, 0, 20, 20);

    // loop {
    //     game.draw();

    //     if !game.tick() {
    //         break;
    //     }

    //     let ch = getch();
    //     game.input(ch);
    //     if ch == KEY_F(1) {
    //         break;
    //     }
    // }

    // endwin();

    game.print_board();
}
