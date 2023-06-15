mod action;
mod error;
mod parse;
mod util;
pub use action::{to_actions, Action};
pub use error::{SgfError, SgfResult};

#[derive(Debug)]
pub struct SgfTree {
    /// The root is stored at nodes[0]
    nodes: Vec<SgfNode>,
    current: usize,
}
impl SgfTree {
    pub fn parse(s: String) -> SgfResult<SgfTree> {
        parse::parse(parse::lex(s))
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

    pub fn set_root(&mut self, s: String) {
        self.nodes[0].text = s;
    }

    pub fn select_root(&mut self) {
        self.current = 0;
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

    /// The text of the current node, followed by the text of the parent,
    /// followed by the text of the parent's parent, all the way to the root node
    pub fn get_all_parent_text(&self) -> Vec<String> {
        let mut node = self.current_node();
        let mut all = Vec::new();

        while let Some(p) = node.parent {
            all.push(node.text.clone());
            node = &self.nodes[p];
        }

        all
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

#[derive(Default, Debug)]
pub struct SgfNode {
    pub text: String,

    /// Indices on the parent `SgfTree::sequences`
    children: Vec<usize>,
    /// All nodes have a parent except the root node
    parent: Option<usize>,
}
