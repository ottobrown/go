use std::collections::HashSet;

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

    groups: Vec<Group>,
}
impl Board {
    pub fn blank(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            stones: vec![Stone::Empty; w * h],
            groups: Vec::new(),
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

    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        if self.index(x, y) > self.stones.len() {
            return None;
        }

        Some(self.stones[self.index(x, y)])
    }

    /// Place a stone, regardless of its legality
    pub fn set(&mut self, s: Stone, x: usize, y: usize) {
        let index = self.index(x, y);
        self.stones[index] = s;
    }
    
    pub fn get_neighbors(&self, x: usize, y: usize) -> Neighbors {
        let mut neighbors = Neighbors::default();

        if x == 0 {
            neighbors.left = None;
        } else {
            neighbors.left = self.get(x-1, y);
        }

        if y == 0  {
            neighbors.up = None;
        } else {
            neighbors.up = self.get(x, y-1);
        }

        neighbors.right = self.get(x+1, y);
        neighbors.down = self.get(x, y+1);

        return neighbors
    }

}

#[derive(Clone, Copy, Default)]
pub struct Neighbors {
    pub up: Option<Stone>,
    pub down: Option<Stone>,
    pub left: Option<Stone>,
    pub right: Option<Stone>,
}

#[derive(Clone)]
pub struct Group {
    pub color: Stone,
    pub points: HashSet<(usize, usize)>,
    pub liberties: HashSet<(usize, usize)>,
    pub enemy_neighbors: HashSet<(usize, usize)>,
}
impl Group {
    /// Merge this group with another
    pub fn eat(&mut self, other: Group) {
        self.points.extend(other.points);
        self.liberties.extend(other.liberties);
        self.enemy_neighbors.extend(other.enemy_neighbors);
    }

    pub fn categorized(&self, x: usize, y: usize) -> bool {
        self.points.contains(&(x, y)) ||
        self.liberties.contains(&(x, y)) ||
        self.enemy_neighbors.contains(&(x, y))
    }
}
