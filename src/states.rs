use std::fmt;

use log::{debug, error, info, log_enabled, Level};

use crate::grid::{Coordinate, Direction, Grid};

const STATE_COUNT: usize = 2;
const GROUND: usize = 0;
const SKY: usize = 1;
const EDGE: usize = 2;

//pub const SIZE: usize = 64;
pub const SIZE: usize = 3;
pub type State = usize;
pub type StateNames = [&'static str; STATE_COUNT];
pub type Weights = [f32; STATE_COUNT];

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
    return ["ground", "sky"];
}

pub fn initial_weights(grid: &Grid, coordinate: &Coordinate) -> Weights {
    return [
        initial_weight_ground(grid, coordinate),
        initial_weight_sky(grid, coordinate),
    ];
}

pub fn initial_weight_ground(grid: &Grid, coordinate: &Coordinate) -> f32 {
    return 2.0 / ((coordinate.z as f32 / (grid.height as f32 * 0.05)).exp() + 1.0);
}

pub fn initial_weight_sky(grid: &Grid, coordinate: &Coordinate) -> f32 {
    return 1.0 - initial_weight_ground(grid, coordinate);
}

pub fn update(weights: &mut Weights, signal: &Signal) {
    debug!(
        "Update weights [{}, {}]",
        update_weight_ground(signal),
        update_weight_sky(signal)
    );
    weights[GROUND] *= update_weight_ground(signal);
    weights[SKY] *= update_weight_sky(signal);
}

fn update_weight_ground(signal: &Signal) -> f32 {
    if signal.state == GROUND
        && (matches!(signal.direction, Direction::Left)
            || matches!(signal.direction, Direction::Right)
            || matches!(signal.direction, Direction::Front)
            || matches!(signal.direction, Direction::Back))
    {
        return 5.0;
    } else {
        return 1.0;
    }
}

fn update_weight_sky(signal: &Signal) -> f32 {
    return 1.0;
}
