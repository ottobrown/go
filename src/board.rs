use crate::game::Marker;
use crate::Rules;

use fxhash::hash64;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum Stone {
    Empty = 0,
    Black = 1,
    White = 2,
}
impl Stone {
    pub fn swap(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::Empty => Self::Empty,
        }
    }
}

/// The state of a go game at a single point in time
#[derive(Clone)]
pub struct Board {
    w: usize,
    h: usize,
    stones: Vec<Stone>,
    marks: Vec<Marker>,

    groups: Vec<Group>,

    /// The number of black stones that have been captured by white.
    pub black_prisoners: u32,
    /// The number of white stones that have been captured by black.
    pub white_prisoners: u32,

    /// hash64(self.stones) after every move
    past_hashes: Vec<u64>,
}
impl Board {
    pub fn blank(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            stones: vec![Stone::Empty; w * h],
            marks: vec![Marker::Empty; w * h],
            groups: Vec::new(),
            black_prisoners: 0,
            white_prisoners: 0,
            past_hashes: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.w
    }

    pub fn height(&self) -> usize {
        self.h
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        match self.stones.get(self.index(x, y)) {
            Some(s) => Some(*s),
            None => None,
        }
    }

    /// Place a stone, regardless of its legality
    pub fn set(&mut self, s: Stone, x: usize, y: usize) {
        let index = self.index(x, y);
        self.stones[index] = s;
    }

    pub fn get_marker(&self, x: usize, y: usize) -> Option<Marker> {
        match self.marks.get(self.index(x, y)) {
            Some(m) => Some(*m),
            None => None,
        }
    }

    /// Remove all markers
    pub fn clear_markers(&mut self) {
        for i in &mut self.marks {
            *i = Marker::Empty;
        }
    }

    pub fn set_marker(&mut self, m: Marker, x: usize, y: usize) {
        let index = self.index(x, y);
        self.marks[index] = m;
    }

