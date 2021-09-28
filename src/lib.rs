//! nodes: a graph based data organization
//!

/// Node is the connecting piece of the organization
#[derive(Debug)]
pub enum Content {
    Empty,
    Edges(Vec<Node>),
    String(String),
}

#[derive(Debug)]
pub struct Node {
    id: u32,
    tags: Vec<String>,
    content: Content,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Node {
            id,
            tags: Vec::new(),
            content: Content::Empty,
        }
    }

    pub fn new_edges(id: u32, edges: Vec<Node>) -> Self {
        Node {
            id,
            tags: Vec::new(),
            content: Content::Edges(edges),
        }
    }

    pub fn new_string(id: u32, s: &str) -> Self {
        Node {
            id,
            tags: Vec::new(),
            content: Content::String(s.to_owned()),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: 0,
            tags: Vec::new(),
            content: Content::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let x: Node = Default::default();
        match x.content {
            Content::Empty => {}
            _ => panic!("invalid node default"),
        }
    }

    #[test]
    fn new_edge_is_empty() {
        const ID: u32 = 43;
        let x = Node::new_edges(ID, vec![]);
        assert_eq!(x.id, ID);
        if let Content::Edges(edges) = x.content {
            assert!(edges.is_empty())
        } else {
            panic!("unknown content");
        }
    }
}
