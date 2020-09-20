//! Config files
//! 
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 0.0];
pub const PURPLE: [f32; 4] = [1.0, 0.0, 1.0, 0.0];
pub const BLUE: [f32; 4] = [0.0, 1.0, 1.0, 0.0];
pub const WHITE: [f32; 4] = [1.0; 4];

pub const COLOR_NUM: usize = 6;
pub const COLOR_LIST: [[f32; 4]; COLOR_NUM] =[GREEN, RED, YELLOW, PURPLE, BLUE, WHITE];

pub const MAX_TICK: usize = 100;