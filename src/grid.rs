use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::point::Point;

pub trait GridVal: Copy + PartialEq {}
impl<T> GridVal for T where T: Copy + PartialEq {}

#[derive(Debug)]
pub struct Grid<T: GridVal> {
    pub width: usize,
    pub height: usize,
    pub bytes: Vec<T>,
}

impl<T: GridVal> Grid<T> {
    #[inline]
    pub fn contains(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width as i32 && p.y < self.height as i32
    }

    #[inline]
    pub fn as_point(&self, i: usize) -> Point {
        Point::new((i % self.width) as i32, (i / self.width) as i32)
    }

    pub fn find(&self, val: T) -> Option<Point> {
        self.bytes
            .iter()
            .position(|&v| v == val)
            .map(|i| self.as_point(i))
    }

    pub fn find_all(&self, val: T) -> Vec<Point> {
        self.bytes
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| {
                if v == val {
                    Some(self.as_point(i))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Grid<u8> {
        let lines: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();
        let width = lines[0].len();
        let height = lines.len();
        let mut bytes = Vec::with_capacity(width * height);
        for l in lines {
            bytes.extend_from_slice(l);
        }
        Grid {
            width,
            height,
            bytes,
        }
    }
}

impl<T: GridVal> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, p: Point) -> &Self::Output {
        &self.bytes[(p.y * self.width as i32 + p.x) as usize]
    }
}

impl<T: GridVal> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.bytes[(index.y * self.width as i32 + index.x) as usize]
    }
}

impl<T: GridVal + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self[Point::new(x as i32, y as i32)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
