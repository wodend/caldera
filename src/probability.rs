use crate::math;
use crate::space::{Direction, Point, Size};
use crate::state::Signal;

const HORIZONTAL_DIRECTIONS: [Direction; 4]= [
    Direction::Left,
    Direction::Right,
    Direction::Front,
    Direction::Back,
];

pub mod initial {
    use super::*;
    
    pub fn test_bottom(dimensions: Size, point: Point) -> f32 {
        if point.z == 0 {
            1.0
        } else {
            0.0
        }
    }

    pub fn test_edge(dimensions: Size, point: Point) -> f32 {
        if point.x == 0 || point.x == dimensions.width - 1 {
            1.0
        } else if point.y == 0 || point.y == dimensions.depth - 1 {
            1.0
        } else {
            0.0
        }
    }

    pub fn test_sky(dimensions: Size, point: Point) -> f32 {
        //point.z as f32 / dimensions.height as f32
        f32::exp((point.z as f32 / dimensions.height as f32) * 2.0) - 1.0
    }


    pub fn ground(dimensions: Size, point: Point) -> f32 {
        let a = 1.0;
        let x_0 = dimensions.width as f32 / 2.0;
        let y_0 = dimensions.depth as f32 / 2.0;
        let s_x = dimensions.width as f32 * 0.9;
        let s_y = s_x;
        let x = point.x as f32;
        let y = point.y as f32;
        (1.0 - sky(dimensions, point)) * math::gaussian(a, x_0, y_0, s_x, s_y, x, y)
    }

    pub fn sky(dimensions: Size, point: Point) -> f32 {
        f32::exp((point.z as f32 / dimensions.height as f32) * 4.0) - 1.0
        // if point.z == 0 {
        //     0.0
        // } else {
        //     0.5
        // }
    }
}

pub mod update {
    use super::*;

    pub fn test_ground(signal: Signal) -> f32 {
        if signal.state_name == "sky"
            && signal.direction == Direction::Down
            && signal.distance == 1
        {
            -1.0
        } else {
            0.0
        }
    }

    pub fn test_flat(signal: Signal) -> f32 {
        if signal.state_name == "ground"
            && HORIZONTAL_DIRECTIONS.iter().any(|d| *d == signal.direction)
            && signal.distance == 1
        {
            0.5
        } else {
            0.0
        }
    }

    pub fn test_sky(signal: Signal) -> f32 {
        if (
            signal.state_name == "ground"
            || signal.state_name == "edge"
            )
            && signal.direction == Direction::Up
            && signal.distance == 1
        {
            -1.0
        } else {
            0.0
        }
    }



    pub fn ground(signal: Signal) -> f32 {
        if signal.state_name == "ground"
            && HORIZONTAL_DIRECTIONS.iter().any(|d| *d == signal.direction)
            && signal.distance == 1
        {
            0.5
        } else if signal.state_name == "sky"
            && signal.direction == Direction::Down
        {
            -1.0
        } else {
            0.0
        }
    }

    pub fn sky(signal: Signal) -> f32 {
        0.0
    }
}
