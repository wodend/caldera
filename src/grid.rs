#[derive(Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize, z: usize) -> Self {
        return Coordinate { x: x, y: y, z: z };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Front,
    Back,
    Down,
    Up,
}

pub type Cell = usize;

#[derive(Debug)]
pub struct Edge {
    pub direction: Direction,
    pub cell: Cell,
}

impl Edge {
    fn new(direction: Direction, cell: Cell) -> Self {
        return Self {
            direction: direction,
            cell: cell,
        };
    }
}

#[derive(Debug)]
pub struct Grid {
    pub cell_count: usize,
    pub width: usize,
    pub depth: usize,
    pub height: usize,
    pub coordinates: Vec<Coordinate>,
    pub graph: Vec<Vec<Edge>>,
    pub directions: Vec<Direction>,
}

fn cell(coordinate: Coordinate, width: usize, depth: usize) -> usize {
    return coordinate.x + (coordinate.y * width) + (coordinate.z * width * depth);
}

impl Grid {
    pub fn new(width: usize, depth: usize, height: usize) -> Self {
        let cell_count = (width * depth * height) as usize;
        let mut coordinates = Vec::new();
        let mut graph = Vec::new();
        let directions = vec![
            Direction::Left,
            Direction::Right,
            Direction::Front,
            Direction::Back,
            Direction::Down,
            Direction::Up,
        ];
        for z in 0..height {
            for y in 0..depth {
                for x in 0..width {
                    coordinates.push(Coordinate::new(x, y, z));
                    let mut edges = Vec::new();
                    if x > 0 {
                        let neighbor = cell(Coordinate::new(x - 1, y, z), width, depth);
                        edges.push(Edge::new(Direction::Right, neighbor));
                    }
                    if x < width - 1 {
                        let neighbor = cell(Coordinate::new(x + 1, y, z), width, depth);
                        edges.push(Edge::new(Direction::Left, neighbor));
                    }
                    if y > 0 {
                        let neighbor = cell(Coordinate::new(x, y - 1, z), width, depth);
                        edges.push(Edge::new(Direction::Back, neighbor));
                    }
                    if y < depth - 1 {
                        let neighbor = cell(Coordinate::new(x, y + 1, z), width, depth);
                        edges.push(Edge::new(Direction::Front, neighbor));
                    }
                    if z > 0 {
                        let neighbor = cell(Coordinate::new(x, y, z - 1), width, depth);
                        edges.push(Edge::new(Direction::Up, neighbor));
                    }
                    if z < height - 1 {
                        let neighbor = cell(Coordinate::new(x, y, z + 1), width, depth);
                        edges.push(Edge::new(Direction::Down, neighbor));
                    }
                    graph.push(edges);
                }
            }
        }
        return Self {
            cell_count: cell_count,
            width: width,
            depth: depth,
            height: height,
            coordinates: coordinates,
            graph: graph,
            directions: directions,
        };
    }
}
