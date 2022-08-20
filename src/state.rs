use crate::space::{Direction, Point, Size};

type InitialProbabilityFn = fn(size: Size, point: Point) -> f32;
type UpdateProbabilityFn = fn(signal: Signal) -> f32;

#[derive(Clone, Copy)]
pub struct Signal {
    pub state_name: &'static str,
    pub direction: Direction,
    pub distance: usize,
}

impl Signal {
    pub fn new(state_name: &'static str, direction: Direction, distance: usize) -> Signal {
        Signal {
            state_name: state_name,
            direction: direction,
            distance: distance,
        }
    }
}

pub struct State {
    pub name: &'static str,
    pub initial_probability: InitialProbabilityFn,
    pub update_probability: UpdateProbabilityFn,
}

impl State {
    pub fn new(
        name: &'static str,
        initial_probability: InitialProbabilityFn,
        update_probability: UpdateProbabilityFn,
    ) -> State {
        State {
            name: name,
            initial_probability: initial_probability,
            update_probability: update_probability,
        }
    }
}
