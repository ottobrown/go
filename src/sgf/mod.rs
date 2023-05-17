mod action;
mod error;
mod util;
pub use action::Action;
pub use error::{SgfError, SgfResult};

#[derive(Debug)]
pub struct SgfTree {
    /// The root is stored at sequences[0]
    sequences: Vec<SgfSequence>,
    current: usize,
}
impl SgfTree {
    /*
    pub fn parse(s: String) -> SgfTree {
        todo!();
    }
    */

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

    pub fn handle_new_text(&mut self, s: String) {
        self.sequences[self.current].handle_new_text(s);
    }
}

impl Default for SgfTree {
    fn default() -> Self {
        Self {
            sequences: vec![SgfSequence::default()],
            current: 0,
        }
    }
}

/// Consists of several sgf nodes, but forms a single node in the sgf tree.
///
/// note the difference between a 'tree node' (like a node in a graph) and an
/// 'sgf node' (a list of sgf properties). SgfSequence is a tree node, but
/// consists of many contiguous sgf nodes.
#[derive(Default, Debug)]
pub struct SgfSequence {
    pub nodes: Vec<String>,

    /// Indices on the parent `SgfTree::sequences`
    children: Vec<usize>,
    /// All sequences have a parent except the root node
    parent: Option<usize>,
}
impl SgfSequence {
    pub fn handle_new_text(&mut self, s: String) {
        if s.starts_with(';') {
            self.nodes.push(s);
            return;
        }

        if let Some(l) = self.nodes.last_mut() {
            l.push_str(&s);
        } else {
            // if self.nodes is empty
            self.nodes.push(s);
        }
    }
}
