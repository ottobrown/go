use crate::Board;
use crate::Stone;
use std::collections::HashSet;

/// Use the flood-fill algorithm to find a continuous group of stones,
/// including bordering liberties and enemy stones.
/// Will assume that a stone of color `s` is located at `(x, y)`
pub fn find_group(board: &Board, x: usize, y: usize, color: Stone) -> Group {
    if color == Stone::Empty {
        panic!["cannot find a group of an empty stone!"];
    }

    let mut stack = vec![(x, y)];
    let mut group = Group::new();

    let (w, h) = board.size();

    while let Some(p) = stack.pop() {
        if group.categorized(&p) {
            continue;
        }

        let s = board.get(p.0, p.1);

        if s == color || p == (x, y) {
            group.inside.insert(p);

            if p.0 < w - 1 {
                stack.push((p.0 + 1, p.1));
            }

            if p.1 < h - 1 {
                stack.push((p.0, p.1 + 1));
            }

            if p.0 > 0 {
                stack.push((p.0 - 1, p.1));
            }

            if p.1 > 0 {
                stack.push((p.0, p.1 - 1));
            }
        } else if s == !color {
            group.neighbors.insert(p);
        } else {
            group.liberties.insert(p);
        }
    }

    return group;
}

pub struct Group {
    pub inside: HashSet<(usize, usize)>,
    pub liberties: HashSet<(usize, usize)>,
    pub neighbors: HashSet<(usize, usize)>,
}
impl Group {
    pub fn new() -> Self {
        Self {
            inside: HashSet::new(),
            liberties: HashSet::new(),
            neighbors: HashSet::new(),
        }
    }

    pub fn categorized(&self, p: &(usize, usize)) -> bool {
        self.inside.contains(p) || self.liberties.contains(p) || self.neighbors.contains(p)
    }
}

#[cfg(test)]
mod flood_fill_tests {
    use super::*;

    #[test]
    fn top_left_corner_group() {
        let mut board = Board::new(9, 9);

        // b b w + + + + + +
        // b w + + + + + + +
        // b w + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +

        board.set(0, 0, Stone::Black);
        board.set(1, 0, Stone::Black);
        board.set(0, 1, Stone::Black);
        board.set(0, 2, Stone::Black);

        board.set(2, 0, Stone::White);
        board.set(1, 1, Stone::White);
        board.set(1, 2, Stone::White);

        let group = find_group(&board, 0, 0, Stone::Black);

        assert_eq!(
            group.inside,
            HashSet::from([(0, 0), (1, 0), (0, 1), (0, 2)])
        );

        assert_eq!(group.neighbors, HashSet::from([(2, 0), (1, 1), (1, 2)]));

        assert_eq!(group.liberties, HashSet::from([(0, 3)]));
    }

    #[test]
    fn bottom_right_corner_group() {
        let mut board = Board::new(9, 9);

        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + + +
        // + + + + + + + w +
        // + + + + + + w b +
        // + + + + + + w b b
        // + + + + + + + + b

        board.set(8, 8, Stone::Black);
        board.set(8, 7, Stone::Black);
        board.set(7, 7, Stone::Black);
        board.set(7, 6, Stone::Black);

        board.set(7, 5, Stone::White);
        board.set(6, 6, Stone::White);
        board.set(6, 7, Stone::White);

        let group = find_group(&board, 8, 8, Stone::Black);

        assert_eq!(
            group.inside,
            HashSet::from([(8, 8), (8, 7), (7, 7), (7, 6)])
        );

        assert_eq!(group.neighbors, HashSet::from([(7, 5), (6, 6), (6, 7)]));

        assert_eq!(group.liberties, HashSet::from([(7, 8), (8, 6)]));
    }

    #[test]
    fn single_group_middle() {
        let mut board = Board::new(9, 9);

        board.set(5, 4, Stone::Black);

        let group = find_group(&board, 5, 4, Stone::Black);

        assert_eq!(group.inside, HashSet::from([(5, 4)]));

        assert_eq!(
            group.liberties,
            HashSet::from([(5, 5), (4, 4), (5, 3), (6, 4)])
        );
    }
}
