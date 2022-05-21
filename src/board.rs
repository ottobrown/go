use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Debug)]
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
        self.h
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.h + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        if self.index(x, y) >= self.stones.len() {
            return None;
        }

        Some(self.stones[self.index(x, y)])
    }

    /// Place a stone, regardless of its legality
    pub fn set(&mut self, s: Stone, x: usize, y: usize) {
        let index = self.index(x, y);
        self.stones[index] = s;
    }

    /// Do move only if it is legal.
    /// Returns if move is legal
    pub fn play(&mut self, s: Stone, x: usize, y: usize) -> bool {
        let index = self.index(x, y);
        if index >= self.stones.len() {
            return false
        }
        if self.stones[index] != Stone::Empty {
            return false
        }

        self.stones[index] = s;
        self.add_group(x, y);

        return true
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Neighbors {
        let c = self.neighbor_coords(x, y);

        fn get(b: &Board, c: Option<(usize, usize)>) -> Option<Stone> {
            if let Some(p) = c {
                return b.get(p.0, p.1)
            } else {
                return None
            }
        }

        Neighbors {
            up: get(self, c.up),
            down: get(self, c.down),
            left: get(self, c.left),
            right: get(self, c.right),
        }
    }

    pub fn neighbor_coords(&self, x: usize, y: usize) -> NeighborCoords {
        let mut coords = NeighborCoords::default();
        if x == 0 {
            coords.left = None;
        } else {
            coords.left = Some((x-1, y));
        }

        if x > self.stones.len() {
            coords.right = None;
        } else {
            coords.right = Some((x+1, y));
        }

        if y == 0 {
            coords.up = None;
        } else {
            coords.up = Some((x, y-1));
        }

        if y > self.stones.len() {
            coords.down = None;
        } else {
            coords.down = Some((x, y+1));
        }

        return coords;
    }

    pub fn add_group(&mut self, x: usize, y: usize) {
        let color = match self.get(x, y) {
            Some(c) => c,
            None => {return},
        };
        let mut group = Group {
            color: color,
            points: HashSet::new(),
            liberties: HashSet::new(),
            enemy_neighbors: HashSet::new(),
        };

        fn add_neighbors(board: &Board, group: &mut Group, x: usize, y: usize) {
            let neighbors = board.get_neighbors(x, y);
            let coords = board.neighbor_coords(x, y);

            let stone_array = [neighbors.up, neighbors.down, neighbors.left, neighbors.right];
            let coord_array = [coords.up, coords.down, coords.left, coords.right];
            
            group.points.insert((x, y));
            for i in 0..4 {
                // If stone_array[i] is Some, coord_array[i] is Some
                if let Some(s) = stone_array[i] {
                    let c = coord_array[i].unwrap();

                    if group.categorized(c.0, c.1) {
                        continue;
                    }

                    if s == group.color {
                        add_neighbors(board, group, c.0, c.1);
                    } else if s == Stone::Empty {
                        group.liberties.insert(c);
                    } else {
                        group.enemy_neighbors.insert(c);
                    }
                    // TODO: 'eat' nother group if they border
                }
            }
        }

        add_neighbors(&self, &mut group, x, y);

        self.groups.push(group);
    }
}

#[derive(Clone, Copy, Default)]
pub struct NeighborCoords {
    pub up: Option<(usize, usize)>,
    pub down: Option<(usize, usize)>,
    pub left: Option<(usize, usize)>,
    pub right: Option<(usize, usize)>,
}

#[derive(Clone, Copy, Default)]
pub struct Neighbors {
    pub up: Option<Stone>,
    pub down: Option<Stone>,
    pub left: Option<Stone>,
    pub right: Option<Stone>,
}

#[derive(Clone, Debug)]
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
