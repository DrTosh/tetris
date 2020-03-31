use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Read, Write, stdout};
use std::{thread};
use termion::{async_stdin, color, clear, cursor};
use std::time::{Duration, SystemTime};

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

pub const HEADER_SIZE_X: usize = BLOCK_SIZE_Y * 3;
pub const HEADER_SIZE_Y: usize = BLOCK_SIZE_Y * 4;

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
            current_tetromino: ActiveTetromino::new(BORDER_SIZE_X, BORDER_SIZE_Y, Tetromino::random()),
            // game: [""; GAME_SIZE_X]
        }
    }

    pub fn play(&mut self) {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        //let mut stdout = stdout().into_raw_mode().unwrap();
        let mut stdin = async_stdin().bytes();

        let mut time_at_last_frame = SystemTime::now();
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
            Self::log(format!("{:?}", self.current_tetromino));
            if time_at_last_frame.elapsed().unwrap().as_millis() > speed_time {
                time_at_last_frame = SystemTime::now();
                self.update();
                Self::log(format!("{}", "updated"));
            }

            // print
            self.print_border();
            self.print(pos_x, pos_y);

            // key listener
            let c = stdin.next();
            Self::log(format!("{:?}", c));
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
    }

    fn update(&mut self) {
        if self.current_tetromino.finished {
            self.current_tetromino = ActiveTetromino::new(BORDER_SIZE_X, BOARD_SIZE_Y, Tetromino::random());
            self.current_tetromino.update(
                &mut self.game, 
                BORDER_SIZE_X,
                BORDER_SIZE_Y,
                Rotation::DEGREE_0
            );
        } else {
            self.current_tetromino.move_down(&mut self.game);
        }
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
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        write!(stdout, "{}{}{}", 
            cursor::Goto(1, GAME_SIZE_Y as u16 + 1),
            clear::CurrentLine,
            message
        ).unwrap();
        
        stdout.flush().unwrap();
    }
}


