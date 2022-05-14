use array2d::Array2D;

#[derive(Clone, Copy, PartialEq)]
pub enum Stone {
    Empty = 0,
    Black = 1,
    White = 2,
}

/// The state of a go game at a single point in time
pub struct Board {
    pub stones: Array2D<Stone>,
}
