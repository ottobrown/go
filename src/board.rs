use crate::flood_fill::*;

/// Represents a location on a [Board]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum Stone {
    Empty,
    Black,
    White,
}
impl std::ops::Not for Stone {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
            Stone::Empty => Stone::Empty,
        }
    }
}

/// The state of a go board at a point in time
pub struct Board {
    stones: Vec<Stone>,
    size: (usize, usize),
}
impl Board {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            stones: vec![Stone::Empty; w * h],
            size: (w, h),
        }
    }

    /// Returns the (width, height) of `Self`.
    pub fn size(&self) -> (usize, usize) {
        self.size
    }

    /// Returns a `&Vec` containing all the `Stone`s in order
    pub fn stones(&self) -> &Vec<Stone> {
        &self.stones
    }

    /// Given coordinates (`x`, `y`), where (0, 0) is the top left,
    /// returns the corresponding index on `self.stones`.
    fn index(&self, x: usize, y: usize) -> usize {
        if x >= self.size.0 {
            panic!["x-coordinate out of bounds"];
        }
        y * self.size.0 + x
    }

    /// Returns the stone at (`x`, `y`), where (0, 0) is the top left.
    pub fn get(&self, x: usize, y: usize) -> Stone {
        let i = self.index(x, y);

        self.stones[i]
    }

    /// Places the stone `s` at (`x`, `y`), where (0, 0) is the top left,
    /// regardless of whether it is a legal move.
    pub fn set(&mut self, x: usize, y: usize, s: Stone) {
        let i = self.index(x, y);

        self.stones[i] = s;
    }

    /// Places the stone if it is a legal move
    pub fn attempt_set(&mut self, x: usize, y: usize, s: Stone) -> bool {
        // prevent playing in a place that is already filled
        if self.get(x, y) != Stone::Empty {
            return false;
        }

        self.kill_neighboring_groups(x, y, s);

        let group = find_group(self, x, y, s);

        // prevent suicidal move
        if group.liberties.is_empty() {
            return false;
        }

        self.set(x, y, s);

        true
    }

    fn kill_group(&mut self, g: Group) {
        for p in g.inside {
            self.set(p.0, p.1, Stone::Empty);
        }
    }

    fn kill_neighboring_groups(&mut self, x: usize, y: usize, s: Stone) {
        if x < self.size.0 - 1 && self.get(x + 1, y) == !s {
            let g = find_group(self, x + 1, y, !s);

            if g.liberties.len() == 1 && g.liberties.contains(&(x, y)) {
                self.kill_group(g);
            }
        }

        if x > 0 && self.get(x - 1, y) == !s {
            let g = find_group(self, x - 1, y, !s);

            if g.liberties.len() == 1 && g.liberties.contains(&(x, y)) {
                self.kill_group(g);
            }
        }

        if y < self.size.1 - 1 && self.get(x, y + 1) == !s {
            let g = find_group(self, x, y + 1, !s);

            if g.liberties.len() == 1 && g.liberties.contains(&(x, y)) {
                self.kill_group(g);
            }
        }

        if y > 0 && self.get(x, y - 1) == !s {
            let g = find_group(self, x, y - 1, !s);

            if g.liberties.len() == 1 && g.liberties.contains(&(x, y)) {
                self.kill_group(g);
            }
        }
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn correct_index() {
        let board = Board::new(9, 9);

        // a + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + b + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + c

        // a
        assert_eq!(board.index(0, 0), 0);

        // b
        assert_eq!(board.index(4, 4), 40);

        // c
        assert_eq!(board.index(8, 8), 80);
    }

    #[test]
    #[should_panic]
    fn x_oob() {
        let board = Board::new(9, 9);

        board.index(19, 5);
    }
}
