use crate::{Coordinate, Dimensions, Signal};

pub type StateInitFn = fn(dimensions: Dimensions, coordinate: Coordinate) -> f32;
pub type StateUpdateFn = fn(signal: Signal) -> f32;

pub struct State {
    name: &'static str,
    init: StateInitFn,
    update: StateUpdateFn,
}

impl State {
    pub fn new(name: &'static str, init: StateInitFn, update: StateUpdateFn) -> Self {
        return Self {
            name: name,
            init: init,
            update: update,
        };
    }
}

pub struct States {
    size: usize,
    states: Vec<State>,
}
impl States {
    pub fn new(size: usize, states: Vec<State>) -> Self {
        return Self {
            size: size,
            states: states,
        };
    }

    pub fn names(&self) -> Vec<&str> {
        return self.states.iter().map(|state| state.name).collect();
    }

    pub fn init(&self, dimensions: Dimensions, coordinate: Coordinate) -> Vec<f32> {
        return self
            .states
            .iter()
            .map(|state| (state.init)(dimensions, coordinate))
            .collect();
    }

    pub fn update(&self, signal: Signal) -> Vec<f32> {
        return self
            .states
            .iter()
            .map(|state| (state.update)(signal))
            .collect();
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn len(&self) -> usize {
        return self.states.len();
    }
}
