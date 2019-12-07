use crate::point::Point;

pub struct Matrix<T> {
    data: Vec<Vec<Option<T>>>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Matrix<T> {
        let mut data_arr1 = Vec::with_capacity(width);

        for _ in 0..width {
            let mut data_arr2 = Vec::with_capacity(height);

            for _ in 0..height {
                data_arr2.push(None);
            }

            data_arr1.push(data_arr2);
        }

        return Matrix {
            data: data_arr1,
            width: width,
            height: height
        };
    }

    pub fn set<V: num::ToPrimitive>(&mut self, x: V, y: V, v: T) {
        self.data[x.to_usize().unwrap()][y.to_usize().unwrap()] = Some(v);
    }

    pub fn set_pt(&mut self, pt: &Point, v: T) {
        self.set(pt.x, pt.y, v);
    }

    pub fn unset<V: num::ToPrimitive>(&mut self, x: V, y: V) {
        self.data[x.to_usize().unwrap()][y.to_usize().unwrap()] = None;
    }

    pub fn unset_pt(&mut self, pt: &Point) {
        self.unset(pt.x, pt.y);
    }

    pub fn get<V: num::ToPrimitive>(&self, x: V, y: V) -> Option<&T> {
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

        let elt = arr1.unwrap().get(y);
        if elt.is_none() {
            return None;
        }

        let elt = elt.unwrap();
        if elt.is_none() {
            return None;
        }

        return elt.as_ref();
    }

    pub fn get_mut<V: num::ToPrimitive>(&mut self, x: V, y: V) -> Option<&mut T> {
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

        let arr1 = self.data.get_mut(x);
        if arr1.is_none() {
            return None;
        }

        let elt = arr1.unwrap().get_mut(y);
        if elt.is_none() {
            return None;
        }

        let elt = elt.unwrap();
        if elt.is_none() {
            return None;
        }

        return elt.as_mut();
    }

    pub fn get_pt(&self, pt: &Point) -> Option<&T> {
        self.get(pt.x, pt.y)
    }

    pub fn get_mut_pt(&mut self, pt: &Point) -> Option<&mut T> {
        self.get_mut(pt.x, pt.y)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl<T: std::fmt::Display> Matrix<T> {
    pub fn print_matrix(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(v) = self.get(x, y) {
                    print!("{:04} ", v);
                } else {
                    print!(" --  ");
                }
            }
            println!();
        }
    }
}