pub mod environment;
pub mod game;
pub mod snake;

use druid::Color;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 600.0;
const FOOD_COLOR: Color = Color::RED;
const SNAKE_COLOR: Color = Color::GREEN;
const BLOCK_SIZE: f64 = 25.0;
const MOVING_PERIOD: f64 = 10.0;
