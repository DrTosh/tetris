use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Read, Write, stdout};
use std::{thread};
use termion::{async_stdin, color, clear, cursor};
use std::time::{Duration, SystemTime};
use std::fs::{File, OpenOptions};

use crate::tetromino::Tetromino;
use crate::active_tetromino::ActiveTetromino;
use crate::traits::*;
use crate::rotation::Rotation;


pub const BLOCK_SIZE_X: usize = 2;
pub const BLOCK_SIZE_Y: usize = 1;

const BOARD_SIZE_X: usize = 10 * BLOCK_SIZE_X;
const BOARD_SIZE_Y: usize = 20 * BLOCK_SIZE_Y;

const HUD_SIZE_X: usize = 5;
const HUD_SIZE_Y: usize = BOARD_SIZE_Y;

pub const BORDER_SIZE_X: usize = BLOCK_SIZE_X;
pub const BORDER_SIZE_Y: usize = BLOCK_SIZE_Y;

pub const GAME_SIZE_X: usize = BOARD_SIZE_X + HUD_SIZE_X + BORDER_SIZE_X * 3;
pub const GAME_SIZE_Y: usize = BOARD_SIZE_Y + BORDER_SIZE_Y * 2;

const level_timer: u128 = 10000;

// pub type Screen<'a> = [[&'a str; GAME_SIZE_X]; GAME_SIZE_Y];
pub type Screen = Vec<Vec<String>>;

pub struct Tetris { 
    game: Screen,
    current_tetromino: ActiveTetromino,
    // board:[[Tetromino;BOARD_SIZE_X];BOARD_SIZE_Y],
    // score: u16,
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            game: vec![vec![String::from(" "); GAME_SIZE_X]; GAME_SIZE_Y],
            current_tetromino: ActiveTetromino::new(),
            // game: [""; GAME_SIZE_X]
        }
    }

    pub fn play(&mut self) {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        // let mut stdout = stdout().into_raw_mode().unwrap();
        let mut stdin = async_stdin().bytes();

        let mut time_at_last_frame = SystemTime::now();

        let mut current_level_timer = 0;
        let mut speed_time = 1000;

        self.update();

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

            // update
            // Self::log(format!("{:?}", self.current_tetromino));
            if time_at_last_frame.elapsed().unwrap().as_millis() > speed_time {
                time_at_last_frame = SystemTime::now();
                if !self.update() {
                    break;
                }
            }

            // print
            self.print_border();
            self.print(pos_x, pos_y);

            // key listener
            let c = stdin.next();
            match c {
                Some(Ok(b'q')) | 
                Some(Ok(3)) => break, // q or Ctrl + c for quit
                Some(Ok(65)) => self.current_tetromino.rotate(&mut self.game), // arrow up
                Some(Ok(66)) => self.current_tetromino.move_down(&mut self.game), // arrow down
                Some(Ok(67)) => self.current_tetromino.move_right(&mut self.game), // arrow right
                Some(Ok(68)) => self.current_tetromino.move_left(&mut self.game), // arrow left
                _ => ()
            }

            thread::sleep(Duration::from_millis(10));
        }

        self.game_over();
    }

    fn update(&mut self) -> bool {
        if self.current_tetromino.finished {
            crate::Tetris::log(format!("pos_x: {}, pos_y: {}", BORDER_SIZE_X, BORDER_SIZE_Y));
            self.current_tetromino = ActiveTetromino::new();
            if !self.current_tetromino.init(&mut self.game) {
                return false; // game over
            }
            Self::log(format!("{:?}", self.current_tetromino));
        } else {
            self.current_tetromino.move_down(&mut self.game);
        }

        return true;
    }

    fn game_over(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();

        write!(stdout, "{}{}{}", 
            cursor::Goto((terminal_width - 10) / 2, terminal_height / 2),
            termion::style::Reset,
            "game over!"
        ).unwrap();

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(2000));
    }

    fn print_border(&mut self) {
        let color = color::Rgb(0, 255, 0);
        // vertical Border
        for i in 0..GAME_SIZE_Y {
            self.game[i][0] = format!("{}{}", color::Fg(color), "█");
            self.game[i][1] = format!("{}{}", color::Fg(color), "█");
            self.game[i][BOARD_SIZE_X + BORDER_SIZE_X] = format!("{}{}", color::Fg(color), "█");
            self.game[i][BOARD_SIZE_X + BORDER_SIZE_X + 1] = format!("{}{}", color::Fg(color), "█");
            self.game[i][GAME_SIZE_X - BORDER_SIZE_X] = format!("{}{}", color::Fg(color), "█");
            self.game[i][GAME_SIZE_X - BORDER_SIZE_X + 1] = format!("{}{}", color::Fg(color), "█");
        }

        // horizontal border
        for i in 0..GAME_SIZE_X {
            self.game[0][i] = format!("{}{}", color::Fg(color), "█");
            self.game[GAME_SIZE_Y - 1][i] = format!("{}{}", color::Fg(color), "█");
        }
    }

    fn print(&mut self, pos_x: usize, pos_y: usize) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        for i in 0..GAME_SIZE_Y {
            write!(stdout, "{}", cursor::Goto(pos_x as u16, (pos_y + i) as u16)).unwrap();
            for j in 0..GAME_SIZE_X {
                write!(stdout, "{}", self.game[i][j]).unwrap();
            }
        }

        stdout.flush().unwrap();
    }
    
    pub fn log(message: String) {
        let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open("debug.log")
                        .unwrap();

        file.write(message.as_bytes()).unwrap();        
        file.write(b"\n").unwrap();        
    }
}


