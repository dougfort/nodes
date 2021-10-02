use std::collections::HashMap;
use thiserror::Error;

use crate::node;

#[derive(Error, Debug)]
pub enum NodeRepoError {
    #[error("unknown NodeId: {0:?}")]
    UnknownNodeId(node::NodeId),

    #[error("unknown node repo error")]
    Unknown,
}

pub trait NodeRepo {
    /// root returns the NodeId of the root node
    fn root(&self) -> node::NodeId;

    /// get retrieves Node with the given id
    /// if there is no such node, it returns None
    fn get(&self, id: &node::NodeId) -> Result<Option<node::Node>, NodeRepoError>;

    /// put stores the Node in the repository
    /// if there is already a node stored for the id, it iis overwritten
    fn put(&mut self, node: &node::Node) -> Result<(), NodeRepoError>;

    /// traverse visits every node with a breadth first search BFS
    fn traverse<F>(&self, filter: F) -> Result<Vec<node::Content>, NodeRepoError>
    where
        F: Fn(Vec<String>) -> bool,
    {
        let mut content: Vec<node::Content> = Vec::new();

        let mut stack = vec![self.root()];

        while !stack.is_empty() {
            let id = stack.pop().unwrap();
            if let Some(n) = self.get(&id)? {
                if filter(n.tags) {
                    if let Some(c) = n.content {
                        content.push(c);
                    }
                }
                for edge in n.edges {
                    stack.push(edge);
                }
            } else {
                return Err(NodeRepoError::UnknownNodeId(id));
            }
        }

        Ok(content)
    }
}

pub struct HashMapRepo {
    root: node::NodeId,
    pub repo: HashMap<usize, node::Node>,
}

impl HashMapRepo {
    pub fn new() -> Self {
        let root: node::NodeId = 0.into();
        HashMapRepo {
            root,
            repo: HashMap::new(),
        }
    }
}

impl Default for HashMapRepo {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeRepo for HashMapRepo {
    /// root returns the NodeId of the root node
    fn root(&self) -> node::NodeId {
        self.root
    }

    /// get retrieves Node with the given id
    /// if there is no such node, it returns None
    fn get(&self, id: &node::NodeId) -> Result<Option<node::Node>, NodeRepoError> {
        let uid = *id;
        Ok(self.repo.get(&uid.into()).cloned())
    }

    /// put stores the Node in the repository
    /// if there is already a node stored for the id, it iis overwritten
    fn put(&mut self, node: &node::Node) -> Result<(), NodeRepoError> {
        let key: usize = node.id.into();
        let value = node.clone();
        self.repo.insert(key, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_map_repo_puts_and_gets() {
        let mut repo = HashMapRepo::new();

        let root_id = repo.root();
        let id = root_id.into();

        let root = node::Node::new(id, None);
        repo.put(&root).unwrap();

        let x = repo.get(&root_id).unwrap();
        assert!(x.is_some());
    }

    #[test]
    fn hash_map_repo_can_traverse() {
        let mut repo = HashMapRepo::new();

        let root_id = repo.root();
        let mut id = root_id.into();

        let mut root = node::Node::new(id, None);
        repo.put(&root).unwrap();

        id += 1;
        let test_slice = "aaa";
        let test_string = test_slice.to_string();
        let s = node::Node::new(id, Some(node::Content::String(test_string)));
        repo.put(&s).unwrap();

        id += 1;
        let mut e = node::Node::new(id, None);
        e.edges = vec![s.id];
        repo.put(&e).unwrap();

        root.edges.push(e.id);
        repo.put(&root).unwrap();

        let res = repo.traverse(|_| true).unwrap();
        assert_eq!(res.len(), 1);

        match &res[0] {
            node::Content::String(res_string) => assert_eq!(res_string, test_slice),
            _ => panic!("invalid content"),
        };
    }
}
