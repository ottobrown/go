mod error;
pub use error::*;

pub struct SgfTree {
    /// The root is stored at nodes[0]
    nodes: Vec<SgfNode>,
    current: usize,
}
impl SgfTree {
    pub fn parse(s: String) -> SgfTree {
        todo!();
    }

    pub fn current_node(&self) -> &SgfNode {
        &self.nodes[self.current]
    }

    /// The number of children the current node has
    pub fn num_children(&self) -> usize {
        self.nodes[self.current].children.len()
    }

    /// sets the specified child of the current node (if it exists) as the new
    /// current node, and returns a reference to the new current
    pub fn select_child(&mut self, child: usize) -> SgfResult<&SgfNode> {
        if let Some(i) = self.nodes[self.current].children.get(child) {
            self.current = *i;

            return Ok(&self.nodes[*i]);
        }

        Err(SgfError::ChildDoesntExist)
    }

    pub fn select_parent(&mut self) -> SgfResult<()> {
        if let Some(i) = self.nodes[self.current].parent {
            self.current = i;

            return Ok(());
        }

        Err(SgfError::ParentOfRoot)
    }
}

pub struct SgfNode {
    pub text: String,

    /// Indices on the parent `SgfTree::nodes`
    children: Vec<usize>,
    /// All nodes have a parent except the root node
    parent: Option<usize>,
}
