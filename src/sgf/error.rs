#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SgfError {
    /// Trying to select a child of a node which doesn't exist
    ChildDoesntExist,
    /// Trying to accesss the parent of the root node
    ParentOfRoot,
    /// A coordinate > 51 (The max sgf allows)
    CoordTooBig,
}

pub type SgfResult<T> = Result<T, SgfError>;
