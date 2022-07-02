mod grid;
mod states;
mod math;
mod state;

use std::collections::{HashSet, HashMap};
use std::fmt;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::path::Path;

use log::{debug, error, info, log_enabled, Level};
use rand;
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::ThreadRng;
use rand::Rng;

use crate::grid::{Cell, Coordinate, Direction, Edge, Grid};
use crate::states::{Signal, State, StateNames, Weights, STATE_COUNT, Weightsv2};

#[derive(Debug)]
struct Model<'a> {
    rng: &'a mut ThreadRng,
    grid: &'a Grid,
    weightsv2: &'a Weightsv2<'a>,
    weights: Vec<Weights>,
    entropies: Vec<f32>,
    observations: Vec<Option<State>>,
    update_map: HashMap<Signal, [f32; 2]>,
    observed_count: usize,
    max_distance: usize,
}

impl<'a> Model<'a> {
    fn new(rng: &'a mut ThreadRng, grid: &'a Grid, max_distance: usize, weightsv2: &'a Weightsv2<'a>) -> Self {
        let mut weights = Vec::with_capacity(grid.cell_count);
        let mut entropies = Vec::with_capacity(grid.cell_count);
        let mut observations = Vec::with_capacity(grid.cell_count);
        let mut observed_count = 0;
        let update_map = weightsv2.update_map(&grid.directions, max_distance);
        // get update map from weightsv2
        //let update_map = HashMap::new();
        for cell in 0..grid.cell_count {
            let coordinate = &grid.coordinates[cell];
            let mut initial_weights = weightsv2.init(grid, coordinate);
            //let mut initial_weights = states::initial_weights(grid, coordinate);
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
            weightsv2: weightsv2,
            weights: weights,
            entropies: entropies,
            observations: observations,
            update_map: update_map,
            observed_count: observed_count,
            max_distance: max_distance,
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
        let mut stack = vec![(cell, 0)];
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
                            //let weight_vectors = Weightsv2::new();
                            //let weightsv2 = Weightsv2::new();
                            // TODO: Add error handling for contradiction during update
                            let signal = Signal::new(state, *direction, neighbor_distance);
                            debug!("Updating {} with {}", self.cell_str(*cell), signal);
                            let update_vector = self.update_map[&signal];
                            //let update_vector = weightsv2.update(signal);
                            for state in 0..STATE_COUNT {
                                self.weights[*cell][state] *= update_vector[state];
                            }
                            //states::update(&mut self.weights[*cell], &signal);
                            // tmp debugging
                            assert!(!self.weights[*cell].iter().any(|&x| x < 0.0));
                            // tmp debugging
                            states::normalize(&mut self.weights[*cell]);
                            debug!("Updated to {}", self.cell_str(*cell));
                            let entropy = states::entropy(&self.weights[*cell]);
                            self.entropies[*cell] = entropy;
                            if entropy.is_nan() {
                                self.observe(*cell);
                                stack.push((*cell, 0));
                            }
                        }
                    }
                    // Propagate up to max_distance
                    None => {
                        if neighbor_distance < self.max_distance {
                            stack.push((neighbor_distance, *cell))
                        }
                    }
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
        //let state_names = states::names();
        let state_names = self.weightsv2.names();
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
            cell, self.weights[cell], self.entropies[cell], observation,
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
    let x = 20;
    let grid = Grid::new(x, x, x);
    let weightsv2 = Weightsv2::new();
    let mut model = Model::new(&mut rng, &grid, 3, &weightsv2);
    info!("{}", model);
    info!("Running...");
    model.wfc();
    info!("Done");
}
