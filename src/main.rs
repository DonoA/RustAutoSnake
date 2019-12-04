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

#[derive(Debug, Clone, Copy)]
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
    fn draw(&self) {
        let mut current_seg = self.head;
        mvhline(current_seg.y, current_seg.x, ACS_BLOCK(), 1);
        for i in 0..self.segment_dirs.len() {
            let next_dir = self.segment_dirs.get(i + 1);
            let to_print = get_snake_seg(&self.segment_dirs[i], next_dir);

            current_seg = match self.segment_dirs[i] {
                Direction::UP => current_seg.add(0, -1),
                Direction::RIGHT => current_seg.add(1, 0),
                Direction::DOWN => current_seg.add(0, 1),
                Direction::LEFT => current_seg.add(-1, 0),
            };

            mvhline(current_seg.y, current_seg.x, to_print, 1);
        }
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

    let snake_points = vec![
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
        Direction::LEFT,
    ];

    let mut snake = Snake {
        facing: Direction::UP,
        segment_dirs: snake_points,
        head: Point::new(10, 4),
    };

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let mut start_y = max_y / 2;
    let mut start_x = max_x / 2;

    loop {
        mvprintw(0, 0, "Currently Running Snake... Exit with F1");
        snake.draw();
        refresh();

        let ch = getch();
        let to_move = match ch {
            KEY_LEFT => Some(Direction::LEFT),
            KEY_RIGHT => Some(Direction::RIGHT),
            KEY_UP => Some(Direction::UP),
            KEY_DOWN => Some(Direction::DOWN),
            _ => None,
        };
        if to_move.is_some() {
            erase();
            snake.move_dir(&to_move.unwrap());
        }

        if ch == KEY_F(1) {
            break;
        }
    }

    endwin();
}
