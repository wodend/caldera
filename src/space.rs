use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Front,
    Back,
    Down,
    Up,
}

#[derive(Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Point {
        Point { x: x, y: y, z: z }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub depth: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, depth: usize, height: usize) -> Size {
        Size {
            width: width,
            depth: depth,
            height: height,
        }
    }
}
