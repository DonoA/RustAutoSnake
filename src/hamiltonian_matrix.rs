use crate::point::Point;
use crate::direction::Direction;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct HamiltonMatrix {
    data: Vec<Vec<u32>>,
    count: usize,
    width: usize,
    height: usize,
}

impl HamiltonMatrix {
    pub fn new_blank(width: usize, height: usize, empty_val: u32) -> HamiltonMatrix {
        let mut arr1 = Vec::new();
        for _ in 0..width {
            let mut arr2 = Vec::new();
            for _ in 0..height {
                arr2.push(empty_val);
            }
            arr1.push(arr2);
        }

        return HamiltonMatrix {
            data: arr1,
            count: width * height,
            width: width,
            height: height,
        };
    }

    pub fn set<V: num::ToPrimitive>(&mut self, x: V, y: V, v: u32) {
        self.data[x.to_usize().unwrap()][y.to_usize().unwrap()] = v;
    }

    pub fn get<V: num::ToPrimitive + PartialOrd>(&self, x: V, y: V) -> Option<&u32> {
        let x = x.to_usize();
        if x.is_none() {
            return None;
        }
        let x = x.unwrap();

        let y = y.to_usize();
        if y.is_none() {
            return None;
        }
        let y = y.unwrap();

        let arr1 = self.data.get(x);
        if arr1.is_none() {
            return None;
        }
        let arr2 = arr1.unwrap().get(y);
        return arr2;
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn new_filled(width: usize, height: usize, start: &Point, end: &Point) -> HamiltonMatrix {
        let mut matrix = HamiltonMatrix::new_blank(width, height, 0);
        matrix.fill_grid_cycle(1, start, &start.add(0, 1));
        return matrix;
    }

    fn test_grid_point(&mut self, current_id: u32, test_point: &Point, end: &Point) -> bool {
        if let Some(v) = self.get(test_point.x, test_point.y) {
            if v == &0 {
                let worked = self.fill_grid_cycle(current_id + 1, test_point, end);
                if worked {
                    return true;
                }

                self.set(test_point.x, test_point.y, 0);
            }
        }
        return false;
    }

    fn is_filled(&self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(v) = self.get(x, y) {
                    if v == &0 {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn fill_grid_cycle(&mut self, current_id: u32, current: &Point, dest: &Point) -> bool {
        self.set(current.x, current.y, current_id);

        if current_id == self.count as u32 && current == dest{
            return true;
        }
        
        let dirs: Vec<Direction> = vec!(Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT);
        let mut start = current_id;
        if current_id % 2 != 0 {
            start = 0;
        } else {
            print!("r")
        }

        for i in 0..dirs.len() {
            match dirs[(i + start as usize) % dirs.len()] {
                Direction::UP => {
                    if self.test_grid_point(current_id, &current.add(0, -1), dest) {
                        return true;
                    }
                }

                Direction::RIGHT => {
                    if self.test_grid_point(current_id, &current.add(1, 0), dest) {
                        return true;
                    }
                }

                Direction::DOWN => {
                    if self.test_grid_point(current_id, &current.add(0, 1), dest) {
                        return true;
                    }
                }

                Direction::LEFT => {
                    print!("l");
                    if self.test_grid_point(current_id, &current.add(-1, 0), dest) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    pub fn print_matrix(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(v) = self.get(x, y) {
                    print!("{:04} ", v)
                }
            }
            println!();
        }
    }
}
