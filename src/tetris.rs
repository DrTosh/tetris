use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Read, Write, stdout};
use std::{thread};
use termion::{async_stdin, color, clear, cursor, style};
use std::time::{Duration, SystemTime};
use std::fs::{OpenOptions};

use crate::active_tetromino::ActiveTetromino;

pub const BLOCK_SIZE_X: usize = 2;
pub const BLOCK_SIZE_Y: usize = 1;

const BOARD_SIZE_X: usize = 10 * BLOCK_SIZE_X;
const BOARD_SIZE_Y: usize = 20 * BLOCK_SIZE_Y;

pub const BORDER_SIZE_X: usize = BLOCK_SIZE_X;
pub const BORDER_SIZE_Y: usize = BLOCK_SIZE_Y;

pub const GAME_SIZE_X: usize = BOARD_SIZE_X + BORDER_SIZE_X * 3;
pub const GAME_SIZE_Y: usize = BOARD_SIZE_Y + BORDER_SIZE_Y * 2;

pub type Screen = Vec<Vec<String>>;

pub struct Tetris { 
    game: Screen,
    current_tetromino: ActiveTetromino,
    changed: bool
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {
            game: vec![vec![String::from(" "); GAME_SIZE_X]; GAME_SIZE_Y],
            current_tetromino: ActiveTetromino::new(),
            changed: true
        }
    }

    pub fn play(&mut self) {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
        // let mut stdout = stdout().into_raw_mode().unwrap();
        
        let mut stdin = async_stdin().bytes();
        let mut time_at_last_frame = SystemTime::now();

        self.update();

        loop {
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
            if time_at_last_frame.elapsed().unwrap().as_millis() > 500 {
                time_at_last_frame = SystemTime::now();
                if !self.update() {
                    break;
                }
            }

            // print
            if self.changed {
                write!(stdout, "{}{}", clear::All, termion::cursor::Hide).unwrap();
                self.print_border();
                self.print(pos_x, pos_y);
                self.changed = false;
            }            

            // key listener
            let c = stdin.next();
            match c {
                Some(Ok(b'q')) | 
                Some(Ok(3)) => break, // q or Ctrl + c for quit
                Some(Ok(65)) => { self.current_tetromino.rotate(&mut self.game); self.changed = true }, // arrow up
                Some(Ok(66)) => { self.current_tetromino.move_down(&mut self.game); self.changed = true }, // arrow down
                Some(Ok(67)) => { self.current_tetromino.move_right(&mut self.game); self.changed = true }, // arrow right
                Some(Ok(68)) => { self.current_tetromino.move_left(&mut self.game); self.changed = true }, // arrow left
                _ => ()
            }

            thread::sleep(Duration::from_millis(10));
        }

        self.game_over();
    }

    fn update(&mut self) -> bool {
        if self.current_tetromino.finished {
            self.update_rows();

            self.current_tetromino = ActiveTetromino::new();
            if !self.current_tetromino.init(&mut self.game) {
                return false; // game over
            }
            Self::log(format!("{:?}", self.current_tetromino));
        } else {
            self.current_tetromino.move_down(&mut self.game);
            self.changed = true;
        }

        return true;
    }

    fn update_rows(&mut self) {
        let mut completed_rows: Vec<usize> = vec![];

        for i in BORDER_SIZE_Y..GAME_SIZE_Y - BORDER_SIZE_Y {
            let mut complete_row = true;
            for j in BORDER_SIZE_X..BOARD_SIZE_X {
                if self.game[i][j] == String::from(" ") {
                    complete_row = false;
                    break;
                }
            }
            
            if complete_row {
                completed_rows.push(i);
            }
        }

        for i in 0..completed_rows.len() {
            self.game.remove(completed_rows[i]);
            self.game.insert(1, vec![String::from(" "); GAME_SIZE_X]);
        }
    }

    fn game_over(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();

        write!(stdout, "{}{}{}{}", 
            clear::All,
            cursor::Goto((terminal_width - 10) / 2, terminal_height / 2),
            style::Reset,
            "game over!"
        ).unwrap();

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(2000));
    }

    fn print_border(&mut self) {
        let color = color::Rgb(0, 102, 102);

        // vertical Border
        for i in 0..GAME_SIZE_Y {
            self.game[i][0] = format!("{}{}", color::Fg(color), "█");
            self.game[i][1] = format!("{}{}", color::Fg(color), "█");
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


