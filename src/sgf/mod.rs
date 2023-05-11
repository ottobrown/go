mod error;
pub use error::*;

#[derive(Default)]
pub struct SgfTree {
    /// The root is stored at sequences[0]
    sequences: Vec<SgfSequence>,
    current: usize,
}
impl SgfTree {
    pub fn parse(s: String) -> SgfTree {
        todo!();
    }

    pub fn current_sequence(&self) -> &SgfSequence {
        &self.sequences[self.current]
    }

    /// The number of children the current sequence has
    pub fn num_children(&self) -> usize {
        self.sequences[self.current].children.len()
    }

    /// sets the specified child of the current sequence (if it exists) as the new
    /// current sequence, and returns a reference to the new current
    pub fn select_child(&mut self, child: usize) -> SgfResult<&SgfSequence> {
        if let Some(i) = self.sequences[self.current].children.get(child) {
            self.current = *i;

            return Ok(&self.sequences[*i]);
        }

        Err(SgfError::ChildDoesntExist)
    }

    pub fn select_parent(&mut self) -> SgfResult<()> {
        if let Some(i) = self.sequences[self.current].parent {
            self.current = i;

            return Ok(());
        }

        Err(SgfError::ParentOfRoot)
    }
}

/// Consists of several sgf nodes, but forms a single node in the sgf tree.
///
/// note the difference between a 'tree node' (like a node in a graph) and an
/// 'sgf node' (a list of sgf properties). SgfSequence is a tree node, but
/// consists of many contiguous sgf nodes.
#[derive(Default)]
pub struct SgfSequence {
    pub text: String,

    /// Indices on the parent `SgfTree::sequences`
    children: Vec<usize>,
    /// All sequences have a parent except the root node
    parent: Option<usize>,
}
