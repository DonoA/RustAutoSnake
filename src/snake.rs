use crate::direction::Direction;
use crate::point::Point;

pub struct Snake {
    segment_dirs: Vec<Direction>,
    head: Point,
}

fn get_snake_seg(curr: &Direction, next: Option<&Direction>) -> u64 {
    match curr {
        Direction::UP => match next {
            Some(Direction::RIGHT) => return ncurses::ACS_ULCORNER(),
            Some(Direction::LEFT) => return ncurses::ACS_URCORNER(),
            _ => return ncurses::ACS_VLINE(),
        },
        Direction::RIGHT => match next {
            Some(Direction::UP) => return ncurses::ACS_LRCORNER(),
            Some(Direction::DOWN) => return ncurses::ACS_URCORNER(),
            _ => return ncurses::ACS_HLINE(),
        },
        Direction::DOWN => match next {
            Some(Direction::RIGHT) => return ncurses::ACS_LLCORNER(),
            Some(Direction::LEFT) => return ncurses::ACS_LRCORNER(),
            _ => return ncurses::ACS_VLINE(),
        },
        Direction::LEFT => match next {
            Some(Direction::UP) => return ncurses::ACS_LLCORNER(),
            Some(Direction::DOWN) => return ncurses::ACS_ULCORNER(),
            _ => return ncurses::ACS_HLINE(),
        },
    };
}

impl Snake {
    pub fn new(head: Point) -> Snake {
        Snake {
            segment_dirs: vec![Direction::LEFT; 5],
            head: head,
        }
    }

    pub fn on_snake(&self, pt: Point) -> bool {
        let mut on = false;
        self.for_each_segment(|other_pt, i| {
            if other_pt == pt {
                on = true;
            }
        });
        return on;
    }

    pub fn for_each_segment<F: FnMut(Point, usize)>(&self, mut f: F) {
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

    pub fn draw(&self, min_x: i32, min_y: i32) {
        self.for_each_segment(|pt, i| {
            if i == 0 {
                ncurses::mvhline(min_y + pt.y, min_x + pt.x, ncurses::ACS_BLOCK(), 1);
                return;
            }

            let next_dir = self.segment_dirs.get(i);
            let to_print = get_snake_seg(&self.segment_dirs[i - 1], next_dir);
            ncurses::mvhline(min_y + pt.y, min_x + pt.x, to_print, 1);
        });
    }

    pub fn check_collide(&self) -> bool {
        let mut dead = false;
        self.for_each_segment(|pt, i| {
            if i != 0 && pt == self.head {
                dead = true;
            }
        });
        return dead;
    }

    pub fn move_dir(&mut self, dir: &Direction) {
        self.segment_dirs.insert(0, dir.oposite());
        self.segment_dirs.pop();

        self.head = match dir {
            Direction::UP => self.head.add(0, -1),
            Direction::RIGHT => self.head.add(1, 0),
            Direction::DOWN => self.head.add(0, 1),
            Direction::LEFT => self.head.add(-1, 0),
        };
    }

    pub fn get_head(&self) -> &Point {
        &self.head
    }
    pub fn get_tail(&self) -> Point {
        let mut tail = None;
        self.for_each_segment(|pt, i| {
            if i == self.segment_dirs.len() {
                tail = Some(pt);
            }
        });

        let tail = tail.expect("No Tail?");
        return tail;
    }
    pub fn size(&self) -> usize {
        self.segment_dirs.len()
    }

    pub fn expand(&mut self) {
        self.segment_dirs
            .push(self.segment_dirs.last().unwrap().oposite());
    }
}
