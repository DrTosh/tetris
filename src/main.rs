mod tetris;
mod tetromino;
mod rotation;
mod traits;
mod boxing_characters;
use tetris::Tetris;

fn main() {
    let mut tetris = Tetris::new();
    tetris.play();  
    // println!("{}", format!("{} hello world in red", termion::color::Fg(color::Rgb(255, 0, 0))));
}