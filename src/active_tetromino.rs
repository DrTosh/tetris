use crate::tetromino::Tetromino;
use crate::tetris::Screen;
use crate::rotation::Rotation;

#[derive(Debug)]
pub struct ActiveTetromino {
    pub tetromino: Tetromino,
    pub rotation: Rotation,
    pub pos_x: usize,
    pub pos_y: usize
}

impl ActiveTetromino {
    pub fn new(pos_x: usize, pos_y: usize) -> ActiveTetromino {
        ActiveTetromino {
            tetromino: Tetromino::O,
            rotation: Rotation::DEGREE_0,
            pos_x,
            pos_y,
        }
    }

    pub fn update(&mut self, screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        self.tetromino.update(screen, self.pos_x, self.pos_y, self.rotation, true);

        self.pos_x = pos_x;
        self.pos_y = pos_y;
        self.rotation = rotation;

        crate::Tetris::log(format!("{:?}", self));

        self.tetromino.update(screen, pos_x, pos_y, rotation, false);
    }
}