use crate::game::BLOCK_SIZE;
use druid::{im::Vector, Data};

#[derive(Copy, Clone, PartialEq, Data)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Data, Debug, Clone)]
pub struct Block {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Data, Clone)]
pub struct Snake {
    pub(crate) direction: Direction,
    pub(crate) body: Vector<Block>,
    pub(crate) tail: Option<Block>,
}

impl Snake {
    pub fn new(x: f64, y: f64) -> Self {
        let mut body = Vector::<Block>::new();
        body.push_back(Block {
            x: x + 2.0 * BLOCK_SIZE,
            y,
        });
        body.push_back(Block {
            x: x + BLOCK_SIZE,
            y,
        });
        body.push_back(Block { x, y });

        return Self {
            direction: Direction::Right,
            body,
            tail: None,
        };
    }

    pub fn head_position(&self) -> (f64, f64) {
        let head_block = self.body.front().unwrap();
        return (head_block.x, head_block.y);
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) = self.head_position();
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - BLOCK_SIZE,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + BLOCK_SIZE,
            },
            Direction::Left => Block {
                x: last_x - BLOCK_SIZE,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + BLOCK_SIZE,
                y: last_y,
            },
        };

        self.body.push_front(new_block);

        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        return self.direction;
    }

    pub fn head_next(&self, dir: Option<Direction>) -> (f64, f64) {
        let (head_x, head_y) = self.head_position();
        let mut moving_dir = self.direction;

        match dir {
            Some(d) => moving_dir = d,
            None => (),
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - BLOCK_SIZE),
            Direction::Down => (head_x, head_y + BLOCK_SIZE),
            Direction::Left => (head_x - BLOCK_SIZE, head_y),
            Direction::Right => (head_x + BLOCK_SIZE, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: f64, y: f64) -> bool {
        let mut c = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            c += 1;
            if c == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}
