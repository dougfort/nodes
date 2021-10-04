use std::collections::{HashMap, HashSet};
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
    /// it takes a filter function that is passed the tags and can return true
    /// or false to accept or ignore the node.
    fn traverse<F>(&self, filter: F) -> Result<Vec<node::Content>, NodeRepoError>
    where
        F: Fn(HashSet<String>) -> bool,
    {
        let mut content: Vec<node::Content> = Vec::new();

        let mut stack = vec![self.root()];

        while !stack.is_empty() {
            let id = stack.pop().unwrap();
            if let Some(n) = self.get(&id)? {
                match n.content {
                    node::Content::Edges(e) => stack.extend_from_slice(&e),
                    _ => {
                        if filter(n.tags) {
                            content.push(n.content);
                        }
                    }
                }
            } else {
                return Err(NodeRepoError::UnknownNodeId(id));
            }
        }

        Ok(content)
    }
}

/// create_match_tag_filter returns a closure for use as a filter in traverse
/// the closure will return true for a node if 'tag' matches any of its tags
pub fn create_match_tag_filter(tag: &str) -> impl Fn(HashSet<String>) -> bool {
    let t = tag.to_owned();
    move |tags| tags.get(&t).is_some()
}

/// create_accept_all_filter returns a closure for use as a filter in traverse
/// the closure will return true for any set of tags including the empty set
pub fn create_accept_all_filter() -> impl Fn(HashSet<String>) -> bool {
    move |_| true
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

        let root = node::Node::new(id, node::Content::Edges(vec![]));
        repo.put(&root).unwrap();

        let x = repo.get(&root_id).unwrap();
        assert!(x.is_some());
    }

    #[test]
    fn hash_map_repo_can_traverse() {
        let mut repo = HashMapRepo::new();

        let root_id = repo.root();
        let mut id = root_id.into();

        id += 1;
        let test_slice = "aaa";
        let test_string = test_slice.to_string();
        let s = node::Node::new(id, node::Content::String(test_string));
        repo.put(&s).unwrap();

        id += 1;
        let e = node::Node::new(id, node::Content::Edges(vec![s.id]));
        repo.put(&e).unwrap();

        let root = node::Node::new(root_id.into(), node::Content::Edges(vec![e.id]));
        repo.put(&root).unwrap();

        let res = repo.traverse(create_accept_all_filter()).unwrap();
        assert_eq!(res.len(), 1);

        match &res[0] {
            node::Content::String(res_string) => assert_eq!(res_string, test_slice),
            _ => panic!("invalid content"),
        };
    }

    #[test]
    fn hash_map_repo_can_find_a_tag() {
        let mut repo = HashMapRepo::new();

        let root_id = repo.root();
        let mut id = root_id.into();

        id += 1;
        let test_slice1 = "aaa";
        let test_string1 = test_slice1.to_string();
        let test_tag_slice1 = "tag1";
        let test_tag1 = test_tag_slice1.to_string();
        let mut s1 = node::Node::new(id, node::Content::String(test_string1));
        s1.tags.insert(test_tag1);
        repo.put(&s1).unwrap();

        id += 1;
        let test_slice2 = "bbb";
        let test_string2 = test_slice2.to_string();
        let test_tag_slice2 = "tag2";
        let test_tag2 = test_tag_slice2.to_string();
        let mut s2 = node::Node::new(id, node::Content::String(test_string2));
        s2.tags.insert(test_tag2);
        repo.put(&s2).unwrap();

        id += 1;
        let e = node::Node::new(id, node::Content::Edges(vec![s1.id, s2.id]));
        repo.put(&e).unwrap();

        let root = node::Node::new(root_id.into(), node::Content::Edges(vec![e.id]));
        repo.put(&root).unwrap();

        let res = repo
            .traverse(create_match_tag_filter(test_tag_slice2))
            .unwrap();
        assert_eq!(res.len(), 1);

        match &res[0] {
            node::Content::String(res_string) => assert_eq!(res_string, test_slice2),
            _ => panic!("invalid content"),
        };
    }
}
