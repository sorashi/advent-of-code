use std::fmt::{Debug, Formatter, Write};
use std::ops::{Add, AddAssign, Neg};

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vector {
    pub const ZERO: Vector = Vector { x: 0, y: 0 };
    pub const LEFT: Vector = Vector { x: -1, y: 0 };
    pub const RIGHT: Vector = Vector { x: 1, y: 0 };
    pub const UP: Vector = Vector { x: 0, y: -1 };
    pub const DOWN: Vector = Vector { x: 0, y: 1 };

    pub fn get<'a, T, A: AsRef<[T]>>(&self, array: &'a [A]) -> Option<&'a T> {
        array
            .get(self.y as usize)
            .and_then(|row| row.as_ref().get(self.y as usize))
    }
    pub fn set<T, A: AsMut<[T]>>(&self, array: &mut [A], value: T) -> Result<(), &'static str> {
        let element = array
            .get_mut(self.y as usize)
            .and_then(|row| row.as_mut().get_mut(self.x as usize));
        if let Some(element) = element {
            *element = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
}

pub struct TwoDimArray<T> {
    array: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for TwoDimArray<T> {
    fn from(array: Vec<Vec<T>>) -> Self {
        Self { array }
    }
}

impl<T: Debug> Debug for TwoDimArray<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.array {
            for c in row {
                c.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T> TwoDimArray<T> {
    pub fn new<A: Default + Clone>(width: usize, height: usize) -> TwoDimArray<A> {
        TwoDimArray {
            array: vec![vec![Default::default(); width]; height],
        }
    }
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.array.get(y).and_then(|row| row.get(x))
    }
    pub fn get_by_vector(&self, position: &Vector) -> Option<&T> {
        if !self.is_vector_in_bounds(position) {
            None
        } else {
            self.get(position.x as usize, position.y as usize)
        }
    }
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), &'static str> {
        let element = self.array.get_mut(y).and_then(|row| row.get_mut(x));
        if let Some(element) = element {
            *element = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
    pub fn width(&self) -> usize {
        self.array.first().map(|row| row.len()).unwrap_or(0)
    }
    pub fn height(&self) -> usize {
        self.array.len()
    }
    pub fn is_vector_in_bounds(&self, position: &Vector) -> bool {
        position.x >= 0
            && position.y >= 0
            && self.is_in_bounds(position.x as usize, position.y as usize)
    }
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_addition() {
        let v1 = Vector { x: 3, y: 1 };
        let v2 = Vector { x: 1, y: 2 };
        assert_eq!(v1 + v2, Vector { x: 4, y: 3 });
    }
}
