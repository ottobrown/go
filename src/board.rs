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
