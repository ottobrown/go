use crate::game::Event;

#[derive(Clone, Debug)]
struct EventNode {
    event: Event,
    children: Vec<Self>,
}
impl EventNode {
    pub fn new(e: Event) -> Self {
        Self {
            event: e,
            children: Vec::new(),
        }
    }

    pub fn get_child(&self, i: usize) -> Option<&Self> {
        self.children.get(i)
    }

    pub fn get_child_mut(&mut self, i: usize) -> Option<&mut Self> {
        self.children.get_mut(i)
    }

    /// returns the last index of self.children
    pub fn append(&mut self, e: Event) -> usize {
        let new = Self::new(e);
        self.children.push(new);

        return self.children.len() - 1;
    }
}

#[derive(Clone, Debug)]
pub struct EventTree {
    root: EventNode,
    current_path: Vec<usize>,
}
impl EventTree {
    pub fn new() -> Self {
        Self {
            root: EventNode::new(Event::Start),
            current_path: Vec::new(),
        }
    }

    /// pops the last element on self.current_path
    fn move_to_parent(&mut self) {
        self.current_path.pop();
    }

    pub fn append_to_current_node(&mut self, e: Event) {
        let i = self.get_current_node_mut().append(e);

        self.current_path.push(i);
    }

    fn get_current_node(&self) -> &EventNode {
        let mut node = &self.root;
        for i in &self.current_path {
            node = node.get_child(*i).unwrap();
        }

        return node;
    }

    fn get_current_node_mut(&mut self) -> &mut EventNode {
        let mut node = &mut self.root;
        for i in &self.current_path {
            node = node.get_child_mut(*i).unwrap();
        }

        return node;
    }
}
