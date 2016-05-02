use super::vector2d::Vec2D;
use super::Int;

pub struct Room {
    rooms: Vec2D<bool>
}

impl Room {
    pub fn new(rows: Int, cols: Int) -> Room {
        Room { rooms: Vec2D::new(cols as usize, rows as usize, false) }
    }
    pub fn mark(&mut self, x: Int, y: Int) -> &mut Room {
        self.rooms.set(x as usize, y as usize, true);
        self
    }
    pub fn was_marked(&self, x: Int, y: Int) -> bool { *self.rooms.get(x as usize, y as usize) }
}
