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
pub type StateNames = [&'static str; STATE_COUNT];
pub type Weights = [f32; STATE_COUNT];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Signal {
    pub state_id: usize,
    pub direction: Direction,
    pub distance: usize,
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

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Signal{{state={}, direction={:?}, distance={}}}",
            self.state_id, self.direction, self.distance,
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

#[derive(Debug)]
pub struct State<'a> {
    name: &'a str,
    init: fn(grid: &'a Grid, coordinate: &'a Coordinate) -> f32,
    update: fn(signal: Signal) -> f32,
}

#[derive(Debug)]
pub struct States<'a> {
    states: [State<'a>; STATE_COUNT],
}
impl<'a> States<'a> {
    pub fn new() -> Self {
        let states = [
            State {name: "ground", init: ground::init, update: ground::update},
            State {name: "sky", init: sky::init, update: sky::update},
        ];
        return Self {
            states: states,
        };
    }

    pub fn update(&self, signal: Signal) -> [f32; STATE_COUNT] {
        let mut update: [f32; STATE_COUNT] = [0.0; STATE_COUNT];
        for i in 0..STATE_COUNT {
            update[i] = (self.states[i].update)(signal) as f32
        }
        return update;
    }

    pub fn init(&self, grid: &'a Grid, coordinate: &'a Coordinate) -> [f32; STATE_COUNT] {
        let mut init: [f32; STATE_COUNT] = [0.0; STATE_COUNT];
        for i in 0..STATE_COUNT {
            init[i] = (self.states[i].init)(grid, coordinate) as f32
        }
        return init;
    }

    pub fn update_map(
        &self,
        directions: &Vec<Direction>,
        max_distance: usize,
    ) -> HashMap<Signal, [f32; STATE_COUNT]> {
        let mut update_map = HashMap::new();
    
        for state_id in 0..self.states.len() {
            for distance in 0..max_distance {
                for direction in directions.iter() {
                    let signal = Signal::new(state_id, *direction, distance);
                    update_map.insert(
                        signal,
                        self.update(signal),
                    );
                }
            }
        }
        return update_map;
    }

    pub fn names(&self) -> [&str; STATE_COUNT] {
        let mut names: [&str; STATE_COUNT] = [""; STATE_COUNT];
        for i in 0..STATE_COUNT {
            names[i] = self.states[i].name
        }
        return names;
    }
}