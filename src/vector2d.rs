pub struct Vec2D<T> {
    cols: usize,
    rows: Vec<Vec<T>>
}

impl<T: Clone> Vec2D<T> {
    /// Create two-dimensional vector containing `rows` vectors of `cols` size each.
    pub fn new(cols: usize, rows: usize, value: T) -> Vec2D<T> {
        Vec2D {
            cols: cols,
            rows: vec![vec![value; cols]; rows]
        }
    }
    pub fn fill(&mut self, value: T) {
        for row in self.rows.iter_mut() {
            for item in row.iter_mut() {
                *item = value.clone();
            }
        }
    }
    pub fn num_rows(&self) -> usize { self.rows.len() }
    pub fn num_cols(&self) -> usize { self.cols }
    pub fn set(&mut self, col: usize, row: usize, value: T) {
        self.rows[row][col] = value;
    }
    pub fn get(&self, col: usize, row: usize) -> &T {
        &self.rows[row][col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_empty_vector() {
        let v2d = Vec2D::new(0, 0, 0u32);
        assert_eq!(0, v2d.num_cols());
        assert_eq!(0, v2d.num_rows());
    }
    #[test]
    fn should_create_vector_with_value() {
        let mut v2d = Vec2D::new(3, 2, 'x');
        assert_eq!(3, v2d.num_cols());
        assert_eq!(2, v2d.num_rows());
        for row in 0..2 {
            for col in 0..3 {
                assert_eq!('x', *v2d.get(col, row));
            }
        }
        v2d.set(1, 0, 'a');
        assert_eq!('a', *v2d.get(1, 0));
        assert_eq!('x', *v2d.get(0, 1));
    }
    #[test]
    fn should_fill_vector_with_value() {
        let mut v2d = Vec2D::new(4, 8, '@');
        assert_eq!(4, v2d.num_cols());
        assert_eq!(8, v2d.num_rows());
        for row in 0..8 {
            for col in 0..4 {
                assert_eq!('@', *v2d.get(col, row));
            }
        }
        v2d.fill('!');
        for row in 0..8 {
            for col in 0..4 {
                assert_eq!('!', *v2d.get(col, row));
            }
        }
    }
}
