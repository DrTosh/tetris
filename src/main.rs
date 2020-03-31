mod tetris;
mod tetromino;
mod active_tetromino;
mod rotation;
mod traits;
use tetris::Tetris;

fn main() {
    let mut tetris = Tetris::new();
    tetris.play();  
}