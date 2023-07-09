#[derive(Debug)]
pub enum SgfError {
    /// Trying to select a child of a node which doesn't exist
    ChildDoesntExist,
    /// Trying to accesss the parent of the root node
    ParentOfRoot,
    /// A coordinate > 51 (The max sgf allows)
    CoordTooBig,
    /// Trying to parse a char as a coordinate that isn't a..z or A..Z
    InvalidCoordChar,
    /// if there is no LParen at the start of an sgf file
    MissingLParen,
    /// Error parsing SZ[] prop
    SizeParse,
    /// a coordinate should be 2 ASCII characters
    InvalidLength,

    Io(std::io::Error),
}

impl From<std::io::Error> for SgfError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

pub type SgfResult<T> = Result<T, SgfError>;
