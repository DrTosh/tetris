use crate::tetromino::Tetromino;
use crate::tetris::{Screen, BLOCK_SIZE_X, BLOCK_SIZE_Y};
use crate::rotation::Rotation;

#[derive(Debug)]
pub struct ActiveTetromino {
    pub tetromino: Tetromino,
    pub rotation: Rotation,
    pub pos_x: usize,
    pub pos_y: usize,
    pub finished: bool 
}

impl ActiveTetromino {
    pub fn new(pos_x: usize, pos_y: usize, tetromino: Tetromino) -> ActiveTetromino {
        ActiveTetromino {
            tetromino,
            rotation: Rotation::DEGREE_0,
            pos_x,
            pos_y,
            finished: false
        }
    }

    pub fn update(&mut self, screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) -> bool {
        self.tetromino.update(screen, self.pos_x, self.pos_y, self.rotation, true);

        if !self.tetromino.collide(screen, pos_x, pos_y, rotation) {

            self.pos_x = pos_x;
            self.pos_y = pos_y;
            self.rotation = rotation;

            self.tetromino.update(screen, pos_x, pos_y, rotation, false);
            return true;
        } else {
            self.tetromino.update(screen, self.pos_x, self.pos_y, self.rotation, false);
            return false;
        }
    }
        
    pub fn rotate(&mut self, screen: &mut Screen) {
        self.update(screen, self.pos_x, self.pos_y, self.rotation.next());
    }

    pub fn move_right(&mut self, screen: &mut Screen) {
        self.update(screen, self.pos_x + BLOCK_SIZE_X, self.pos_y, self.rotation);
    }

    pub fn move_left(&mut self, screen: &mut Screen) {
        if self.pos_x >= BLOCK_SIZE_X {
            self.update(screen, self.pos_x - BLOCK_SIZE_X, self.pos_y, self.rotation);
        }
    }

    pub fn move_down(&mut self, screen: &mut Screen) {
        if !self.update(screen, self.pos_x, self.pos_y + BLOCK_SIZE_Y, self.rotation) {
            self.finished = true;
        } 
    }
}