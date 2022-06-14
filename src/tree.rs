use crate::Event;

pub struct EventNode {
    e: Event,
    children: Vec<EventNode>,
}

pub struct EventTree {
    root: EventNode,
    path: Vec<usize>,
}
