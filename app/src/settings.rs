#![allow(unused)] // FIXME
use macroquad::color::{DARKGRAY, GRAY};
use macroquad::prelude::{Color, Vec2, ORANGE, YELLOW};

// pub const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
pub const GROUND_COLOR: Color = GRAY;
pub const UNIT_COLOR: Color = DARKGRAY;
pub const SELECTOR_COLOR: Color = YELLOW;
// pub const UNIT_SIZE: (f32, f32) = (60.0, 75.0);
pub const UNIT_SIZE: f32 = 40.;

pub const UNIT_SPEED: f32 = 130.0;
pub const UNIT_ROTATION_SPEED: f32 = 4.0;

pub const LINE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
pub const TRANSPARENT: Color = Color::new(0., 0., 0., 0.);
