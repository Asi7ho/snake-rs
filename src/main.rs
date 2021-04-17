use std::time::Instant;

use druid::{widget::Flex, AppLauncher, Size, TimerToken, Widget, WindowDesc};
use snake_rs::{AppData, Environment, GameWidget, Snake};

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 600.0;

fn make_widget() -> impl Widget<AppData> {
    Flex::column().with_flex_child(
        GameWidget {
            timer_id: TimerToken::INVALID,
            last_update: Instant::now(),
        },
        1.0,
    )
}

fn main() {
    let window = WindowDesc::new(make_widget)
        .window_size(Size {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        })
        .resizable(false)
        .title("Snake");

    let win = Environment::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    AppLauncher::with_window(window)
        .launch(AppData {
            environment: win,
            food_exist: false,
            snake: Snake::new(50.0, 50.0),
            game_over: false,
        })
        .expect("launch failed");
}
