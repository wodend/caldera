use std::collections::HashMap;
use std::fmt;

use log::{debug, error, info, log_enabled, Level};

use crate::grid::{Coordinate, Direction, Grid};

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

pub fn gaussian(a: f32, x_0: f32, y_0: f32, s_x: f32, s_y: f32, x: f32, y: f32) -> f32 {
    return (a * f32::exp(
        -(((x - x_0).powi(2) / (2.0 * s_x.powi(2))) + ((y - y_0).powi(2) / (2.0 * s_y.powi(2)))),
    ));
}

pub fn update_vector_dict(
    states: [usize; 2],
    max_distance: usize,
) -> HashMap<(usize, Direction, usize), [f32; 2]> {
    let directions: [Direction; 6] = [
        Direction::Left,
        Direction::Right,
        Direction::Front,
        Direction::Back,
        Direction::Down,
        Direction::Up,
    ];

    let mut dictionary = HashMap::new();

    for state in states.iter() {
        for distance in 0..max_distance {
            for direction in directions.iter() {
                dictionary.insert(
                    //Signal::new(*state, direction, distance),
                    (*state, *direction, distance),
                    [
                        update_weight_ground(&Signal::new(*state, direction,distance)),
                        update_weight_sky(&Signal::new(*state, direction,distance)),
                    ],
                );
            }
        }
    }
    return dictionary;
}

pub fn initial_weights(grid: &Grid, coordinate: &Coordinate) -> Weights {
    return [
        initial_weight_ground(grid, coordinate),
        initial_weight_sky(grid, coordinate),
        //initial_weight_edge(grid, coordinate),
    ];
}

//pub fn initial_weight_ground(grid: &Grid, coordinate: &Coordinate) -> f32 {
//    return 2.0 / (f32::exp(coordinate.z as f32 / 0.4) + 1.0);
//    //return 2.0 / ((coordinate.z as f32 / (grid.height as f32 * 0.05)).exp() + 1.0);
//}

pub fn initial_weight_sky(grid: &Grid, coordinate: &Coordinate) -> f32 {
    // return 0.1;
    // if coordinate.z == 0 {
    //     return 0.00001;
    // }
    // return 1.0 - (2.0 / (f32::exp(coordinate.z as f32 / 2.0) + 1.0));
    return (coordinate.z as f32 + 1.0) / grid.height as f32;
    //return coordinate.z as f32;
    //let z = grid.height as f32 - coordinate.z as f32;
    //return (2.0 / ((z / (grid.height as f32 * 0.05)).exp() + 1.0001));
}
//pub fn initial_weight_sky(grid: &Grid, coordinate: &Coordinate) -> f32 {
//    return 1.0 - initial_weight_ground(grid, coordinate);
//}

pub fn initial_weight_ground(grid: &Grid, coordinate: &Coordinate) -> f32 {
    //let a = coordinate.z as f32 + 1.0;
    //let a = 1.0;
    let a = 0.5;
    //let a = (grid.height as f32 - coordinate.z as f32) * 0.2;
    //let a = (1.0 - ((coordinate.z as f32 + 1.0) / grid.height as f32)) * 0.1;
    let x_0 = grid.width as f32 / 2.0;
    let y_0 = grid.depth as f32 / 2.0;
    let s_x = grid.width as f32 * 0.1;
    let s_y = s_x;
    let x = coordinate.x as f32;
    let y = coordinate.y as f32;
    return (1.0 - initial_weight_sky(grid, coordinate))
        * (gaussian(a, x_0, y_0, s_x, s_y, x, y));
    //return gaussian(a, x_0, y_0, s_x, s_y, x, y);
}

pub fn update(weights: &mut Weights, signal: &Signal) {
    // updates = dict(signal => vector)
    // element_wise_multiply(weights, updates[signal])
    // updates = update_dict[signal];
    // weights.iter().for_each(|&x| x * update[]);
    debug!(
        "Update weights [{}, {}]",
        update_weight_ground(signal),
        update_weight_sky(signal)
    );
    weights[GROUND] *= update_weight_ground(signal);
    weights[SKY] *= update_weight_sky(signal);
    //weights[EDGE] *= update_weight_edge(signal);
}

//fn update_weight_ground(signal: &Signal) -> f32 {
//    if signal.state == GROUND
//        && (matches!(signal.direction, Direction::Left)
//            || matches!(signal.direction, Direction::Right)
//            || matches!(signal.direction, Direction::Front)
//            || matches!(signal.direction, Direction::Back))
//    {
//        return 20.0;
//    } else if signal.state == SKY
//        && matches!(signal.direction, Direction::Down)
//        && signal.distance == 1
//    {
//        return 0.00;
//    } else {
//        return 1.0;
//    }
//}
fn update_weight_ground(signal: &Signal) -> f32 {
    //if signal.state == GROUND && matches!(signal.direction, Direction::Down) && signal.distance == 1 {
    //    return 100.0;
    //}
    return 1.0;
}
fn update_weight_sky(signal: &Signal) -> f32 {
    return 1.0;
}

//fn update_weight_sky(signal: &Signal) -> f32 {
//    if signal.state == GROUND && matches!(signal.direction, Direction::Up) && signal.distance == 1 {
//        return 0.00;
//    } else {
//        return 1.0;
//    }
//}

fn update_weight_edge(signal: &Signal) -> f32 {
    return 1.0;
}

pub fn init_ground(grid: &Grid, coordinate: &Coordinate) -> f32 {
    if coordinate.z < 5 {
        return 1.0;
    }
    return 0.0;
}

pub fn init_sky(grid: &Grid, coordinate: &Coordinate) -> f32 {
    return 1.0 - init_ground(grid, coordinate);
}

pub fn update_ground(signal: &Signal) -> f32 {
    return 1.0;
}

pub fn update_sky(signal: &Signal) -> f32 {
    return 0.0;
}

pub struct Statev3<'a> {
    name: &'a str,
    init: fn(grid: &'a Grid, coordinate: &'a Coordinate) -> f32,
    update: fn(signal: &'a Signal<'a>) -> f32,
}

pub struct Weightsv2<'a> {
    states: [Statev3<'a>; STATE_COUNT],
}
impl<'a> Weightsv2<'a> {
    pub fn new() -> Self {
        let states = [
            Statev3 {name: "ground", init: init_ground, update: update_ground},
            Statev3 {name: "sky", init: init_sky, update: update_sky},
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn states_new() {
        let weights = Weightsv2::new();
        let signal = Signal {state: 0, direction: &Direction::Left, distance: 1};
        //println!("{:?}", weights.init_weights2(&signal));
        println!("{:?}", weights.update(&signal));
        //let states = [
        //    Statev2 {name: "ground", init: init_ground, update: update_ground},
        //    Statev2 {name: "sky", init: init_sky, update: update_sky},
        //];
        //println!("{:?}", update_vector(&states, &Signal { state: 1, direction: &Direction::Back, distance: 0 }));
        assert!(false);
    }
}