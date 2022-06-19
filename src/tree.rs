use crate::Event;

#[derive(Clone, Debug)]
struct EventNode {
    event: Event,
    children: Vec<EventNode>,
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
}

#[derive(Clone, Debug)]
pub struct EventTree {
    root: EventNode,
    path: Vec<usize>,
}
impl EventTree {
    /// An empty EventTree, with nothing but an [Event::Start] in the root.
    pub fn blank() -> Self {
        Self {
            root: EventNode::new(Event::Start),
            path: Vec::new(),
        }
    }

    fn get_current_node(&self) -> &EventNode {
        let mut current_node = &self.root;

        for i in &self.path {
            current_node = current_node.get_child(*i).unwrap();
        }

        return current_node;
    }

    fn get_current_node_mut(&mut self) -> &mut EventNode {
        let mut current_node = &mut self.root;

        for i in &self.path {
            current_node = current_node.get_child_mut(*i).unwrap();
        }

        return current_node;
    }

    pub fn get_current_event(&self) -> Event {
        return self.get_current_node().event;
    }

    pub fn get_path(&self) -> Vec<Event> {
        let mut vec = Vec::new();
        let mut current_node = &self.root;

        for i in &self.path {
            current_node = current_node.get_child(*i).unwrap();

            vec.push(current_node.event);
        }

        return vec;
    }

    pub fn push(&mut self, e: Event) {
        let current_node = self.get_current_node_mut();
        let last_idx = current_node.children.len();

        current_node.children.push(EventNode::new(e));

        self.path.push(last_idx);
    }

    pub fn move_to_parent(&mut self) {
        self.path.pop();
    }

    pub fn move_to_first_child(&mut self) {
        if !self.get_current_node().children.is_empty() {
            self.path.push(0);
        }
    }

    pub fn move_to_last_sibling(&mut self) {
        let last = match self.path.last_mut() {
            Some(n) => n,
            None => return,
        };
        if *last == 0 {
            return
        }

        *last = *last - 1;
    }

    pub fn move_to_next_sibling(&mut self) {
        let last = match self.path.pop() {
            Some(n) => n,
            None => return,
        };
        let len = self.get_current_node().children.len();

        if last + 1 >= len {
            self.path.push(last);

            return
        }

        self.path.push(last + 1)
    }

    /// Remove the current node and move to the parent
    pub fn pop(&mut self) -> Option<Event> {
        let last_idx = self.path.pop()?;
        let node = self.get_current_node_mut();

        return Some(node.children.remove(last_idx).event);
    }
}
