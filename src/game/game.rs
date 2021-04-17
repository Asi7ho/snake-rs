use std::time::{Duration, Instant};

use crate::game::{environment::Environment, snake::Direction, snake::Snake};
use crate::game::{
    BLOCK_SIZE, FOOD_COLOR, GAME_OVER_COLOR, MOVING_PERIOD, RESTART_TIME, SNAKE_COLOR,
};
use druid::{Data, Lens, Rect, RenderContext, Size, TimerToken, Widget};

// App Data
#[derive(Clone, Lens, Data)]
pub struct AppData {
    pub environment: Environment,
    pub snake: Snake,
    pub food_exist: bool,
    pub game_over: bool,
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
        if self.check_bounds(dir) {
            self.snake.move_forward(dir);
            self.eating_apple();
        } else {
            self.game_over = true;
        }
    }

    pub fn check_bounds(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.head_next(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        return next_x > -BLOCK_SIZE
            && next_y > -BLOCK_SIZE
            && next_x < self.environment.width
            && next_y < self.environment.height;
    }

    pub fn restart(&mut self) {
        self.snake = Snake::new(50.0, 50.0);
        self.game_over = false;
        self.food_exist = false;
        self.environment = Environment::new(self.environment.width, self.environment.height);
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
                if data.game_over {
                    let waiting_time = Instant::now();
                    let deadline = Duration::from_millis(data.iter_interval());
                    self.timer_id = ctx.request_timer(deadline);
                    if waiting_time - self.last_update > Duration::from_secs_f64(RESTART_TIME) {
                        data.restart();
                    }
                    ctx.request_paint();
                    return;
                }

                if !data.food_exist {
                    data.environment.add_food();
                    while data
                        .snake
                        .overlap_tail(data.environment.food_x, data.environment.food_y)
                    {
                        data.environment.add_food();
                    }
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
                if data.game_over {
                    return;
                }

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
        if data.game_over {
            let rect = Rect::new(0.0, 0.0, data.environment.width, data.environment.height);
            ctx.fill(rect, &GAME_OVER_COLOR);
        }

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
