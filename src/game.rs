use crate::snake::Snake;
use crate::point::Point;
use crate::hamiltonian_matrix::HamiltonMatrix;
use crate::direction::Direction;

pub struct Game {
    snake: Snake,
    apple: Point,
    board: HamiltonMatrix,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,

    pub running: bool,
    pub tick_speed: u32,
}

impl Game {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Game {
        let board_width = (max_x - min_x) as usize;
        let board_height = (max_y - min_y) as usize;

        if board_height % 2 != 0 {
            println!("Bad board height");
        }

        if board_width % 2 != 0 {
            println!("Bad board width");
        }

        let snake = Snake::new(Point::new(10, 10));

        let mut gm = Game {
            snake: snake,
            apple: Point::new(0, 0),
            board: HamiltonMatrix::new_filled(board_width, board_height),
            min_x: min_x,
            min_y: min_y,
            max_x: max_x,
            max_y: max_y,

            running: true,
            tick_speed: 20,
        };

        gm.apple = gm.random_point();
        return gm;
    }

    pub fn tick(&mut self) -> bool {
        // if self.snake.check_collide() {
        //     return false;
        // }

        // if self.min_x + self.snake.get_head().x < self.min_x || self.min_x + self.snake.get_head().x > self.max_x {
        //     return false;
        // }

        // if self.min_y + self.snake.get_head().y < self.min_y || self.min_y + self.snake.get_head().y > self.max_y {
        //     return false;
        // }

        if self.snake.get_head() == &self.apple {
            self.snake.expand();
            self.apple = self.random_point();
        }

        return true;
    }

    pub fn draw(&self) {
        ncurses::erase();
        // mvprintw(0, 0, "Currently Running Snake... Exit with F1");
        ncurses::mvprintw(
            0,
            0,
            &format!(
                "Apple={:?}, SnakeLen={:?}, Board={}x{}, Speed={}",
                self.apple,
                self.snake.size(),
                self.board.get_width(),
                self.board.get_height(),
                self.tick_speed
            ),
        );
        self.draw_border();
        // self.draw_cycle();
        self.snake.draw(self.min_x, self.min_y);
        ncurses::mvhline(self.min_y + self.apple.y, self.min_y + self.apple.x, ncurses::ACS_DIAMOND(), 1);
        ncurses::refresh();
    }

    fn draw_border(&self) {
        ncurses::mvhline(
            self.min_y - 1,
            self.min_x - 1,
            ncurses::ACS_HLINE(),
            self.max_x - self.min_x + 2,
        );
        ncurses::mvhline(
            self.max_y + 1,
            self.min_x - 1,
            ncurses::ACS_HLINE(),
            self.max_x - self.min_x + 2,
        );

        ncurses::mvvline(
            self.min_y - 1,
            self.min_x - 1,
            ncurses::ACS_VLINE(),
            self.max_y - self.min_y + 2,
        );
        ncurses::mvvline(
            self.min_y - 1,
            self.max_x + 1,
            ncurses::ACS_VLINE(),
            self.max_y - self.min_y + 2,
        );

        ncurses::mvhline(self.min_y - 1, self.min_x - 1, ncurses::ACS_ULCORNER(), 1);
        ncurses::mvhline(self.max_y + 1, self.min_x - 1, ncurses::ACS_LLCORNER(), 1);
        ncurses::mvhline(self.max_y + 1, self.max_x + 1, ncurses::ACS_LRCORNER(), 1);
        ncurses::mvhline(self.min_y - 1, self.max_x + 1, ncurses::ACS_URCORNER(), 1);
    }

    fn check_dir(&self, currid: &u32, dir: &Direction) -> bool {
        let test_pt = match dir {
            Direction::UP => { self.snake.get_head().add(0, -1) }
            Direction::RIGHT => { self.snake.get_head().add(1, 0) }
            Direction::DOWN => { self.snake.get_head().add(0, 1) }
            Direction::LEFT => { self.snake.get_head().add(-1, 0) }
        };

        if let Some(other) = self.board.get(test_pt.x, test_pt.y) {
            if other == &(currid + 1) {
                return true;
            }
        }
        return false;
    }

    pub fn get_next_cycle_dir(&self) -> Direction {
        let currid = *self.board.get(self.snake.get_head().x, self.snake.get_head().y).unwrap();
        for dir in Direction::all() {
            if self.check_dir(&currid, dir) {
                return *dir;
            }
        }

        for dir in Direction::all() {
            if self.check_dir(&0, dir) {
                return *dir;
            }
        }

        panic!("Reach end!");
    }

    pub fn move_snake(&mut self) {
        let ideal_move = self.get_next_cycle_dir();
        self.snake.move_dir(&ideal_move);
    }

    fn random_point(&self) -> Point {
        let x = rand::random::<u8>();
        let y = rand::random::<u8>();

        let screen_x: u8 = (self.max_x - self.min_x) as u8;
        let screen_y: u8 = (self.max_y - self.min_y) as u8;

        let mut pos_point = Point {
            x: ((x % screen_x) as u8) as i32,
            y: ((y % screen_y) as u8) as i32,
        };

        let mut allowed = true;
        self.snake.for_each_segment(|pt, _| {
            if pt == pos_point {
                allowed = false;
            }
        });

        if !allowed {
            pos_point = self.random_point();
        }
        return pos_point;
    }
}