pub struct SgfTree {
    nodes: Vec<SgfNode>,
    current: usize,
}
impl SgfTree {
    pub fn parse(s: String) -> SgfTree {
        todo!();
    }
}

pub struct SgfNode {
    pub text: String,

    /// Indices on the parent `SgfTree::nodes`
    children: Vec<usize>,
}
