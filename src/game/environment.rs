use crate::game::{BLOCK_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};
use druid::Data;
use rand::{thread_rng, Rng};

#[derive(Clone, Data, PartialEq)]
pub struct Environment {
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) food_x: f64,
    pub(crate) food_y: f64,
}

impl Environment {
    pub fn new(width: f64, height: f64) -> Self {
        return Self {
            width,
            height,
            food_x: 0.0,
            food_y: 0.0,
        };
    }

    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        let min_x: u32 = 0;
        let max_x: u32 = (WINDOW_WIDTH / BLOCK_SIZE) as u32;

        let min_y: u32 = 0;
        let max_y: u32 = (WINDOW_HEIGHT / BLOCK_SIZE) as u32;

        let new_x: u32 = rng.gen_range(min_x..max_x);
        let new_y: u32 = rng.gen_range(min_y..max_y);

        self.food_x = new_x as f64 * BLOCK_SIZE;
        self.food_y = new_y as f64 * BLOCK_SIZE;
    }
}
