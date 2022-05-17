#[derive(Clone, Copy, PartialEq)]
pub enum Stone {
    Empty = 0,
    Black = 1,
    White = 2,
}

/// The state of a go game at a single point in time
#[derive(Clone)]
pub struct Board {
    w: usize,
    h: usize,
    stones: Vec<Stone>,
}
impl Board {
    pub fn blank(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            stones: vec![Stone::Empty; w * h],
        }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.w
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.h + x
    }

    pub fn get(&self, x: usize, y: usize) -> Stone {
        self.stones[self.index(x, y)]
    }

    /// Place a stone, regardless of its legality
    pub fn set(&mut self, s: Stone, x: usize, y: usize) {
        let index = self.index(x, y);
        self.stones[index] = s;
    }

    /// Do move only if it is legal
    pub fn play(&mut self, s: Stone, x: usize, y: usize) {
        let index = self.index(x, y);
        if self.stones[index] != Stone::Empty {
            return
        }

        self.stones[index] = s
    }
}
