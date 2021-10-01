use std::collections::HashMap;
use thiserror::Error;

use crate::node;

#[derive(Error, Debug)]
pub enum NodeRepoError {
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

pub fn traverse(repo: &dyn NodeRepo, id: &node::NodeId) -> Result<(), NodeRepoError> {
    if let Some(n) = repo.get(id)? {
        println!("{:?}", n);
        for edge in &n.edges {
            traverse(repo, edge)?;
        }
    }

    Ok(())
}
