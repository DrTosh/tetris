use crate::tetris::Screen;

pub trait Printable {
    fn print(&self, screen: &mut Screen, pos_x: usize, pos_y: usize);
}

pub trait Boundingbox {
    fn get_bound() -> (usize, usize);
}

pub trait Collision {
    fn is_collide() -> bool;
}