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
}

impl Game {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Game {
        let board_width = (max_x - min_x) as usize;
        let board_height = (max_y - min_y) as usize;

        let snake = Snake::new(Point::new(0, 0));
        let snake_head = *snake.get_head();
        let snake_tail = snake.get_tail();

        let mut gm = Game {
            snake: snake,
            apple: Point::new(0, 0),
            board: HamiltonMatrix::new_filled(board_width, board_height, &snake_head, &snake_tail),
            min_x: min_x,
            min_y: min_y,
            max_x: max_x,
            max_y: max_y,
        };

        gm.apple = gm.random_point();
        return gm;
    }

    pub fn tick(&mut self) -> bool {
        if self.snake.check_collide() {
            return false;
        }

        if self.snake.get_head().x < self.min_x || self.snake.get_head().x > self.max_x {
            return false;
        }

        if self.snake.get_head().y < self.min_y || self.snake.get_head().y > self.max_y {
            return false;
        }

        if self.snake.get_head() == &self.apple {
            self.snake.expand();
            self.apple = self.random_point();
        }

        return true;
    }

    pub fn draw(&self) {
        // mvprintw(0, 0, "Currently Running Snake... Exit with F1");
        ncurses::mvprintw(
            0,
            0,
            &format!(
                "Apple={:?}, SnakeLen={:?}",
                self.apple,
                self.snake.size()
            ),
        );
        self.draw_border();
        // self.draw_cycle();
        self.snake.draw();
        ncurses::mvhline(self.apple.y, self.apple.x, ncurses::ACS_DIAMOND(), 1);
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

    pub fn input(&mut self, ch: i32) {
        let to_move = match ch {
            ncurses::KEY_LEFT => Some(Direction::LEFT),
            ncurses::KEY_RIGHT => Some(Direction::RIGHT),
            ncurses::KEY_UP => Some(Direction::UP),
            ncurses::KEY_DOWN => Some(Direction::DOWN),
            _ => None,
        };

        if to_move.is_some() {
            ncurses::erase();
            self.snake.move_dir(&to_move.unwrap());
        }
    }

    fn random_point(&self) -> Point {
        let x = rand::random::<u8>();
        let y = rand::random::<u8>();

        let screen_x: u8 = (self.max_x - self.min_x) as u8;
        let screen_y: u8 = (self.max_y - self.min_y) as u8;

        let mut pos_point = Point {
            x: ((x % screen_x) + self.min_x as u8) as i32,
            y: ((y % screen_y) + self.min_y as u8) as i32,
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

    fn draw_cycle(&self) {
        for x in 0..self.board.get_width() {
            for y in 0..self.board.get_height() {
                let screen_x = self.min_x + x as i32;
                let screen_y = self.min_y + y as i32;
                ncurses::mvhline(screen_y, screen_x, ncurses::ACS_BULLET(), 1);

                if let Some(v) = self.board.get(x, y) {
                    if v != &0 {
                        ncurses::mvhline(screen_y, screen_x, ncurses::ACS_BLOCK(), 1);
                    }
                }
            }
        }
    }

    pub fn print_board(&self) {
        self.board.print_matrix();
    }
}