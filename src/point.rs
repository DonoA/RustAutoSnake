use crate::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn add(&self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn dir_adj(&self, dir: &Direction) -> Point {
        match dir {
            Direction::UP => { self.add(0, -1) }
            Direction::RIGHT => { self.add(1, 0) }
            Direction::DOWN => { self.add(0, 1) }
            Direction::LEFT => { self.add(-1, 0) }
        }
    }
}