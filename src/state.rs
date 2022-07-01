use crate::math::{fermi_dirac, gaussian};
use crate::grid::{Coordinate, Direction, Grid};
use crate::states::Signal;

pub mod ground {
    use super::*;

    pub fn init(grid: &Grid, coordinate: &Coordinate) -> f32 {
        let a = 1.0;
        let x_0 = grid.width as f32 / 2.0;
        let y_0 = grid.depth as f32 / 2.0;
        let s_x = grid.width as f32 * 0.2;
        let s_y = s_x;
        let x = coordinate.x as f32;
        let y = coordinate.y as f32;
        return (1.0 - sky::init(grid, coordinate))
            * (gaussian(a, x_0, y_0, s_x, s_y, x, y));
    }

    pub fn update(signal: &Signal) -> f32 {
        return 1.0;
    }
}

pub mod sky {
    use super::*;

    pub fn init(grid: &Grid, coordinate: &Coordinate) -> f32 {
        let a = 2.0;
        let u = 0.0;
        let kt = grid.height as f32 * 0.5;
        let x = grid.height as f32 - coordinate.z as f32;
        return fermi_dirac(a, u, kt, x);
    }

    pub fn update(signal: &Signal) -> f32 {
        return 1.0;
    }
}
