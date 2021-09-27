//! nodes: a graph based data organization
//!

/// Node is the connecting piece of the organization
#[derive(Debug)]
pub enum Content {
    Terminus,
    Edges(std::vec::Vec<Node>),
}

#[derive(Debug)]
pub struct Node {
    id: u32,
    content: Content,
}

impl Node {
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: 0,
            content: Content::Terminus
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_terminus() {
        let x: Node = Default::default();
        match x.content {
            Content::Terminus => {}
            _ => panic!("invalid node default")
        }
    }
}
