
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const ALL_DIRS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT
];

impl Direction {
    pub fn oposite(&self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
        }
    }

    pub fn all() -> &'static [Direction] {
        &ALL_DIRS
    }
}