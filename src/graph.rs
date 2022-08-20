use crate::space::{Direction, Point, Size};

pub struct Edge {
    pub cell_id: usize,
    pub direction: Direction,
}

impl Edge {
    pub fn new(cell_id: usize, direction: Direction) -> Edge {
        Edge {
            cell_id: cell_id,
            direction: direction,
        }
    }
}

fn cell(size: Size, point: Point) -> usize {
    point.x + (point.y * size.width) + (point.z * size.width * size.depth)
}

pub fn edges(dimensions: Size, point: Point) -> Vec<Edge> {
    let mut edges = Vec::new();
    if point.x > 0 {
        let cell = cell(dimensions, Point::new(point.x - 1, point.y, point.z));
        edges.push(Edge::new(cell, Direction::Right));
    }
    if point.x < dimensions.width - 1 {
        let cell = cell(dimensions, Point::new(point.x + 1, point.y, point.z));
        edges.push(Edge::new(cell, Direction::Left));
    }
    if point.y > 0 {
        let cell = cell(dimensions, Point::new(point.x, point.y - 1, point.z));
        edges.push(Edge::new(cell, Direction::Back));
    }
    if point.y < dimensions.depth - 1 {
        let cell = cell(dimensions, Point::new(point.x, point.y + 1, point.z));
        edges.push(Edge::new(cell, Direction::Front));
    }
    if point.z > 0 {
        let cell = cell(dimensions, Point::new(point.x, point.y, point.z - 1));
        edges.push(Edge::new(cell, Direction::Up));
    }
    if point.z < dimensions.height - 1 {
        let cell = cell(dimensions, Point::new(point.x, point.y, point.z + 1));
        edges.push(Edge::new(cell, Direction::Down));
    }
    edges
}
