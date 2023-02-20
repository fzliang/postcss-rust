use super::container::Container;

#[derive(Debug)]
pub struct Root<'a> {
    container: Container<'a>,
}

impl<'a> Root<'a> {
    pub fn new() -> Self {
        Root {
            container: Container::new_by_node_type("root"),
        }
    }
}
