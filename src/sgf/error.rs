#[derive(Clone, Copy, Debug)]
pub enum SgfError {
    /// Trying to select a child of a node which doesn't exist
    ChildDoesntExist,
    /// Trying to accesss the parent of the root node
    ParentOfRoot,
}

pub type SgfResult<T> = Result<T, SgfError>;
