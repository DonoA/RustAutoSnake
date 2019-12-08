use crate::a_star::a_star_path;
use crate::direction::Direction;
use crate::hamiltonian_matrix::HamiltonMatrix;
use crate::point::Point;
use crate::snake::Snake;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PathMode {
    ASTAR,
    HAMILTON,
}

pub struct Game {
    snake: Snake,
    apple: Point,
    ham_cycle: HamiltonMatrix,
    a_star_path: Vec<Point>,
    a_star_current: usize,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,

    board_width: usize,
    board_height: usize,

    pub running: bool,
    pub tick_speed: u32,
    current_path_mode: PathMode,
}

const SNAKE_HEAD: Point = Point { x: 10, y: 10 };

impl Game {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32, path_mode: PathMode) -> Game {
        let board_width = (max_x - min_x) as usize;
        let board_height = (max_y - min_y) as usize;

        if board_height % 2 != 0 {
            panic!("Bad board height");
        }

        if board_width % 2 != 0 {
            panic!("Bad board width");
        }

        let snake = Snake::new(SNAKE_HEAD);

        let mut gm = Game {
            snake: snake,
            apple: Point::new(0, 0),

            ham_cycle: HamiltonMatrix::new_filled(board_width, board_height),
            a_star_path: vec![],
            a_star_current: 1,

            min_x: min_x,
            min_y: min_y,
            max_x: max_x,
            max_y: max_y,

            board_width: board_width,
            board_height: board_height,

            running: false,
            tick_speed: 20,

            current_path_mode: path_mode,
        };

        gm.apple = gm.new_apple_point();
        gm.a_star_path =
            a_star_path(&gm.apple, &gm.snake, board_width, board_height).expect("Bad path");
        return gm;
    }

    pub fn tick(&mut self) -> bool {
        if self.snake.check_collide() {
            return false;
        }

        if self.min_x + self.snake.get_head().x < self.min_x
            || self.min_x + self.snake.get_head().x > self.max_x
        {
            return false;
        }

        if self.min_y + self.snake.get_head().y < self.min_y
            || self.min_y + self.snake.get_head().y > self.max_y
        {
            return false;
        }

        if self.snake.get_head() == &self.apple {
            let board_max = self.ham_cycle.get_width() * self.ham_cycle.get_height();
            if self.snake.size() == board_max - 2 {
                return false;
            }
            self.snake.expand();
            self.apple = self.new_apple_point();
            if self.current_path_mode == PathMode::ASTAR {
                let pos_path = a_star_path(
                    &self.apple,
                    &self.snake,
                    self.board_width,
                    self.board_height,
                );
                if pos_path.is_none() {
                    self.running = false;
                } else {
                    self.a_star_path = pos_path.unwrap();
                }
                self.a_star_current = 1;
            }
        }

        return true;
    }

    pub fn draw(&self) {
        ncurses::erase();
        let ham_v = self
            .ham_cycle
            .get(self.snake.get_head().x, self.snake.get_head().y)
            .unwrap();
        let apple_v = self.ham_cycle.get(self.apple.x, self.apple.y).unwrap();

        ncurses::mvprintw(
            0,
            0,
            &format!(
                "Apple={:?}, SnakeLen={:}, Board={}x{}, Speed={}, HamV={:03}, AppleV={:03}, HeadTail={:03}",
                self.apple,
                self.snake.size(),
                self.ham_cycle.get_width(),
                self.ham_cycle.get_height(),
                self.tick_speed,
                ham_v,
                apple_v,
                self.tail_mod_dist(*ham_v)
            ),
        );
        self.draw_border();

        // self.draw_cycle();
        // self.draw_astar();

        self.snake.draw(self.min_x, self.min_y);
        ncurses::mvhline(
            self.min_y + self.apple.y,
            self.min_x + self.apple.x,
            ncurses::ACS_CKBOARD(),
            1,
        );
        ncurses::refresh();
    }

    fn draw_astar(&self) {
        for pt in &self.a_star_path {
            ncurses::mvhline(
                self.min_y + pt.y,
                self.min_x + pt.x,
                ncurses::ACS_BLOCK(),
                1,
            );
        }
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

    fn is_dir_next(&self, currid: &u32, dir: &Direction) -> bool {
        let test_pt = self.snake.get_head().dir_adj(dir);

        if let Some(other) = self.ham_cycle.get(test_pt.x, test_pt.y) {
            if other == &(currid + 1) {
                return true;
            }
        }
        return false;
    }

    pub fn move_snake(&mut self) {
        match self.current_path_mode {
            PathMode::ASTAR => self.move_snake_astar(),
            PathMode::HAMILTON => self.move_snake_ham(),
        };
    }

    fn move_snake_astar(&mut self) {
        let next_dir = self
            .snake
            .get_head()
            .in_dir(&self.a_star_path[self.a_star_current]);
        self.a_star_current += 1;
        self.snake.move_dir(&next_dir);
    }

    fn move_snake_ham(&mut self) {
        let currid = *self
            .ham_cycle
            .get(self.snake.get_head().x, self.snake.get_head().y)
            .unwrap();
        let apple_val = *self
            .ham_cycle
            .get(self.apple.x, self.apple.y)
            .expect("Apple not on board");

        let mut closest_path: Option<(u32, Direction)> = None;
        for dir in Direction::all() {
            let test_pt = self.snake.get_head().dir_adj(dir);

            if let Some(other) = self.ham_cycle.get(test_pt.x, test_pt.y) {
                // don't go past apple
                if apple_val > currid && other > &apple_val {
                    continue;
                }
                //    dont go backwards
                if other < &currid {
                    continue;
                }

                // get largest value
                if closest_path.is_some() && &closest_path.unwrap().0 > other {
                    continue;
                }

                // try not to turn into self
                if self.snake.on_snake(test_pt) {
                    continue;
                }

                // don't jump too close to tail
                if self.tail_mod_dist(*other) < 5 {
                    continue;
                }

                closest_path = Some((*other, *dir));
            }
        }

        if closest_path.is_some() {
            self.snake.move_dir(&closest_path.unwrap().1);
            return;
        }

        // Check for next num in cycle
        for dir in Direction::all() {
            if self.is_dir_next(&currid, dir) {
                self.snake.move_dir(dir);
                return;
            }
        }

        // if we hit end, find pt 1
        for dir in Direction::all() {
            if self.is_dir_next(&0, dir) {
                self.snake.move_dir(dir);
                return;
            }
        }

        panic!("Reach end!");
    }

    fn new_apple_point(&self) -> Point {
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
            pos_point = self.new_apple_point();
        }
        return pos_point;
    }

    fn tail_mod_dist(&self, test_val: u32) -> i32 {
        let board_max = (self.ham_cycle.get_width() * self.ham_cycle.get_height()) as u32;
        let tail_pos = self.snake.get_tail();
        let tail_id = self
            .ham_cycle
            .get(tail_pos.x, tail_pos.y)
            .expect("Bad tail unwrap");
        let head_id = self
            .ham_cycle
            .get(self.snake.get_head().x, self.snake.get_head().y)
            .unwrap();

        let test_dist = if tail_id < head_id && tail_id < &test_val && head_id > &test_val {
            -1
        } else if tail_id > head_id && tail_id < &test_val && head_id < &test_val {
            -1
        } else if tail_id < &test_val {
            ((board_max - test_val) + tail_id) as i32
        } else {
            (tail_id - test_val) as i32
        };

        return test_dist;
    }
}
