use super::Int;

pub use self::Direction::*;

pub enum Direction { Up, Right, Down, Left }

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub dx: Int,
    pub dy: Int
}

impl Move {
    fn up() -> Move { Move{dx:    0, dy:   -1} }
    fn rt() -> Move { Move{dx:    1, dy:    0} }
    fn dn() -> Move { Move{dx:    0, dy:    1} }
    fn lt() -> Move { Move{dx:   -1, dy:    0} }
    pub fn from_dir(dir: Direction) -> Move {
        match dir {
            Up      => { Move::up() }
            Right   => { Move::rt() }
            Down    => { Move::dn() }
            Left    => { Move::lt() }
        }
    }
    pub fn to_dir(&self) -> Direction {
        match (self.dx, self.dy) {
            ( 0,  1) => { Down }
            (-1,  0) => { Left }
            ( 1,  0) => { Right }
            ( 0, -1) => { Up }
            _ => {
                panic!("invalid Move value: {:?}", self);
            }
        }
    }
    pub fn back(&self)    -> Move { Move{dx: -self.dx, dy: -self.dy} }
    pub fn turn_rt(&self) -> Move { Move{dx: -self.dy, dy:  self.dx} }
    pub fn turn_lt(&self) -> Move { Move{dx:  self.dy, dy: -self.dx} }
    pub fn move_xy(&self, x: &mut Int, y: &mut Int) {
        *x += self.dx;
        *y += self.dy;
    }
}
