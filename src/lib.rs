mod cells;
mod graph;
mod math;
mod probability;
mod space;
mod state;

use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::rngs::ThreadRng;
use rand::thread_rng;

use cells::{Cell, Cells};
use graph::Edge;
use probability::{initial, update};
use space::{Point, Size};
use state::{Signal, State};

#[derive(Debug)]
pub enum MapSize {
    TestOne,
    Small,
    Medium,
    Large,
}

#[derive(Debug)]
pub enum MapConfig {
    TestInitialWeightsError,
    TestContradiction,
    TestEdge,
    Test100Ground,
    Simple,
}

pub enum GenerationError {
    InitialWeightsError,
    Contradiction,
}

struct Node {
    pub current_cell_id: usize,
    pub current_state_name: &'static str,
    pub current_distance: usize,
}

impl Node {
    pub fn new(
        current_cell_id: usize,
        current_state_name: &'static str,
        current_distance: usize,
    ) -> Node {
        Node {
            current_cell_id: current_cell_id,
            current_state_name: current_state_name,
            current_distance: current_distance,
        }
    }
}

/// A map generator implementing the wave function collapse algorithm.
pub struct MapGenerator {
    size: MapSize,
    config: MapConfig,
    dimensions: Size,
    state_names: Vec<&'static str>,
    states: Vec<State>,
    tile_size: usize,
    max_distance: usize,
    cells: Cells,
    rng: ThreadRng,
}

impl MapGenerator {
    pub fn new(map_size: MapSize, map_config: MapConfig) -> MapGenerator {
        let dimensions = match map_size {
            MapSize::TestOne => space::Size::new(1, 1, 1),
            MapSize::Small => space::Size::new(10, 10, 10),
            MapSize::Medium => space::Size::new(20, 20, 20),
            MapSize::Large => space::Size::new(30, 30, 30),
        };
        let states = match map_config {
            MapConfig::TestInitialWeightsError => {
                vec![State::new("ground", |_, _| 0.0, |_| 0.0)]
            }
            MapConfig::TestContradiction => vec![State::new("ground", |_, _| 1.0, |_| -1.0)],
            MapConfig::Test100Ground => vec![
                State::new("ground", |_, _| 1.0, |_| 0.0),
                State::new("sky", |_, _| 0.0, |_| 0.0),
            ],
            MapConfig::TestEdge => vec![
                State::new("ground", |d, p| 1.0 - initial::test_sky(d, p) - initial::test_edge(d, p), |s| update::test_ground(s) + update::test_flat(s)),
                State::new("edge", |d, p| initial::test_edge(d, p) - initial::test_sky(d, p), update::test_ground),
                State::new("sky", |d, p| initial::test_sky(d, p), update::test_sky),
            ],
            MapConfig::Simple => vec![
                State::new("ground", initial::ground, update::ground),
                State::new("sky", initial::sky, update::sky),
            ],
        };
        let tile_size = match map_config {
            _ => 3,
        };
        let max_distance = match map_config {
            _ => 2,
        };
        let state_names = states
            .iter()
            .map(|state| state.name)
            .collect::<Vec<&'static str>>();
        let cell_count = dimensions.width * dimensions.depth * dimensions.height;
        let cells = Cells::with_capacity(cell_count);
        let rng = thread_rng();
        MapGenerator {
            size: map_size,
            config: map_config,
            tile_size: tile_size,
            dimensions: dimensions,
            states: states,
            max_distance: max_distance,
            state_names: state_names,
            cells: cells,
            rng: rng,
        }
    }

    fn initialize(&mut self) -> Result<(), GenerationError> {
        let noise_distribution = Uniform::from(-1.0..=1.0);
        for z in 0..self.dimensions.height {
            for y in 0..self.dimensions.depth {
                for x in 0..self.dimensions.width {
                    let point = Point::new(x, y, z);
                    let edges = graph::edges(self.dimensions, point);
                    let mut weights = self
                        .states
                        .iter()
                        .map(|state| (state.initial_probability)(self.dimensions, point))
                        .collect();
                    math::normalize(&mut weights);
                    let noise = noise_distribution.sample(&mut self.rng) * 0.001;
                    let mut entropy = math::entropy(&weights);
                    let mut observation = None;
                    if point.x == 0 && point.y == 0 && point.z > 10 {
                        println!("{}", point);
                        println!("{:?}", weights);
                    }
                    if entropy.is_nan() {
                        let state_id = weights.iter().position(|p| *p == 1.0);
                        observation = match state_id {
                            Some(id) => Some(self.state_names[id]),
                            None => return Err(GenerationError::InitialWeightsError),
                        };
                    } else {
                        entropy += noise;
                    }
                    self.cells.add(point, edges, weights, entropy, observation);
                }
            }
        }
        Ok(())
    }

