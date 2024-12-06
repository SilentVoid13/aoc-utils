use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

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

const UP: Point = Point::new(0, -1);
const LEFT: Point = Point::new(-1, 0);
const DOWN: Point = Point::new(0, 1);
const RIGHT: Point = Point::new(1, 0);
const ORTHO: [Point; 4] = [UP, LEFT, DOWN, RIGHT];
const DIAG: [Point; 8] = [
    UP,
    Point::new(1, -1),
    LEFT,
    Point::new(-1, -1),
    DOWN,
    Point::new(1, 1),
    RIGHT,
    Point::new(-1, 1),
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
