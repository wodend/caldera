mod grid;
mod states;

use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use log::{debug, error, info, log_enabled, Level};
use rand;
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::grid::{Cell, Coordinate, Direction, Edge, Grid};
use crate::states::{Signal, State, StateNames, Weights};

#[derive(Debug)]
struct Model<'a> {
    rng: &'a mut ThreadRng,
    grid: &'a Grid,
    weights: Vec<Weights>,
    entropies: Vec<f32>,
    observations: Vec<Option<State>>,
    observed_count: usize,
}

impl<'a> Model<'a> {
    fn new(rng: &'a mut ThreadRng, grid: &'a Grid) -> Self {
        let mut weights = Vec::new();
        let mut entropies = Vec::new();
        let mut observations = Vec::new();
        let mut observed_count = 0;
        for cell in 0..grid.cell_count {
            let coordinate = &grid.coordinates[cell];
            let mut initial_weights = states::initial_weights(grid, coordinate);
            states::normalize(&mut initial_weights);
            let entropy = states::entropy(&initial_weights);
            if entropy.is_nan() {
                let observed_state = initial_weights.iter().position(|&p| p == 1.0);
                observations.push(observed_state);
                observed_count += 1;
            } else {
                observations.push(None);
            }
            let noise = rng.gen::<f32>() * 0.001;
            weights.push(initial_weights);
            entropies.push(entropy + noise);
        }
        return Self {
            rng: rng,
            grid: grid,
            weights: weights,
            entropies: entropies,
            observations: observations,
            observed_count: observed_count,
        };
    }

    fn min_entropy_cell(&self) -> Cell {
        let mut min_entropy_cell = None;
        let mut min_entropy = f32::MAX; // 2^32
        for cell in 0..self.grid.cell_count {
            if self.observations[cell] == None && self.entropies[cell] < min_entropy {
                min_entropy_cell = Some(cell);
                min_entropy = self.entropies[cell];
            }
        }
        match min_entropy_cell {
            Some(c) => return c,
            None => panic!("No min entropy found"),
        };
    }

    fn observe(&mut self, cell: Cell) {
        let dist = WeightedIndex::new(self.weights[cell]).unwrap(); // TODO: Add error handling
        let observed_state = dist.sample(self.rng);
        self.observations[cell] = Some(observed_state);
        self.observed_count += 1;
    }

    fn propagate(&mut self, cell: Cell) {
        let max_distance = 5;
        let mut stack= vec![(cell,0)];
        let mut visited = HashSet::new();
        while let Some((current_cell, distance)) = stack.pop() {
            visited.insert(current_cell);
            let neighbor_distance = distance + 1 as usize;
            debug!("Propagating from {}", self.cell_str(current_cell));
            for Edge { direction, cell } in self.grid.graph[current_cell].iter() {
                match self.observations[current_cell] {
                    // Propagate to neighbors if cell collapsed
                    Some(state) => {
                        if !visited.contains(cell) && self.observations[*cell] == None {
                            // TODO: Add error handling for contradiction during update
                            let signal = Signal::new(state, direction, neighbor_distance);
                            debug!("Updating {} with {}", self.cell_str(*cell), signal);
                            states::update(&mut self.weights[*cell], &signal);
                            states::normalize(&mut self.weights[*cell]);
                            debug!("Updated to {}", self.cell_str(*cell));
                            let entropy = states::entropy(&self.weights[*cell]);
                            self.entropies[*cell] = entropy;
                            if entropy.is_nan() {
                                self.observe(*cell);
                                stack.push((*cell, 0));
                            }
                        }
                    },
                    // Propagate up to max_distance
                    None => {
                        if neighbor_distance < max_distance {
                            stack.push((neighbor_distance, *cell))
                        }
                    },
                }
            }
        }
    }

    fn wfc(&mut self) {
        while self.observed_count < self.grid.cell_count {
            let min_entropy_cell = self.min_entropy_cell();
            debug!("Observing {}", self.cell_str(min_entropy_cell));
            self.observe(min_entropy_cell);
            self.propagate(min_entropy_cell);
        }
        self.render();
    }

    fn render(&self) {
        // TODO: Add error handling
        let output_file = "mv_import.txt";
        let file = File::create(output_file).expect("Unable to create vox viewer file");
        let mut writer = BufWriter::new(file);
        writer
            .write("// Generated by Caldera\n".as_bytes())
            .unwrap();
        let max_dimension_size = std::cmp::max(
            self.grid.width,
            std::cmp::max(self.grid.depth, self.grid.height),
        );
        let mv_import_size = max_dimension_size * states::SIZE;
        let header = format!(
            "mv_import {mv_import_size}\n",
            mv_import_size = mv_import_size
        );
        writer.write(header.as_bytes()).unwrap();
        let state_names = states::names();
        for cell in 0..self.grid.cell_count {
            let coordinate = &self.grid.coordinates[cell];
            let x = coordinate.x * states::SIZE;
            let y = coordinate.y * states::SIZE;
            let z = coordinate.z * states::SIZE;
            let state = self.observations[cell].unwrap();
            let state_name = state_names[state];
            let path = Path::new("src/states")
                .join(state_name)
                .with_extension("vox");
            let absolute_path = path.canonicalize().unwrap();
            let path_str = absolute_path.to_str().unwrap();
            let mv_import_line =
                format!("{x} {y} {z} {path}\n", x = x, y = y, z = z, path = path_str,);
            writer.write(mv_import_line.as_bytes()).unwrap();
        }
    }

    fn cell_str(&self, cell: usize) -> String {
        let observation = match self.observations[cell] {
            Some(state) => states::names()[state],
            None => "None",
        };
        return format!(
            "Cell{{id={}, weights={:?}, entropy={}, observation={}}}",
            cell,
            self.weights[cell],
            self.entropies[cell],
            observation,
        );
    }
}

impl<'a> fmt::Display for Model<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Model")?;
        for cell in 0..self.grid.cell_count {
            let names = states::names();
            let observation = match self.observations[cell] {
                Some(state) => names[state],
                None => "None",
            };
            writeln!(
                f,
                "Cell {} {:?} Weights {:?} Entropy {} Observation {}",
                cell,
                self.grid.coordinates[cell],
                self.weights[cell],
                self.entropies[cell],
                observation,
            )?;
        }
        return Ok(());
    }
}

fn main() {
    env_logger::init();
    let mut rng = rand::thread_rng();
    let x = 40;
    let grid = Grid::new(x, x, x);
    let mut model = Model::new(&mut rng, &grid);
    info!("Running...");
    model.wfc();
    info!("Done");
}