mod tetris;
mod tetromino;
mod active_tetromino;
mod rotation;
use tetris::Tetris;

fn main() {
    let mut tetris = Tetris::new();
    tetris.play();  
}