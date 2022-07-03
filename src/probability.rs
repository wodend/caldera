use crate::math;
use crate::{Coordinate, Dimensions, Direction, Signal};

pub mod init {
    use super::*;

    pub fn ground(dimensions: Dimensions, coordinate: Coordinate) -> f32 {
        let a = 1.0;
        let x_0 = dimensions.width as f32 / 2.0;
        let y_0 = dimensions.depth as f32 / 2.0;
        let s_x = dimensions.width as f32 * 0.9;
        let s_y = s_x;
        let x = coordinate.x as f32;
        let y = coordinate.y as f32;
        return (1.0 - init::sky(dimensions, coordinate))
            * (math::gaussian(a, x_0, y_0, s_x, s_y, x, y));
    }

    pub fn sky(dimensions: Dimensions, coordinate: Coordinate) -> f32 {
        let a = 2.0;
        let u = 0.0;
        let kt = dimensions.height as f32 * 0.5;
        let x = dimensions.height as f32 - coordinate.z as f32;
        return math::fermi_dirac(a, u, kt, x);
    }
}

pub mod update {
    use super::*;

    pub fn ground(signal: Signal) -> f32 {
        return 1.0;
    }

    pub fn sky(signal: Signal) -> f32 {
        return 1.0;
    }
}