    /// Do move only if it is legal.
    /// Returns if move is legal
    pub fn play(&mut self, s: Stone, x: usize, y: usize, rules: &Rules) -> bool {
        let mut new = self.clone();

        // If coordinate is off the board
        if x > new.w || y > new.h {
            return false;
        }

        if new.index(x, y) >= new.stones.len() {
            return false;
        }

        // If point is already filled
        if self.get(x, y).unwrap() != Stone::Empty {
            return false;
        }

        // Place stone
        new.set(s, x, y);

        // Find group of newly-placed stone
        let group = new.find_group(x, y).unwrap();
        new.groups.push(group.clone());

        new.update_groups();

        // Kill enemy groups
        for i in (0..new.groups.len()).rev() {
            let g = &new.groups[i];

            if g.liberties.is_empty() && g.color == group.color.swap() {
                new.kill_group(i);
            }
        }

        new.update_groups();

        // If group of last placed stone still has no liberties
        if new.groups.last().unwrap().liberties.is_empty() {
            if rules.suicide_allowed {
                new.kill_group(new.groups.len() - 1);
            } else {
                return false;
            }
        }

        // If board position is immediately repeated
        let hash = hash64(&new.stones);
        if Some(&hash) == new.past_hashes.iter().rev().nth(1) {
            return false;
        }

        // If superko rules are enabled and board position is repeated ever
        if rules.superko && new.past_hashes.contains(&hash) {
            return false;
        }

        new.past_hashes.push(hash);

        *self = new;
        return true;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Neighbors {
        let c = self.neighbor_coords(x, y);

        fn get(b: &Board, c: Option<(usize, usize)>) -> Option<Stone> {
            if let Some(p) = c {
                return b.get(p.0, p.1);
            } else {
                return None;
            }
        }

        Neighbors {
            up: get(self, c.up),
            down: get(self, c.down),
            left: get(self, c.left),
            right: get(self, c.right),
        }
    }

    fn neighbor_coords(&self, x: usize, y: usize) -> NeighborCoords {
        let mut coords = NeighborCoords::default();
        if x == 0 {
            coords.left = None;
        } else {
            coords.left = Some((x - 1, y));
        }

        // ¯\_(ツ)_/¯
        if x >= self.w - 1 {
            coords.right = None;
        } else {
            coords.right = Some((x + 1, y));
        }

        if y == 0 {
            coords.up = None;
        } else {
            coords.up = Some((x, y - 1));
        }

        if y >= self.h {
            coords.down = None;
        } else {
            coords.down = Some((x, y + 1));
        }

        return coords;
    }

    /// Returns the index on self.groups of the group the point is part of
    fn get_group(&self, x: usize, y: usize) -> Option<usize> {
        for i in 0..self.groups.len() {
            if self.groups[i].points.contains(&(x, y)) {
                return Some(i);
            }
        }

        return None;
    }

    fn find_group(&mut self, x: usize, y: usize) -> Option<Group> {
        let color = match self.get(x, y) {
            Some(c) => c,
            None => return None,
        };
        let mut group = Group {
            color: color,
            points: HashSet::new(),
            liberties: HashSet::new(),
            enemy_neighbors: HashSet::new(),
        };

        fn add_neighbors(board: &mut Board, group: &mut Group, x: usize, y: usize) {
            let neighbors = board.get_neighbors(x, y);
            let coords = board.neighbor_coords(x, y);

            let stone_array = [
                neighbors.up,
                neighbors.down,
                neighbors.left,
                neighbors.right,
            ];
            let coord_array = [coords.up, coords.down, coords.left, coords.right];

            group.points.insert((x, y));
            for i in 0..4 {
                // If stone_array[i] is Some, coord_array[i] is Some
                if let Some(s) = stone_array[i] {
                    let c = coord_array[i].unwrap();

                    if group.categorized(c.0, c.1) {
                        continue;
                    }

                    if let Some(g) = board.get_group(c.0, c.1) {
                        if board.groups[g].color == group.color {
                            group.eat(&board.groups[g]);
                            board.groups.remove(g);
                        }
                    }

                    if s == group.color {
                        add_neighbors(board, group, c.0, c.1);
                    } else if s == Stone::Empty {
                        group.liberties.insert(c);
                    } else {
                        group.enemy_neighbors.insert(c);
                    }
                }
            }
        }

        add_neighbors(self, &mut group, x, y);

        return Some(group);
    }

    /// Updates liberties and enemy_neighbors
    fn update_groups(&mut self) {
        for g in 0..self.groups.len() {
            let mut group = self.groups[g].clone();

            let mut liberties = HashSet::new();
            let mut enemy_neighbors = HashSet::new();

            for i in &group.points {
                let neighbors = self.get_neighbors(i.0, i.1);
                let coords = self.neighbor_coords(i.0, i.1);
                let stone_array = [
                    neighbors.up,
                    neighbors.down,
                    neighbors.left,
                    neighbors.right,
                ];
                let coord_array = [coords.up, coords.down, coords.left, coords.right];

                for j in 0..4 {
                    if stone_array[j] == Some(Stone::Empty) {
                        liberties.insert(coord_array[j].unwrap());
                    } else if let Some(s) = stone_array[j] {
                        if s != group.color {
                            enemy_neighbors.insert(coord_array[j].unwrap());
                        }
                    }
                }
            }

            group.liberties = liberties;
            group.enemy_neighbors = enemy_neighbors;

            self.groups[g] = group;
        }
    }

    /// Kill the group at the given index on self.groups.
    fn kill_group(&mut self, i: usize) {
        let mut num_captured = 0_u32;
        let g = self.groups[i].clone();

        for j in &g.points {
            num_captured += 1;
            self.set(Stone::Empty, j.0, j.1);
        }
        self.groups.remove(i);

        match g.color {
            Stone::Black => self.black_prisoners += num_captured,
            Stone::White => self.white_prisoners += num_captured,

            _ => {}
        }
    }
}

#[derive(Clone, Copy, Default)]
struct NeighborCoords {
    pub up: Option<(usize, usize)>,
    pub down: Option<(usize, usize)>,
    pub left: Option<(usize, usize)>,
    pub right: Option<(usize, usize)>,
}

#[derive(Clone, Copy, Default)]
struct Neighbors {
    pub up: Option<Stone>,
    pub down: Option<Stone>,
    pub left: Option<Stone>,
    pub right: Option<Stone>,
}

#[derive(Clone, Debug)]
struct Group {
    pub color: Stone,
    pub points: HashSet<(usize, usize)>,
    pub liberties: HashSet<(usize, usize)>,
    pub enemy_neighbors: HashSet<(usize, usize)>,
}
impl Group {
    /// Merge this group with another
    pub fn eat(&mut self, other: &Group) {
        self.points.extend(other.points.clone());
        self.liberties.extend(other.liberties.clone());
        self.enemy_neighbors.extend(other.enemy_neighbors.clone());
    }

    pub fn categorized(&self, x: usize, y: usize) -> bool {
        self.points.contains(&(x, y))
            || self.liberties.contains(&(x, y))
            || self.enemy_neighbors.contains(&(x, y))
    }
}
