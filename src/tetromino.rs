use crate::traits::{Printable};
use crate::tetris::{ Screen, BLOCK_SIZE_X, BLOCK_SIZE_Y };
use crate::rotation::Rotation;
use termion::color;

pub enum Tetromino {
    NONE,
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Tetromino {
    pub fn print(&self, screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        match self {
            Tetromino::I => Self::print_i(screen, pos_x, pos_y, rotation),
            Tetromino::J => Self::print_j(screen, pos_x, pos_y, rotation),
            Tetromino::L => Self::print_l(screen, pos_x, pos_y, rotation),
            Tetromino::O => Self::print_o(screen, pos_x, pos_y),
            Tetromino::S => Self::print_s(screen, pos_x, pos_y, rotation),
            Tetromino::T => Self::print_t(screen, pos_x, pos_y, rotation),
            Tetromino::Z => Self::print_z(screen, pos_x, pos_y, rotation),
            _ => (),
            // Thing::B => self.print_J(),
          }
    }

    fn print_i(screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        let color = termion::color::Rgb(40, 211, 228);

        match rotation {
            Rotation::DEGREE_0 | 
            Rotation::DEGREE_180 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X"],
                        vec!["X"],
                        vec!["X"],
                        vec!["X"]
                    ]
                );
            }, 
            Rotation::DEGREE_90 |
            Rotation::DEGREE_270 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X", "X", "X"]
                    ]
                );
            }
        }
    }

    fn print_j(screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        let color = termion::color::Rgb(22, 4, 230);
        match rotation {
            Rotation::DEGREE_0 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec![" ", "X"],
                        vec![" ", "X"],
                        vec!["X", "X"]
                    ]
                );
            },
            Rotation::DEGREE_90 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X"],
                        vec!["X", "X", "X"]
                    ]
                );            
            }, 
            Rotation::DEGREE_180 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X"],
                        vec!["X"],
                        vec!["X"]
                    ]
                );
            },
            Rotation::DEGREE_270 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X", "X"],
                        vec![" ", " ", "X"]
                    ]
                );
            }
        }
    }

    fn print_l(screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        let color = termion::color::Rgb(219, 105, 34);
        match rotation {
            Rotation::DEGREE_0 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X"],
                        vec!["X"],
                        vec!["X", "X"]
                    ]
                );
            }
            Rotation::DEGREE_90 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X", "X"],
                        vec!["X"],
                    ]
                );
            }, 
            Rotation::DEGREE_180 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X"],
                        vec![" ", "X"],
                        vec![" ", "X"]
                    ]
                );
            },
            Rotation::DEGREE_270 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec![" ", " ", "X"],
                        vec!["X", "X", "X"]
                    ]
                );
            }
        }
    }

    fn print_o(screen: &mut Screen, pos_x: usize, pos_y: usize) {
        let color = termion::color::Rgb(240, 223, 40);
        Self::print_block_vec(screen, pos_x, pos_y, color, 
            vec![
                vec!["X", "X"],
                vec!["X", "X"]
            ]
        );
    }

    fn print_s(screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        let color = termion::color::Rgb(111, 255, 41);
        match rotation {
            Rotation::DEGREE_0 |
            Rotation::DEGREE_180 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec![" ", "X", "X"],
                        vec!["X", "X"]
                    ]
                );
            },
            Rotation::DEGREE_90 |
            Rotation::DEGREE_270 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", " "],
                        vec!["X", "X"],
                        vec![" ", "X"]
                    ]
                );
            }, 
        }
    }

    fn print_t(screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        let color = termion::color::Rgb(184, 27, 238);
        match rotation {
            Rotation::DEGREE_0 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec![" ", "X", " "],
                        vec!["X", "X", "X"]
                    ]
                );
            }
            Rotation::DEGREE_90 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X"],
                        vec!["X", "X"],
                        vec!["X"]
                    ]
                );
            }, 
            Rotation::DEGREE_180 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X", "X"],
                        vec![" ", "X", " "]
                    ]
                );
            },
            Rotation::DEGREE_270 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec![" ", "X"],
                        vec!["X", "X"],
                        vec![" ", "X"]
                    ]
                );
            }
        }
    }

    fn print_z(screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) {
        let color = termion::color::Rgb(222, 33, 61);
        match rotation {
            Rotation::DEGREE_0 |
            Rotation::DEGREE_180 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec!["X", "X"],
                        vec![" ", "X", "X"]
                    ]
                );
            }
            Rotation::DEGREE_90 |
            Rotation::DEGREE_270 => {
                Self::print_block_vec(screen, pos_x, pos_y, color, 
                    vec![
                        vec![" ", "X"],
                        vec!["X", "X"],
                        vec!["X", " "]
                    ]
                );
            }, 
        }
    }

    pub fn print_block_vec(screen: &mut Screen, pos_x: usize, pos_y: usize, color: termion::color::Rgb, blocks: Vec<Vec<&str>>) {
        for i in 0..blocks.len() {
            for j in 0..blocks[i].len() {
                if blocks[i][j] == "X" {
                    Self::print_block(screen, pos_x + j * BLOCK_SIZE_X, pos_y + i * BLOCK_SIZE_Y, color);
                }
            }
        }
    }

    pub fn print_block(screen: &mut Screen, pos_x: usize, pos_y: usize, color: termion::color::Rgb) {
        screen[pos_y as usize][pos_x as usize] = format!("{}{}", color::Fg(color), "█"); 
        screen[pos_y as usize][(pos_x + 1) as usize] = format!("{}{}", color::Fg(color), "█");
    }
}