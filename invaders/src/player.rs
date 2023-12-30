use crate::{NUM_COLS, NUM_ROWS, frame::Drawable};

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    // Create a Player
    pub fn new() -> Self {
        Self {
        x: NUM_COLS / 2,
        y: NUM_ROWS - 1,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "A"; // Spaceship Icon
    }
}
