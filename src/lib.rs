use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use random::random_range;

mod random;

pub type Position = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let pos = (col, row);

                if !self.open_fields.contains(&pos) {
                    if self.flagged_fields.contains(&pos) {
                        f.write_str("🏁 ");
                    } else {
                        f.write_str("⬛️ ")?;
                    }
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    write!(f, " {} ", self.neighboring_mines(pos))?;
                }
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_amount: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mine_amount {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }

                mines
            },
            flagged_fields: HashSet::new(),
        }
    }

    pub fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        if self.flagged_fields.contains(&position) {
            return None;
        }

        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            Some(OpenResult::Mine)
        } else {
            Some(OpenResult::NoMine(0))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position) {
        if self.open_fields.contains(&pos) {
            return;
        }

        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 9);

        ms.open((5, 5));
        ms.toggle_flag((6, 6));
        ms.open((6, 6));

        println!("{}", ms);
    }
}