    pub fn min_entropy_cell_id(&self) -> Option<usize> {
        let mut min_entropy_cell_id = None;
        let mut min_entropy = f32::NAN;
        for cell_id in self.cells.ids() {
            if self.cells.observations[cell_id].is_none()
                && (min_entropy.is_nan() || self.cells.entropies[cell_id] < min_entropy)
            {
                min_entropy_cell_id = Some(cell_id);
                min_entropy = self.cells.entropies[cell_id];
            }
        }
        min_entropy_cell_id
    }

    fn observe(&mut self, cell_id: usize) -> Result<(), GenerationError> {
        let distribution = match WeightedIndex::new(&self.cells.weights[cell_id]) {
            Ok(distribution) => distribution,
            Err(_) => return Err(GenerationError::Contradiction),
        };
        let state_id = distribution.sample(&mut self.rng);
        self.cells.observations[cell_id] = Some(self.state_names[state_id]);
        for (id, probability) in self.cells.weights[cell_id].iter_mut().enumerate() {
            if id == state_id {
                *probability = 1.0;
            } else {
                *probability = 0.0;
            }
        }
        self.cells.entropies[cell_id] = 0.0;
        Ok(())
    }

    fn propagate(&mut self, cell_id: usize) {
        let mut stack = match self.cells.observations[cell_id] {
            Some(state_name) => vec![Node::new(cell_id, state_name, 0)],
            None => Vec::new(),
        };
        let mut visited = HashSet::new();
        while let Some(Node {
            current_cell_id,
            current_state_name,
            current_distance,
        }) = stack.pop()
        {
            visited.insert(current_cell_id);
            for Edge { cell_id, direction } in self.cells.edges[current_cell_id].iter() {
                if !visited.contains(cell_id) {
                    let distance = current_distance + 1;
                    // Update cell if not collapsed
                    if self.cells.observations[*cell_id].is_none() {
                        let signal = Signal::new(current_state_name, *direction, distance);
                        let weights = self
                            .states
                            .iter()
                            .map(|state| (state.update_probability)(signal))
                            .collect();
                        math::add(&mut self.cells.weights[*cell_id], &weights);
                        math::normalize(&mut self.cells.weights[*cell_id]);
                        self.cells.entropies[*cell_id] =
                            math::entropy(&self.cells.weights[*cell_id]);
                    }
                    // Continue the propagation up to max_distance
                    if distance < self.max_distance {
                        stack.push(Node::new(*cell_id, current_state_name, distance));
                    }
                }
            }
        }
    }

    pub fn wave_function_collapse(&mut self) -> Result<(), GenerationError> {
        self.initialize()?;
        while let Some(cell_id) = self.min_entropy_cell_id() {
            self.observe(cell_id)?;
            self.propagate(cell_id);
        }
        Ok(())
    }

    pub fn write_mv_import(&self) -> Result<(), std::io::Error> {
        let file = File::create("mv_import.txt")?;
        let mut writer = BufWriter::new(file);
        writer
            .write("// Generated by Caldera\n".as_bytes())
            .unwrap();
        let max_dimension_size = std::cmp::max(
            self.dimensions.width,
            std::cmp::max(self.dimensions.depth, self.dimensions.height),
        );
        let mv_import_size = max_dimension_size * self.tile_size;
        let header = format!("mv_import {}\n", mv_import_size);
        writer.write(header.as_bytes())?;
        for id in self.cells.ids() {
            let cell = self.cells.get(id);
            match cell.observation {
                Some(state_name) => {
                    let x = cell.point.x * self.tile_size;
                    let y = cell.point.y * self.tile_size;
                    let z = cell.point.z * self.tile_size;
                    let path = PathBuf::from(format!("src/states/{}.vox", state_name));
                    let absolute_path = path.canonicalize()?;
                    let absolute_path_str = absolute_path.to_str();
                    match absolute_path_str {
                        Some(path) => {
                            let line = format!("{} {} {} {}\n", x, y, z, path);
                            writer.write(line.as_bytes()).unwrap();
                        }
                        None => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
                    }
                }
                None => (),
            }
        }
        Ok(())
    }

    pub fn cells(&self) -> Vec<Cell> {
        let mut cells = Vec::with_capacity(self.cells.count);
        for id in self.cells.ids() {
            let cell = self.cells.get(id);
            cells.push(cell);
        }
        cells
    }
}

impl fmt::Display for MapGenerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(size={:?}, config={:?})", self.size, self.config,)
    }
}
