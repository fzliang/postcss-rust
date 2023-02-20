use super::node::Node;

#[derive(Debug)]
pub struct Container<'a> {
    node: Node<'a>,
}

impl<'a> Container<'a> {
    pub fn new() -> Self {
        Container {
            node: Node::new("document"),
        }
    }

    pub fn new_by_node_type(node_type: &'a str) -> Self {
        Container {
            node: Node::new(node_type),
        }
    }
}
