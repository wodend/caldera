use std::collections::HashMap;
use std::fmt;

use log::{debug, error, info, log_enabled, Level};

use crate::math::{fermi_dirac, gaussian};
use crate::grid::{Coordinate, Direction, Grid};
use crate::state::{ground, sky};

pub const STATE_COUNT: usize = 2;
const GROUND: usize = 0;
const SKY: usize = 1;
const EDGE: usize = 2;

//pub const SIZE: usize = 64;
pub const SIZE: usize = 3;
pub type State = usize;
pub type StateNames = [&'static str; STATE_COUNT];
pub type Weights = [f32; STATE_COUNT];
pub const STATES: [State; STATE_COUNT] = [GROUND, SKY];

pub struct Signal<'a> {
    pub state: State,
    pub direction: &'a Direction,
    pub distance: usize,
}

impl<'a> Signal<'a> {
    pub fn new(state: State, direction: &'a Direction, distance: usize) -> Self {
        return Self {
            state: state,
            direction: direction,
            distance: distance,
        };
    }
}

impl<'a> fmt::Display for Signal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Signal{{state={}, direction={:?}, distance={}}}",
            self.state, self.direction, self.distance,
        )?;
        return Ok(());
    }
}

pub fn entropy(weights: &Weights) -> f32 {
    return weights.iter().fold(0.0, |acc, p| acc + (-p) * p.log2());
}

pub fn normalize(weights: &mut Weights) {
    let sum: f32 = weights.iter().sum();
    for x in weights {
        *x = *x / sum
    }
}

pub fn names() -> StateNames {
    //return ["ground", "sky", "edge"];
    return ["ground", "sky"];
}

// pub fn update_vector_dict(
//     states: [usize; 2],
//     max_distance: usize,
// ) -> HashMap<(usize, Direction, usize), [f32; 2]> {
//     let directions: [Direction; 6] = [
//         Direction::Left,
//         Direction::Right,
//         Direction::Front,
//         Direction::Back,
//         Direction::Down,
//         Direction::Up,
//     ];
// 
//     let mut dictionary = HashMap::new();
// 
//     for state in states.iter() {
//         for distance in 0..max_distance {
//             for direction in directions.iter() {
//                 dictionary.insert(
//                     //Signal::new(*state, direction, distance),
//                     (*state, *direction, distance),
//                     [
//                         update_weight_ground(&Signal::new(*state, direction,distance)),
//                         update_weight_sky(&Signal::new(*state, direction,distance)),
//                     ],
//                 );
//             }
//         }
//     }
//     return dictionary;
// }

// pub fn init_ground(grid: &Grid, coordinate: &Coordinate) -> f32 {
//     let a = 1.0;
//     let x_0 = grid.width as f32 / 2.0;
//     let y_0 = grid.depth as f32 / 2.0;
//     let s_x = grid.width as f32 * 0.2;
//     let s_y = s_x;
//     let x = coordinate.x as f32;
//     let y = coordinate.y as f32;
//     return (1.0 - init_sky(grid, coordinate))
//         * (gaussian(a, x_0, y_0, s_x, s_y, x, y));
// }

#[derive(Debug)]
pub struct Statev3<'a> {
    name: &'a str,
    init: fn(grid: &'a Grid, coordinate: &'a Coordinate) -> f32,
    update: fn(signal: &'a Signal<'a>) -> f32,
}

#[derive(Debug)]
pub struct Weightsv2<'a> {
    states: [Statev3<'a>; STATE_COUNT],
}
impl<'a> Weightsv2<'a> {
    pub fn new() -> Self {
        let states = [
            Statev3 {name: "ground", init: ground::init, update: ground::update},
            Statev3 {name: "sky", init: sky::init, update: sky::update},
        ];
        return Self {
            states: states,
        };
    }

    pub fn update(&self, signal: &'a Signal) -> [f32; STATE_COUNT] {
        let mut init: [f32; STATE_COUNT] = [0.0; STATE_COUNT];
        for i in 0..STATE_COUNT {
            init[i] = (self.states[i].update)(signal) as f32
        }
        return init;
    }

    pub fn init(&self, grid: &'a Grid, coordinate: &'a Coordinate) -> [f32; STATE_COUNT] {
        let mut init: [f32; STATE_COUNT] = [0.0; STATE_COUNT];
        for i in 0..STATE_COUNT {
            init[i] = (self.states[i].init)(grid, coordinate) as f32
        }
        return init;
    }
}