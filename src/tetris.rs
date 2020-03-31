use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Read, Write, stdout};
use std::{thread};
use termion::{async_stdin, color};
use std::time::Duration;

use crate::tetromino::Tetromino;
use crate::traits::*;
use crate::rotation::Rotation;

pub const BLOCK_SIZE_X: usize = 2;
pub const BLOCK_SIZE_Y: usize = 1;

const BOARD_SIZE_X: usize = 10 * BLOCK_SIZE_X;
const BOARD_SIZE_Y: usize = 20;

const HUD_SIZE_X: usize = 5;
const HUD_SIZE_Y: usize = BOARD_SIZE_Y;

const BORDER_SIZE: usize = 1;

pub const GAME_SIZE_X: usize = BOARD_SIZE_X + HUD_SIZE_X + BORDER_SIZE * 3;
pub const GAME_SIZE_Y: usize = BOARD_SIZE_Y + BORDER_SIZE * 2;

pub const HEADER_SIZE_X: usize = BLOCK_SIZE_Y * 3;
pub const HEADER_SIZE_Y: usize = BLOCK_SIZE_Y * 4;

// pub type Screen<'a> = [[&'a str; GAME_SIZE_X]; GAME_SIZE_Y];
pub type Screen = Vec<Vec<String>>;

pub struct Tetris { 
    game: Screen,
    // board:[[Tetromino;BOARD_SIZE_X];BOARD_SIZE_Y],
    // score: u16,
    // current_tetromino: Tetromino,
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            game: vec![vec![String::from(" "); GAME_SIZE_X]; GAME_SIZE_Y],
            // game: [""; GAME_SIZE_X]
        }
    }

    pub fn play(&mut self) {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        //let mut stdout = stdout().into_raw_mode().unwrap();
        let mut stdin = async_stdin().bytes();

        loop {
            write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();
            stdout.flush().unwrap();

            let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
            
            let mut pos_x = 1;
            let mut pos_y = 1;

            if usize::from(terminal_width) > GAME_SIZE_X {
                pos_x = (usize::from(terminal_width) - GAME_SIZE_X) / 2;
            }

            if usize::from(terminal_height) > GAME_SIZE_Y {
                pos_y = (usize::from(terminal_height) - GAME_SIZE_Y) / 2;
            }

            self.print_border();
            self.print(pos_x, pos_y);
            
            let c = stdin.next();
            write!(stdout, "\r{:?}", c).unwrap();
            match c {
                Some(Ok(b'q')) => break,
                Some(Ok(3)) => break,
                _ => ()
            }

            thread::sleep(Duration::from_millis(100));
        }
    }

    fn print_border(&mut self) {
        // vertical Border
        for i in 1..GAME_SIZE_Y - 1 {
            self.game[i][0] = String::from("│");
            self.game[i][BOARD_SIZE_X] = String::from("│");
            self.game[i][GAME_SIZE_X - 1] = String::from("│");
        }

        // horizontal border
        for i in 1..GAME_SIZE_X - 1 {
            self.game[0][i] = String::from("─");
            self.game[GAME_SIZE_Y - 1][i] = String::from("─");
        }

        // edges
        self.game[0][0] = String::from("┌");
        self.game[GAME_SIZE_Y - 1][0] = String::from("└");
        self.game[0][GAME_SIZE_X - 1] = String::from("┐");
        self.game[GAME_SIZE_Y - 1][GAME_SIZE_X - 1] = String::from("┘");

        // board-hud seperator egdes
        self.game[0][BOARD_SIZE_X] = String::from("┬");
        self.game[GAME_SIZE_Y - 1][BOARD_SIZE_X] = String::from("┴");
    }

    fn print(&mut self, pos_x: usize, pos_y: usize) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        for i in 0..GAME_SIZE_Y {
            write!(stdout, "{}", termion::cursor::Goto(pos_x as u16, (pos_y + i) as u16)).unwrap();
            for j in 0..GAME_SIZE_X {
                write!(stdout, "{}", self.game[i][j]).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
}

