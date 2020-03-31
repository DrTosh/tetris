mod tetris;
mod tetromino;
mod active_tetromino;
mod rotation;
mod traits;
mod boxing_characters;
use tetris::Tetris;

fn main() {
    let mut tetris = Tetris::new();
    tetris.play();  
}