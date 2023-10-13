use bevy::prelude::*;

// dimensions are in m

pub const RESTITUTION_COEFFICIENT: f32 = 0.7;

pub const TERRAIN_HALF_X: f32 = 13.4 / 2.0; // 13.4 m
pub const TERRAIN_HALF_Z: f32 = 10.4 / 2.0; // 10.4 m

pub const BALL_RADIUS: f32 = 0.04267 / 2.0; // 42.67 mm diameter

pub const ROBOT_RADIUS: f32 = 0.15 / 2.0; // 15 cm diameter
pub const ROBOT_HEIGHT: f32 = 0.18; // 18 cm height

pub const TERRAIN_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);
pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.0);
pub const ROBOT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

pub const TIME_STEP : f32 = 1.0 / 60.0;