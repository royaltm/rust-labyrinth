use rand::{thread_rng, Rng};
use super::Wall;
use super::super::room::Room;
use super::super::Direction::*;
use super::super::direction::{Move as Mov};

// fn gen_rnd() -> Box<FnMut(Int) -> Int> {
//     let mut rng = thread_rng();
//     Box::new(move |x| rng.gen_range(0, x))
// }
// let mut rnd = gen_rnd(); wall.random_start(&mut *rnd)

impl Wall {
    /// Carve a fresh labyrinth using hunt and seek algorithm.
    ///
    /// The labyrinth must be in an initial state otherwise this method will never return.
    pub fn carve(&mut self) {
        let mut rooms = Room::new(self.rows, self.cols);
        let mut rnd = {
            let mut rng = thread_rng();
            move |x| rng.gen_range(0, x)
        };
        let mut mov = Mov::from_dir(Up);
        let (mut x, mut y) = self.random_start(&mut rnd);

        rooms.mark(x, y);

        /* hunt */
        for _ in 1..(self.rows * self.cols) {
            match rnd(3) {
                1 => { mov.turn_rt(); }
                2 => { mov.turn_lt(); }
                _ => { }
            }
            {
                let outbound_or_marked = |x, y, mov: Mov| {
                    let (x, y) = (x + mov.dx, y + mov.dy);
                    !self.in_bounds(x, y) || rooms.was_marked(x, y)
                };

                'check: while outbound_or_marked(x, y, mov) {
                    for _ in 0..3 {
                        mov.turn_rt();
                        if !outbound_or_marked(x, y, mov) { break 'check; }
                    }
                    /* seek */
                    loop {
                        x += 1;
                        y += 1;
                        if !self.in_bounds(x, y) {
                            let (a, b) = self.random_start(&mut rnd);
                            x = a;
                            y = b;
                        }
                        if rooms.was_marked(x, y) { continue 'check; }
                    }
                }
            }
            self.open(x, y, mov.to_dir());
            mov.move_xy(&mut x, &mut y);
            rooms.mark(x, y);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::super::*;
    use Direction::*;

    fn count_open(labyrinth: &Wall) -> usize {
        let mut opened = 0usize;
        for y in 0..labyrinth.rows {
            for x in 0..labyrinth.cols {
                if labyrinth.is_open(x, y, Up)    { opened+= 1 };
                if labyrinth.is_open(x, y, Right) { opened+= 1 };
                if labyrinth.is_open(x, y, Left)  { opened+= 1 };
                if labyrinth.is_open(x, y, Down)  { opened+= 1 };
            }
        }
        opened
    }
    #[test]
    fn should_carve_a_labyrinth() {
        let mut labyrinth = Wall::new(137, 111);
        assert_eq!(0, count_open(&labyrinth));
        labyrinth.carve();
        assert_eq!(137*111*2-2, count_open(&labyrinth));
    }
}
