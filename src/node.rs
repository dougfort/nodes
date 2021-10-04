//! nodes: a graph based data organization
//!
//!
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("unknown node error")]
    Unknown,
}

/// Node is the connecting piece of the organization
#[derive(Debug, Clone)]
pub enum Content {
    Edges(Vec<NodeId>),
    String(String),
    Path(String),
}

#[derive(Copy, Clone, Debug)]
pub struct NodeId(usize);

impl From<NodeId> for usize {
    fn from(n: NodeId) -> usize {
        n.0
    }
}

impl From<usize> for NodeId {
    fn from(n: usize) -> NodeId {
        NodeId(n)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub tags: HashSet<String>,
    pub content: Content,
}

impl Node {
    pub fn new(id: usize, content: Content) -> Self {
        Node {
            id: NodeId(id),
            tags: HashSet::new(),
            content,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: NodeId(0),
            tags: HashSet::new(),
            content: Content::Edges(vec![]),
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
            Content::Edges(edges) => assert!(edges.is_empty()),
            _ => panic!("invalid content"),
        };
    }
}
