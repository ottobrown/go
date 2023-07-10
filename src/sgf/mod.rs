mod action;
mod error;
mod util;
pub use action::{to_actions, Action};
pub use error::{SgfError, SgfResult};

#[derive(Debug, PartialEq)]
pub struct SgfTree {
    /// The root is stored at nodes[0]
    nodes: Vec<SgfNode>,
    current: usize,
}
impl SgfTree {
    pub fn parse(s: String) -> SgfResult<SgfTree> {
        parse(lex(s))
    }

    pub fn to_text(&self) -> String {
        let mut s = String::from('(');
        self.stringify_node(0, &mut s);
        s.push(')');

        s
    }

    fn stringify_node(&self, node_i: usize, s: &mut String) {
        let node = &self.nodes[node_i];
        s.push(';');

        for a in &node.actions {
            // TODO: handle this error?
            s.push_str(&a.to_sgf_text().unwrap());
        }
        match node.children.len() {
            0 => {}
            1 => self.stringify_node(node.children[0], s),

            _ => {
                for c in &node.children {
                    s.push('(');
                    self.stringify_node(*c, s);
                    s.push(')');
                }
            }
        };
    }

    pub fn current_node(&self) -> &SgfNode {
        &self.nodes[self.current]
    }

    pub fn root(&self) -> &SgfNode {
        &self.nodes[0]
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

    pub fn set_root(&mut self, s: String) -> SgfResult<()> {
        self.nodes[0].actions = to_actions(&s);

        Ok(())
    }

    pub fn select_root(&mut self) {
        self.current = 0;
    }

    pub fn handle_new_text(&mut self, s: String) {
        if s.starts_with(';') {
            let n = SgfNode {
                actions: to_actions(&s),
                parent: Some(self.current),
                children: Vec::new(),
            };

            let l = self.nodes.len();
            self.nodes.push(n);
            self.nodes[self.current].children.push(l);

            self.current = l;
        } else {
            self.nodes[self.current].actions.extend(to_actions(&s));
        }
    }

    /// The action of the current node, followed by the action of the parent,
    /// followed by the action of the parent's parent, all the way to the root node
    pub fn get_all_parent_action(&self) -> Vec<Vec<Action>> {
        let mut node = self.current_node();
        let mut all = Vec::new();

        while let Some(p) = node.parent {
            all.push(node.actions.clone());
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

#[derive(Default, Debug, PartialEq)]
pub struct SgfNode {
    pub actions: Vec<Action>,

    /// Indices on the parent `SgfTree::sequences`
    children: Vec<usize>,
    /// All nodes have a parent except the root node
    parent: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParserToken {
    /// '('
    LParen,
    /// ')'
    RParen,
    /// ';' followed by text
    Node(String),
}

fn lex(s: String) -> Vec<ParserToken> {
    use ParserToken::*;

    let mut tokens = Vec::new();
    let mut node = String::new();

    for ch in s.chars() {
        match ch {
            '(' => {
                if node.starts_with(';') {
                    tokens.push(Node(node.trim().to_string()));
                    node.clear();
                }
                tokens.push(LParen);
            }
            ')' => {
                if node.starts_with(';') {
                    tokens.push(Node(node.trim().to_string()));
                    node.clear();
                }
                tokens.push(RParen);
            }
            ';' => {
                if node.starts_with(';') {
                    tokens.push(Node(node.trim().to_string()));
                    node.clear();
                }
                node.push(';');
            }

            _ => {
                if node.starts_with(';') {
                    node.push(ch);
                }
            }
        }
    }

    tokens
}

fn parse(tokens: Vec<ParserToken>) -> SgfResult<SgfTree> {
    let mut tree = SgfTree::default();
    let mut iter = tokens.iter();
    let mut stack = Vec::new();

    // parse root node
    if iter.next() != Some(&ParserToken::LParen) {
        return Err(SgfError::MissingLParen);
    }
    if let Some(ParserToken::Node(s)) = iter.next() {
        tree.set_root(s.clone())?;
    }

    for token in iter {
        match token {
            ParserToken::Node(s) => {
                tree.handle_new_text(s.clone());
            }

            ParserToken::LParen => {
                stack.push(tree.current);
            }

            ParserToken::RParen => {
                if let Some(n) = stack.pop() {
                    tree.current = n;
                }
            }
        }
    }

    tree.select_root();
    Ok(tree)
}

#[cfg(test)]
mod sgf_tests {
    use super::*;

    #[test]
    fn lex_test() {
        use ParserToken::*;

        let s = String::from("(;FF[4]  ;B[pd]   ;W[dp];B[dd](;W[qp];B[oq])(;W[pq];B[qo]))");
        let l = vec![
            LParen,
            Node(String::from(";FF[4]")),
            Node(String::from(";B[pd]")),
            Node(String::from(";W[dp]")),
            Node(String::from(";B[dd]")),
            LParen,
            Node(String::from(";W[qp]")),
            Node(String::from(";B[oq]")),
            RParen,
            LParen,
            Node(String::from(";W[pq]")),
            Node(String::from(";B[qo]")),
            RParen,
            RParen,
        ];

        assert_eq!(lex(s), l);
    }

    #[test]
    fn parse_test() {
        use ParserToken::*;

        let l = vec![
            LParen,
            Node(String::from(";FF[4]")),
            Node(String::from(";B[pd]")),
            Node(String::from(";W[dp]")),
            Node(String::from(";B[dd]")),
            LParen,
            Node(String::from(";W[qp]")),
            Node(String::from(";B[oq]")),
            RParen,
            LParen,
            Node(String::from(";W[pq]")),
            Node(String::from(";B[qo]")),
            RParen,
            RParen,
        ];
        let t = SgfTree {
            nodes: vec![
                SgfNode {
                    // 0
                    actions: vec![Action::Other("FF".to_string(), "4".to_string())],
                    children: vec![1],
                    parent: None,
                },
                SgfNode {
                    // 1
                    actions: vec![Action::PlayBlack(15, 3)],
                    children: vec![2],
                    parent: Some(0),
                },
                SgfNode {
                    // 2
                    actions: vec![Action::PlayWhite(3, 15)],
                    children: vec![3],
                    parent: Some(1),
                },
                SgfNode {
                    // 3
                    actions: vec![Action::PlayBlack(3, 3)],
                    children: vec![4, 6],
                    parent: Some(2),
                },
                SgfNode {
                    // 4
                    actions: vec![Action::PlayWhite(16, 15)],
                    children: vec![5],
                    parent: Some(3),
                },
                SgfNode {
                    // 5
                    actions: vec![Action::PlayBlack(14, 16)],
                    children: vec![],
                    parent: Some(4),
                },
                SgfNode {
                    // 6
                    actions: vec![Action::PlayWhite(15, 16)],
                    children: vec![7],
                    parent: Some(3),
                },
                SgfNode {
                    // 7
                    actions: vec![Action::PlayBlack(16, 14)],
                    children: vec![],
                    parent: Some(6),
                },
            ],
            current: 0,
        };

        assert_eq!(parse(l).unwrap(), t);
    }

    #[test]
    fn to_text_test() {
        let s = "(;FF[4];B[pd];W[dp];B[dd](;W[qp];B[oq])(;W[pq];B[qo]))".to_string();

        assert_eq!(s, SgfTree::parse(s.clone()).unwrap().to_text());
    }
}
