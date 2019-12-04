extern crate ncurses;

use ncurses::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn oposite(&self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn add(&self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

struct Snake {
    facing: Direction,
    segment_dirs: Vec<Direction>,
    head: Point,
}

fn get_snake_seg(curr: &Direction, next: Option<&Direction>) -> u64 {
    match curr {
        Direction::UP => match next {
            Some(Direction::RIGHT) => return ACS_ULCORNER(),
            Some(Direction::LEFT) => return ACS_URCORNER(),
            _ => return ACS_VLINE(),
        },
        Direction::RIGHT => match next {
            Some(Direction::UP) => return ACS_LRCORNER(),
            Some(Direction::DOWN) => return ACS_URCORNER(),
            _ => return ACS_HLINE(),
        },
        Direction::DOWN => match next {
            Some(Direction::RIGHT) => return ACS_LLCORNER(),
            Some(Direction::LEFT) => return ACS_LRCORNER(),
            _ => return ACS_VLINE(),
        },
        Direction::LEFT => match next {
            Some(Direction::UP) => return ACS_LLCORNER(),
            Some(Direction::DOWN) => return ACS_ULCORNER(),
            _ => return ACS_HLINE(),
        },
    };
}

impl Snake {
    fn for_each_segment<F>(&self, mut f: F) where F: FnMut(Point, usize) {
        let mut current_seg = self.head;
        f(current_seg, 0);
        for i in 0..self.segment_dirs.len() {
            current_seg = match self.segment_dirs[i] {
                Direction::UP => current_seg.add(0, -1),
                Direction::RIGHT => current_seg.add(1, 0),
                Direction::DOWN => current_seg.add(0, 1),
                Direction::LEFT => current_seg.add(-1, 0),
            };

            f(current_seg, i + 1);
        }
    }

    fn draw(&self) {
        self.for_each_segment(|pt, i| {
            if i == 0 {
                mvhline(pt.y, pt.x, ACS_BLOCK(), 1);
                return;
            }

            let next_dir = self.segment_dirs.get(i);
            let to_print = get_snake_seg(&self.segment_dirs[i - 1], next_dir);
            mvhline(pt.y, pt.x, to_print, 1);
        });
    }

    fn check_collide(&self) -> bool {
        let mut dead = false;
        self.for_each_segment(|pt, i| {
            if i != 0 && pt == self.head {
                dead = true;
            }
        });
        return dead;
    }

    fn move_dir(&mut self, dir: &Direction) {
        self.segment_dirs.insert(0, dir.oposite());
        self.segment_dirs.pop();

        self.head = match dir {
            Direction::UP => self.head.add(0, -1),
            Direction::RIGHT => self.head.add(1, 0),
            Direction::DOWN => self.head.add(0, 1),
            Direction::LEFT => self.head.add(-1, 0),
        };
    }
}

struct Game {
    snake: Snake,
    apple: Point,
    min_x: i32, 
    min_y: i32, 
    max_x: i32, 
    max_y: i32
}

impl Game {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Game {
        let mut gm = Game {
            snake: Snake {
                facing: Direction::UP,
                segment_dirs: vec![
                    Direction::LEFT,
                    Direction::LEFT,
                    Direction::LEFT,
                ],
                head: Point::new(10, 4),
            },
            apple: Point::new(0, 0),
            min_x: min_x, 
            min_y: min_y, 
            max_x: max_x, 
            max_y: max_y
        };

        gm.apple = gm.random_point();
        return gm;
    }

    fn tick(&mut self) -> bool {
        if self.snake.check_collide() {
            return false;
        }

        if self.snake.head.x < self.min_x || self.snake.head.x > self.max_x {
            return false;
        }

        if self.snake.head.y < self.min_y || self.snake.head.y > self.max_y {
            return false;
        }

        if self.snake.head == self.apple {
            self.snake.segment_dirs.push(
                self.snake.segment_dirs.last().unwrap().oposite()
            );

            self.apple = self.random_point();
        }

        return true;
    }

    fn draw(&self) {
        // mvprintw(0, 0, "Currently Running Snake... Exit with F1");
        mvprintw(0, 0, &format!("Apple={:?}, SnakeLen={:?}", self.apple, self.snake.segment_dirs.len()));
        self.draw_border();
        self.snake.draw();
        mvhline(self.apple.y, self.apple.x, ACS_DIAMOND(), 1);
        refresh();
    }

    fn draw_border(&self) {
        mvhline(self.min_y - 1, self.min_x - 1, ACS_HLINE(), self.max_x - self.min_x + 2);
        mvhline(self.max_y + 1, self.min_x - 1, ACS_HLINE(), self.max_x - self.min_x + 2);

        mvvline(self.min_y - 1, self.min_x - 1, ACS_VLINE(), self.max_y - self.min_y + 2);
        mvvline(self.min_y - 1, self.max_x + 1, ACS_VLINE(), self.max_y - self.min_y + 2);

        mvhline(self.min_y - 1, self.min_x - 1, ACS_ULCORNER(), 1);
        mvhline(self.max_y + 1, self.min_x - 1, ACS_LLCORNER(), 1);
        mvhline(self.max_y + 1, self.max_x + 1, ACS_LRCORNER(), 1);
        mvhline(self.min_y - 1, self.max_x + 1, ACS_URCORNER(), 1);
    }

    fn input(&mut self, ch: i32) {
        let to_move = match ch {
            KEY_LEFT => Some(Direction::LEFT),
            KEY_RIGHT => Some(Direction::RIGHT),
            KEY_UP => Some(Direction::UP),
            KEY_DOWN => Some(Direction::DOWN),
            _ => None,
        };

        if to_move.is_some() {
            erase();
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
}

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
    setup_ncurses();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut game = Game::new(1, 2, max_x - 2, max_y - 2);

    loop {
        game.draw();

        if !game.tick() {
            break;
        }

        let ch = getch();
        game.input(ch);
        if ch == KEY_F(1) {
            break;
        }
    }

    endwin();
}
