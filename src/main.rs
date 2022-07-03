mod cells;
mod math;
mod probability;
mod states;

use cells::Cells;
use states::{State, States};

use rand;
use rand::distributions::{Distribution, WeightedIndex};

#[derive(Clone, Copy)]
pub struct Dimensions {
    width: usize,
    depth: usize,
    height: usize,
}

impl Dimensions {
    fn new(width: usize, depth: usize, height: usize) -> Self {
        return Self {
            width: width,
            depth: depth,
            height: height,
        };
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signal {
    pub state_id: usize,
    pub direction: Direction,
    pub distance: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Front,
    Back,
    Down,
    Up,
}

impl Signal {
    pub fn new(state_id: usize, direction: Direction, distance: usize) -> Self {
        return Self {
            state_id: state_id,
            direction: direction,
            distance: distance,
        };
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let x = 3;
    let dimensions = Dimensions::new(x, x, x);
    println!(
        "Dimensions {} {} {}",
        dimensions.width, dimensions.depth, dimensions.height
    );
    let states = States::new(
        3,
        vec![
            State::new(
                "ground",
                probability::init::ground,
                probability::update::ground,
            ),
            State::new("sky", probability::init::sky, probability::update::sky),
        ],
    );
    println!("States {:?}", states.names());
    let cells = Cells::new(&mut rng, dimensions, states);
    println!("Wave {:?}", cells.wave);
}
