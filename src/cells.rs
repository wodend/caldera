use std::fmt;

use crate::graph::Edge;
use crate::space::{Direction, Point};

pub struct Cell {
    pub point: Point,
    pub observation: Option<&'static str>,
}

impl Cell {
    pub fn new(point: Point, observation: Option<&'static str>) -> Cell {
        Cell {
            point: point,
            observation: observation,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(point={}, observation={})",
            self.point,
            self.observation.unwrap_or("None"),
        )
    }
}

pub struct Cells {
    pub count: usize,
    pub points: Vec<Point>,
    pub edges: Vec<Vec<Edge>>,
    pub weights: Vec<Vec<f32>>,
    pub entropies: Vec<f32>,
    pub observations: Vec<Option<&'static str>>,
}

impl Cells {
    pub fn with_capacity(capacity: usize) -> Cells {
        Cells {
            count: 0,
            points: Vec::with_capacity(capacity),
            edges: Vec::with_capacity(capacity),
            weights: Vec::with_capacity(capacity),
            entropies: Vec::with_capacity(capacity),
            observations: Vec::with_capacity(capacity),
        }
    }

    pub fn ids(&self) -> std::ops::Range<usize> {
        0..self.count
    }

    pub fn add(
        &mut self,
        point: Point,
        edges: Vec<Edge>,
        weights: Vec<f32>,
        entropy: f32,
        observation: Option<&'static str>,
    ) {
        self.count += 1;
        self.points.push(point);
        self.edges.push(edges);
        self.weights.push(weights);
        self.entropies.push(entropy);
        self.observations.push(observation);
    }

    pub fn get(&self, id: usize) -> Cell {
        Cell::new(self.points[id], self.observations[id])
    }
}
