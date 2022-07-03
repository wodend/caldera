use std::collections::HashMap;
use std::fmt;

use log::{debug, error, info, log_enabled, Level};

use crate::grid::{Coordinate, Direction, Grid};
use crate::state::{ground, sky};
use crate::Signal;

pub const SIZE: usize = 3;

pub struct State<'a> {
    name: &'a str,
    init: fn(grid: &'a Grid, coordinate: &'a Coordinate) -> f32,
    update: fn(signal: Signal) -> f32,
}

pub struct States<'a> {
    states: Vec<State<'a>>,
}
impl<'a> States<'a> {
    pub fn new() -> Self {
        let states = vec![
            State {
                name: "ground",
                init: ground::init,
                update: ground::update,
            },
            State {
                name: "sky",
                init: sky::init,
                update: sky::update,
            },
        ];
        return Self { states: states };
    }

    pub fn init(&self, grid: &'a Grid, coordinate: &'a Coordinate) -> Vec<f32> {
        return self
            .states
            .iter()
            .map(|state| (state.init)(grid, coordinate))
            .collect();
    }

    pub fn update(&self, signal: Signal) -> Vec<f32> {
        return self
            .states
            .iter()
            .map(|state| (state.update)(signal))
            .collect();
    }

    pub fn update_map(
        &self,
        directions: &Vec<Direction>,
        max_distance: usize,
    ) -> HashMap<Signal, Vec<f32>> {
        let mut update_map = HashMap::new();

        for state_id in 0..self.states.len() {
            for distance in 0..max_distance {
                for direction in directions.iter() {
                    let signal = Signal::new(state_id, *direction, distance);
                    update_map.insert(signal, self.update(signal));
                }
            }
        }
        return update_map;
    }

    pub fn names(&self) -> Vec<&str> {
        return self.states.iter().map(|state| state.name).collect();
    }
}
