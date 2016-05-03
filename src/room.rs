use super::vector2d::Vec2D;
use super::Int;

pub type Room = Vec2D<bool>;

pub trait RoomFactory {
    fn new(rows: Int, cols: Int) -> Room;
}

pub trait RoomMethods {
    fn mark(&mut self, x: Int, y: Int);
    fn was_marked(&self, x: Int, y: Int) -> bool;
}

impl RoomFactory for Room {
    fn new(rows: Int, cols: Int) -> Room {
        Vec2D::new(cols as usize, rows as usize, false)
    }
}

impl RoomMethods for Room {
    fn mark(&mut self, x: Int, y: Int) {
        self.set(x as usize, y as usize, true);
    }
    fn was_marked(&self, x: Int, y: Int) -> bool { *self.get(x as usize, y as usize) }
}
