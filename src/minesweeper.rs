use crate::random::random_range;
use std::{collections::HashSet, fmt::Display};

type Position = (usize, usize);

enum OpenResult {
    Mine,
    NoMine(u8), // 周りのmineの数を表示する
}

struct Minesweeper {
    width: usize,
    height: usize,
    mines: HashSet<Position>,
    open_fields: HashSet<Position>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = (x, y);
                if self.open_fields.contains(&pos) {
                    let mine_count = self.neighboring_mines(pos);
                    write!(f, " {} ", mine_count)?;
                } else if self.mines.contains(&pos) {
                    f.write_str("💣 ")?;
                } else {
                    f.write_str("⬜ ")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Minesweeper {
    fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper {
        Minesweeper {
            width,
            height,
            mines: {
                let mut mines = HashSet::new();
                while mines.len() < mine_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            open_fields: HashSet::new(),
        }
    }

    fn iter_neighbors(&self, (x, y): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j))) // 周辺のインデックスを生成
            .filter(move |&pos| pos != (x, y))
    }

    fn neighboring_mines(&self, pos: Position) -> u8 {
        self.iter_neighbors(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    fn open(&mut self, pos: Position) -> OpenResult {
        self.open_fields.insert(pos); // 開封履歴にいれる

        let is_mine = self.mines.contains(&pos);
        if is_mine {
            OpenResult::Mine
        } else {
            let mine_count = self.neighboring_mines(pos);
            OpenResult::NoMine(mine_count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(10, 10, 5);
        ms.open((5, 5));
        println!("{}", ms);
    }
}
