use std::time::{Duration, Instant};

use crate::game::{environment::Environment, snake::Direction, snake::Snake};
use crate::game::{BLOCK_SIZE, FOOD_COLOR, MOVING_PERIOD, SNAKE_COLOR};
use druid::{Data, Lens, Rect, RenderContext, Size, TimerToken, Widget};

// App Data
#[derive(Clone, Lens, Data)]
pub struct AppData {
    pub environment: Environment,
    pub snake: Snake,
    pub food_exist: bool,
}

impl AppData {
    pub fn iter_interval(&self) -> u64 {
        (1000. / MOVING_PERIOD) as u64
    }

    pub fn eating_apple(&mut self) {
        let (head_x, head_y) = self.snake.head_position();

        if self.food_exist && self.environment.food_x == head_x && self.environment.food_y == head_y
        {
            self.food_exist = false;
            self.snake.restore_tail();
        }
    }

    pub fn update_snake(&mut self, dir: Option<Direction>) {
        self.snake.move_forward(dir);
        self.eating_apple();
    }
}

// Game Widget
pub struct GameWidget {
    pub timer_id: TimerToken,
    pub last_update: Instant,
}

impl GameWidget {}

impl Widget<AppData> for GameWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut AppData,
        _env: &druid::Env,
    ) {
        match event {
            druid::Event::WindowConnected => {
                ctx.request_paint();
                ctx.request_focus();
                let deadline = Duration::from_millis(data.iter_interval());
                self.last_update = Instant::now();
                self.timer_id = ctx.request_timer(deadline);
            }
            druid::Event::Timer(id) => {
                if !data.food_exist {
                    data.environment.add_food();
                    data.food_exist = true;
                    ctx.request_paint();
                }

                if *id == self.timer_id {
                    data.update_snake(Some(data.snake.direction));
                    ctx.request_paint();
                    let deadline = Duration::from_millis(data.iter_interval());
                    self.last_update = Instant::now();
                    self.timer_id = ctx.request_timer(deadline);
                }
            }
            druid::Event::KeyUp(k) => {
                let dir = match k.key {
                    druid::keyboard_types::Key::ArrowDown => Some(Direction::Down),
                    druid::keyboard_types::Key::ArrowLeft => Some(Direction::Left),
                    druid::keyboard_types::Key::ArrowRight => Some(Direction::Right),
                    druid::keyboard_types::Key::ArrowUp => Some(Direction::Up),
                    _ => Some(data.snake.head_direction()),
                };

                if dir.unwrap() != data.snake.head_direction().opposite() {
                    data.snake.direction = dir.unwrap();
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &AppData,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        ctx: &mut druid::UpdateCtx,
        old_data: &AppData,
        data: &AppData,
        _env: &druid::Env,
    ) {
        if data.environment != old_data.environment {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &AppData,
        _env: &druid::Env,
    ) -> druid::Size {
        let max_size = bc.max();
        let min_side = max_size.height.min(max_size.width);
        Size {
            width: min_side,
            height: min_side,
        }
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppData, _env: &druid::Env) {
        for block in &data.snake.body {
            let block = Rect::new(
                block.x as f64,
                block.y as f64,
                block.x as f64 + BLOCK_SIZE,
                block.y as f64 + BLOCK_SIZE,
            );

            ctx.fill(block, &SNAKE_COLOR);
        }

        if data.food_exist {
            let food = Rect::new(
                data.environment.food_x as f64,
                data.environment.food_y as f64,
                data.environment.food_x as f64 + BLOCK_SIZE as f64,
                data.environment.food_y as f64 + BLOCK_SIZE as f64,
            );
            ctx.fill(food, &FOOD_COLOR);
        }
    }
}
