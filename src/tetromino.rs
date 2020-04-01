extern crate rand;

use crate::tetris::{Screen, BLOCK_SIZE_X, BLOCK_SIZE_Y};
use crate::rotation::Rotation;
use termion::color;
use rand::Rng;

#[derive(Debug)]
pub enum Tetromino {
    I,
    J,
    L,
    O,
    S,
    T,
    Z
}

impl Tetromino {
    pub fn random() -> Tetromino {
        match rand::thread_rng().gen_range(0, 7) {
            0 => return Self::I,
            1 => return Self::J,
            2 => return Self::L,
            3 => return Self::O,
            4 => return Self::S,
            5 => return Self::T,
            _ => return Self::Z,
        }
    }

    pub fn update(&self, screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation, erase: bool) {
        Self::update_block_vec(
            screen, 
            pos_x, 
            pos_y, 
            Self::get_color(self),
            Self::get_pattern(self, rotation),
            erase
        );
    }

    pub fn collide(&self, screen: &mut Screen, pos_x: usize, pos_y: usize, rotation: Rotation) -> bool {
        let pattern = self.get_pattern(rotation);

        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                if pattern[i][j] == "X" {
                    let mut collide = false;

                    collide |= screen[pos_y + i * BLOCK_SIZE_Y][pos_x + j * BLOCK_SIZE_X] != String::from(" ");
                    collide |= screen[pos_y + i * BLOCK_SIZE_Y][pos_x + j * BLOCK_SIZE_X + 1] != String::from(" ");

                    if collide {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    pub fn get_pattern<'a>(&self, rotation: Rotation) -> Vec<Vec<&'a str>> {
        match self {
            Tetromino::I => return Self::pattern_i(rotation),
            Tetromino::J => return Self::pattern_j(rotation),
            Tetromino::L => return Self::pattern_l(rotation),
            Tetromino::O => return Self::pattern_o(),
            Tetromino::S => return Self::pattern_s(rotation),
            Tetromino::T => return Self::pattern_t(rotation),
            Tetromino::Z => return Self::pattern_z(rotation),
        }
    }

    pub fn get_color(&self) -> color::Rgb {
        match self {
            Tetromino::I => return color::Rgb(102, 255, 255),
            Tetromino::J => return color::Rgb(0, 153, 255),
            Tetromino::L => return color::Rgb(255, 153, 51),
            Tetromino::O => return color::Rgb(255, 255, 102),
            Tetromino::S => return color::Rgb(102, 255, 102),
            Tetromino::T => return color::Rgb(184, 27, 238),
            Tetromino::Z => return color::Rgb(204, 51, 153)
        }
    }

    fn pattern_i<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Degree0 | 
            Rotation::Degree180 => {
                return 
                    vec![
                        vec!["X"],
                        vec!["X"],
                        vec!["X"],
                        vec!["X"]
                    ]
                ;
            }, 
            Rotation::Degree90 |
            Rotation::Degree270 => {
                return 
                    vec![
                        vec!["X", "X", "X", "X"]
                    ]
                ;
            }
        }
    }

    fn pattern_j<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Degree0 => {
                return
                    vec![
                        vec![" ", "X"],
                        vec![" ", "X"],
                        vec!["X", "X"]
                    ]
                ;
            },
            Rotation::Degree90 => {
                return
                    vec![
                        vec!["X"],
                        vec!["X", "X", "X"]
                    ]
                ;
            }, 
            Rotation::Degree180 => {
                return
                    vec![
                        vec!["X", "X"],
                        vec!["X"],
                        vec!["X"]
                    ]
                ;
            },
            Rotation::Degree270 => {
                return
                    vec![
                        vec!["X", "X", "X"],
                        vec![" ", " ", "X"]
                    ]
                ;
            }
        }
    }

    fn pattern_l<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Degree0 => {
                return
                    vec![
                        vec!["X"],
                        vec!["X"],
                        vec!["X", "X"]
                    ]
                ;
            }
            Rotation::Degree90 => {
                return
                    vec![
                        vec!["X", "X", "X"],
                        vec!["X"],
                    ]
                ;
            }, 
            Rotation::Degree180 => {
                return
                    vec![
                        vec!["X", "X"],
                        vec![" ", "X"],
                        vec![" ", "X"]
                    ]
                ;
            },
            Rotation::Degree270 => {
                return
                    vec![
                        vec![" ", " ", "X"],
                        vec!["X", "X", "X"]
                    ]
                ;
            }
        }
    }

    fn pattern_o<'a>() -> Vec<Vec<&'a str>> {
        return
            vec![
                vec!["X", "X"],
                vec!["X", "X"]
            ]
        ;
    }

    fn pattern_s<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Degree0 |
            Rotation::Degree180 => {
                return
                    vec![
                        vec![" ", "X", "X"],
                        vec!["X", "X"]
                    ]
                ;
            },
            Rotation::Degree90 |
            Rotation::Degree270 => {
                return
                    vec![
                        vec!["X", " "],
                        vec!["X", "X"],
                        vec![" ", "X"]
                    ]
                ;
            }, 
        }
    }

    fn pattern_t<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Degree0 => {
                return
                    vec![
                        vec![" ", "X", " "],
                        vec!["X", "X", "X"]
                    ]
                ;
            }
            Rotation::Degree90 => {
                return
                    vec![
                        vec!["X"],
                        vec!["X", "X"],
                        vec!["X"]
                    ]
                ;
            }, 
            Rotation::Degree180 => {
                return
                    vec![
                        vec!["X", "X", "X"],
                        vec![" ", "X", " "]
                    ]
                ;
            },
            Rotation::Degree270 => {
                return
                    vec![
                        vec![" ", "X"],
                        vec!["X", "X"],
                        vec![" ", "X"]
                    ]
                ;
            }
        }
    }

    fn pattern_z<'a>(rotation: Rotation) -> Vec<Vec<&'a str>> {
        match rotation {
            Rotation::Degree0 |
            Rotation::Degree180 => {
                return
                    vec![
                        vec!["X", "X"],
                        vec![" ", "X", "X"]
                    ]
                ;
            }
            Rotation::Degree90 |
            Rotation::Degree270 => {
                return
                    vec![
                        vec![" ", "X"],
                        vec!["X", "X"],
                        vec!["X", " "]
                    ]
                ;
            }, 
        }
    }

    pub fn update_block_vec(screen: &mut Screen, pos_x: usize, pos_y: usize, color: termion::color::Rgb, blocks: Vec<Vec<&str>>, erase: bool) {
        for i in 0..blocks.len() {
            for j in 0..blocks[i].len() {
                if blocks[i][j] == "X" {
                    Self::update_block(screen, pos_x + j * BLOCK_SIZE_X, pos_y + i * BLOCK_SIZE_Y, color, erase);
                }
            }
        }
    }

    pub fn update_block(screen: &mut Screen, pos_x: usize, pos_y: usize, color: termion::color::Rgb, erase: bool) {
        if erase {
            screen[pos_y][pos_x] = String::from(" "); 
            screen[pos_y][pos_x + 1] = String::from(" ");
        } else {
            screen[pos_y][pos_x] = format!("{}{}", color::Fg(color), "█"); 
            screen[pos_y][pos_x + 1] = format!("{}{}", color::Fg(color), "█");
        }        
    }
}