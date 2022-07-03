use crate::math;
use crate::states::States;
use crate::{Coordinate, Dimensions, Direction};

use rand::rngs::ThreadRng;
use rand::Rng;

fn cell_id(coordinate: Coordinate, dimensions: Dimensions) -> usize {
    return coordinate.x
        + (coordinate.y * dimensions.width)
        + (coordinate.z * dimensions.width * dimensions.depth);
}

fn edges(coordinate: Coordinate, dimensions: Dimensions) -> Vec<Edge> {
    let mut edges = Vec::new();
    if coordinate.x > 0 {
        let cell_id = cell_id(
            Coordinate::new(coordinate.x - 1, coordinate.y, coordinate.z),
            dimensions,
        );
        edges.push(Edge::new(cell_id, Direction::Right));
    }
    if coordinate.x < dimensions.width - 1 {
        let cell_id = cell_id(
            Coordinate::new(coordinate.x + 1, coordinate.y, coordinate.z),
            dimensions,
        );
        edges.push(Edge::new(cell_id, Direction::Left));
    }
    if coordinate.y > 0 {
        let cell_id = cell_id(
            Coordinate::new(coordinate.x, coordinate.y - 1, coordinate.z),
            dimensions,
        );
        edges.push(Edge::new(cell_id, Direction::Back));
    }
    if coordinate.y < dimensions.depth - 1 {
        let cell_id = cell_id(
            Coordinate::new(coordinate.x, coordinate.y + 1, coordinate.z),
            dimensions,
        );
        edges.push(Edge::new(cell_id, Direction::Front));
    }
    if coordinate.z > 0 {
        let cell_id = cell_id(
            Coordinate::new(coordinate.x, coordinate.y, coordinate.z - 1),
            dimensions,
        );
        edges.push(Edge::new(cell_id, Direction::Up));
    }
    if coordinate.z < dimensions.height - 1 {
        let cell_id = cell_id(
            Coordinate::new(coordinate.x, coordinate.y, coordinate.z + 1),
            dimensions,
        );
        edges.push(Edge::new(cell_id, Direction::Down));
    }
    return edges;
}

pub struct Edge {
    pub cell_id: usize,
    pub direction: Direction,
}

impl Edge {
    fn new(cell_id: usize, direction: Direction) -> Self {
        return Self {
            cell_id: cell_id,
            direction: direction,
        };
    }
}

pub struct Cells {
    pub cell_count: usize,
    pub coordinates: Vec<Coordinate>,
    pub graph: Vec<Vec<Edge>>,
    pub wave: Vec<Vec<f32>>,
    pub entropies: Vec<f32>,
    pub observations: Vec<Option<usize>>,
}

impl Cells {
    pub fn new(rng: &mut ThreadRng, dimensions: Dimensions, states: States) -> Self {
        let cell_count = dimensions.width * dimensions.depth * dimensions.height;
        let mut coordinates = Vec::new();
        let mut graph = Vec::new();
        let mut wave = Vec::new();
        let mut entropies = Vec::new();
        let mut observations = Vec::new();
        for z in 0..dimensions.height {
            for y in 0..dimensions.depth {
                for x in 0..dimensions.width {
                    let coordinate = Coordinate::new(x, y, z);
                    coordinates.push(coordinate);
                    graph.push(edges(coordinate, dimensions));
                    let mut weights = states.init(dimensions, coordinate);
                    math::normalize(&mut weights);
                    wave.push(weights.clone());
                    let entropy = math::entropy(&weights);
                    let noise = rng.gen::<f32>() * 0.001;
                    entropies.push(entropy + noise);
                    if entropy.is_nan() {
                        let observed_state = weights.iter().position(|&p| p == 1.0);
                        observations.push(observed_state);
                    } else {
                        observations.push(None);
                    }
                }
            }
        }
        return Self {
            cell_count: cell_count,
            coordinates: coordinates,
            graph: graph,
            wave: wave,
            entropies: entropies,
            observations: observations,
        };
    }
}
