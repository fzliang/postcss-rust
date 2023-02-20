use super::{container::Container, node::Node, root::Root};

#[derive(Debug)]
pub enum NodeType<'a> {
    Root(Box<Root<'a>>),
    Container(Box<Container<'a>>),
    Node(Box<Node<'a>>),
}
