use super::Int;
use super::vector2d::Vec2D;
use super::direction::Direction;
use super::direction::Direction::*;

mod carve;
mod print;

/// # Wall organizes a labyrinth. 
pub struct Wall {
    /// Number of rooms in a row.
    pub cols: Int,
    /// Number of rooms in a column.
    pub rows: Int,
    exits: Vec2D<bool>
}

impl Wall {
    fn exit_at(&self, x: Int, y: Int, dir: Direction) -> (usize, usize) {
        let (dx, dy): (usize, usize) = match dir {
            Up    => { (1, 0) }
            Right => { (2, 0) }
            Down  => { (1, 1) }
            Left  => { (0, 0) }
        };
        (x as usize * 2 + dx, y as usize + dy)
    }
}

impl Wall {
    /// Create a new labyrinth with all exits being closed.
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth::Wall;
    ///
    /// let mut labyrinth = Wall::new(5, 2);
    /// assert_eq!(5, labyrinth.cols);
    /// assert_eq!(2, labyrinth.rows);
    /// ```
    pub fn new(cols: Int, rows: Int) -> Wall {
        Wall { cols: cols,
               rows: rows,
              exits: Vec2D::new(cols as usize * 2 + 1, rows as usize + 1, false) }
    }
    /// Close all of the labyrinth exits bringing it to the initial state.
    /// This is required to reuse the instance for another carving.
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth::Wall;
    ///
    /// let mut labyrinth = Wall::new(10, 10);
    /// labyrinth.carve();
    /// labyrinth.close_all().carve();
    /// ```
    pub fn close_all(&mut self) -> &mut Wall {
        self.exits.fill(true);
        self
    }
    /// Check existence of the specified exit.
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth::{Wall, Direction};
    ///
    /// let labyrinth = Wall::new(10, 10);
    /// assert_eq!(false, labyrinth.is_open(0, 0, Direction::Right));
    /// ```
    pub fn is_open(&self, x: Int, y: Int, dir: Direction) -> bool {
        let (x, y) = self.exit_at(x, y, dir);
        *self.exits.get(x, y)
    }
    /// Set existence of the specified exit.
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth::{Wall, Direction};
    ///
    /// let mut labyrinth = Wall::new(10, 10);
    /// assert_eq!(false, labyrinth.is_open(4, 5, Direction::Up));
    /// labyrinth.set(4, 5, Direction::Up, true);
    /// assert_eq!(true, labyrinth.is_open(4, 5, Direction::Up));
    /// ```
    pub fn set(&mut self, x: Int, y: Int, dir: Direction, val: bool) -> &mut Wall {
        let (x, y) = self.exit_at(x, y, dir);
        self.exits.set(x, y, val);
        self
    }
    /// Open specified exit.
    ///
    /// # Examples
    ///
    /// ```
    /// use labyrinth::{Wall, Direction};
    ///
    /// let mut labyrinth = Wall::new(10, 10);
    /// labyrinth.open(2, 3, Direction::Down);
    /// assert_eq!(true, labyrinth.is_open(2, 3, Direction::Down));
    /// labyrinth.close(2, 3, Direction::Down);
    /// assert_eq!(false, labyrinth.is_open(2, 3, Direction::Down));
    /// ```
    pub fn open(&mut self, x: Int, y: Int, dir: Direction) -> &mut Wall {
        self.set(x, y, dir, true)
    }
    /// Close specified exit.
    pub fn close(&mut self, x: Int, y: Int, dir: Direction) -> &mut Wall {
        self.set(x, y, dir, false)
    }
    /// Check if x and y coordinates are valid in a labyrinth.
    /// # Examples
    ///
    /// ```
    /// use labyrinth::Wall;
    ///
    /// let labyrinth = Wall::new(3, 2);
    /// assert_eq!(true, labyrinth.in_bounds(2, 1));
    /// assert_eq!(false, labyrinth.in_bounds(4, 5));
    /// assert_eq!(false, labyrinth.in_bounds(-1, 0));
    /// assert_eq!(false, labyrinth.in_bounds(-4, -5));
    /// assert_eq!(false, labyrinth.in_bounds(1, -1));
    /// ```
    pub fn in_bounds(&self, x: Int, y: Int) -> bool {
        x >= 0 && y >= 0 && x < self.cols && y < self.rows
    }

    /* Dynamic dispatch */
    // fn random_start(&self, rnd: &mut FnMut(Int) -> Int) -> (Int, Int) {
    //     (rnd(self.cols), rnd(self.rows))
    // }

    /* Static dispatch */
    /// Randomize coordinates.
    ///
    /// The `rnd(n: Int) -> Int` should return a random number between `0` and `n` excluding `n`.
    pub fn random_start<F>(&self, mut rnd: F) -> (Int, Int) where F : FnMut(Int) -> Int {
        (rnd(self.cols), rnd(self.rows))
    }
}
