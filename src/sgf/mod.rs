mod action;
mod error;
mod util;
pub use action::Action;
pub use error::{SgfError, SgfResult};

#[derive(Debug)]
pub struct SgfTree {
    /// The root is stored at nodes[0]
    nodes: Vec<SgfNode>,
    current: usize,
}
impl SgfTree {
    /*
    pub fn parse(s: String) -> SgfTree {
        todo!();
    }
    */

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

    pub fn handle_new_text(&mut self, s: String) {
        if s.starts_with(';') {
            let n = SgfNode {
                text: s,
                parent: Some(self.current),
                children: Vec::new(),
            };
            let l = self.nodes.len();
            self.nodes.push(n);
            self.nodes[self.current].children.push(l);

            self.current = l;
        } else {
            self.nodes[self.current].text.push_str(&s);
        }
    }
}

impl Default for SgfTree {
    fn default() -> Self {
        Self {
            nodes: vec![SgfNode::default()],
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
pub struct SgfNode {
    pub text: String,

    /// Indices on the parent `SgfTree::sequences`
    children: Vec<usize>,
    /// All nodes have a parent except the root node
    parent: Option<usize>,
}
