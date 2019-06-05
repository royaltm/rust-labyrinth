use std::io::{self, Write};
use super::super::direction::Direction::*;
use super::Wall;

impl Wall {
    /// Print a labyrinth to stdout using UNICODE BOX characters.
    pub fn print(&self) {
        let mut stdout = io::stdout();
        let mut writer = |s: &str| { stdout.write_all(s.as_bytes()).unwrap() };
        self.draw(&mut writer);
    }
    /// Draw a labyrinth as a String using UNICODE BOX characters.
    pub fn to_string(&self) -> String {
        let mut out = String::new();
        self.draw(&mut |s: &str| { out.push_str(s) });
        out
    }
    /// Draw a labyrinth using UNICODE BOX characters to a `writer`.
    pub fn draw(&self, writer: &mut dyn FnMut(&str)) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                if x == 0 || self.is_open(x - 1, y, Up) {
                    if y == 0 || self.is_open(x, y - 1, Left) {
                        if self.is_open(x, y, Up) {
                            writer(if self.is_open(x, y, Left) { "  " } else { "╻ " });
                        } else {
                            writer(if self.is_open(x, y, Left) { "╺━" } else { "┏━" });
                        }
                    } else {
                        if self.is_open(x, y, Up) {
                            writer(if self.is_open(x, y, Left) { "╹ " } else { "┃ " });
                        } else {
                            writer(if self.is_open(x, y, Left) { "┗━" } else { "┣━" });
                        }
                    }
                } else {
                    if y == 0 || self.is_open(x, y - 1, Left) {
                        if self.is_open(x, y, Up) {
                            writer(if self.is_open(x, y, Left) { "╸ " } else { "┓ " });
                        } else {
                            writer(if self.is_open(x, y, Left) { "━━" } else { "┳━" });
                        }
                    } else {
                        if self.is_open(x, y, Up) {
                            writer(if self.is_open(x, y, Left) { "┛ " } else { "┫ " });
                        } else {
                            writer(if self.is_open(x, y, Left) { "┻━" } else { "╋━" });
                        }
                    }
                }
            }
            let x = self.cols;
            if self.is_open(x - 1, y, Up) {
                if y == 0 || self.is_open(x, y - 1, Left) {
                    writer(if self.is_open(x, y, Left) { " " } else { "╻" });
                } else {
                    writer(if self.is_open(x, y, Left) { "╹" } else { "┃" });
                }
            } else {
                if y == 0 || self.is_open(x, y - 1, Left) {
                    writer(if self.is_open(x, y, Left) { "╸" } else { "┓" });
                } else {
                    writer(if self.is_open(x, y, Left) { "┛" } else { "┫" });
                }
            }
            writer("\n");
        }
        let y = self.rows;
        for x in 0..self.cols {
            if x == 0 || self.is_open(x - 1, y, Up) {
                if self.is_open(x, y - 1, Left) {
                    writer(if self.is_open(x, y, Up) { "  " } else { "╺━" });
                } else {
                    writer(if self.is_open(x, y, Up) { "╹ " } else { "┗━" });
                }
            } else {
                if y == 0 || self.is_open(x, y - 1, Left) {
                    writer(if self.is_open(x, y, Up) { "╸ " } else { "━━" });
                } else {
                    writer(if self.is_open(x, y, Up) { "┛ " } else { "┻━" });
                }
            }
        }
        let x = self.cols;
        if self.is_open(x - 1, y, Up) {
            writer(if self.is_open(x, y - 1, Left) { " " } else { "╹" });
        } else {
            writer(if self.is_open(x, y - 1, Left) { "╸" } else { "┛" });
        }
        writer("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use Direction::*;

    #[test]
    fn should_draw_a_labyrinth() {
        let mut labyrinth = Wall::new(2, 2);
        assert_eq!(
"┏━┳━┓
┣━╋━┫
┗━┻━┛
",      &labyrinth.to_string());
        labyrinth.open(0, 0, Right);
        assert_eq!(
"┏━━━┓
┣━┳━┫
┗━┻━┛
",      &labyrinth.to_string());
        labyrinth.open(1, 0, Down);
        assert_eq!(
"┏━━━┓
┣━┓ ┃
┗━┻━┛
",      &labyrinth.to_string());
        labyrinth.open(1, 1, Left);
        assert_eq!(
"┏━━━┓
┣━╸ ┃
┗━━━┛
",      &labyrinth.to_string());
        labyrinth.open(0, 1, Up);
        assert_eq!(
"┏━━━┓
┃   ┃
┗━━━┛
",      &labyrinth.to_string());
        labyrinth.open(0, 0, Left);
        assert_eq!(
"╺━━━┓
╻   ┃
┗━━━┛
",      &labyrinth.to_string());
        labyrinth.open(0, 0, Up);
        assert_eq!(
"  ╺━┓
╻   ┃
┗━━━┛
",      &labyrinth.to_string());
        labyrinth.open(1, 0, Up);
        assert_eq!(
"    ╻
╻   ┃
┗━━━┛
",      &labyrinth.to_string());
        labyrinth.open(1, 0, Right);
        assert_eq!(
"     
╻   ╻
┗━━━┛
",      &labyrinth.to_string());
        labyrinth.open(0, 1, Left);
        assert_eq!(
"     
    ╻
╺━━━┛
",      &labyrinth.to_string());
        labyrinth.open(0, 1, Down);
        assert_eq!(
"     
    ╻
  ╺━┛
",      &labyrinth.to_string());
        labyrinth.open(1, 1, Down);
        assert_eq!(
"     
    ╻
    ╹
",      &labyrinth.to_string());
        labyrinth.open(1, 1, Right);
        assert_eq!(
"     
     
     
",      &labyrinth.to_string());

    }
}
