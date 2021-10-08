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
        let mut visited: HashSet<usize> = HashSet::new();

        while !stack.is_empty() {
            let id = stack.pop().unwrap();
            if !visited.contains(&id.into()) {
                visited.insert(id.into());

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
        }

        Ok(content)
    }

    /// bfs_dump walks through the repo, breadth first and dumps the content
    fn bfs_dump(&self) -> Result<(), NodeRepoError> {
        let mut stack = vec![self.root()];
        let mut visited: HashSet<usize> = HashSet::new();

        while !stack.is_empty() {
            let id = stack.pop().unwrap();
            if !visited.contains(&id.into()) {
                visited.insert(id.into());
                if let Some(n) = self.get(&id)? {
                    println!("{:?}", n);
                    if let node::Content::Edges(e) = n.content {
                        stack.extend_from_slice(&e)
                    };
                }
            } else {
                return Err(NodeRepoError::UnknownNodeId(id));
            }
        }

        Ok(())
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
    repo: HashMap<usize, node::Node>,
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
    use crate::node::Content;
    use crate::node::Node;
    use crate::node::NodeId;

    #[test]
    fn hash_map_repo_puts_and_gets() {
        let mut repo = HashMapRepo::new();

        let root_id = repo.root();
        let id = root_id.into();

        let root = node::Node::new(id, vec![], node::Content::Edges(vec![]));
        repo.put(&root).unwrap();

        let x = repo.get(&root_id).unwrap();
        assert!(x.is_some());
    }

    #[test]
    fn hash_map_repo_can_traverse() {
        let mut repo = HashMapRepo::new();

        for (id, s, tags, e) in vec![
            (1, "aaa", vec!["tag1"], vec![]),
            (2, "", vec![], vec![1]),
            (0, "", vec![], vec![2]),
        ] {
            let mut edges: Vec<NodeId> = Vec::new();
            for e_id in e {
                edges.push(e_id.into())
            }
            let content = if s.is_empty() {
                Content::Edges(edges)
            } else {
                Content::String(s.to_string())
            };
            let n = Node::new(id, tags, content);
            repo.put(&n).unwrap();
        }

        let res = repo.traverse(create_accept_all_filter()).unwrap();
        assert_eq!(res.len(), 1);

        match &res[0] {
            node::Content::String(res_string) => assert_eq!(res_string, "aaa"),
            _ => panic!("invalid content"),
        };
    }

    #[test]
    fn hash_map_repo_can_find_a_tag() {
        let mut repo = HashMapRepo::new();

        for (id, s, tags, e) in vec![
            (1, "aaa", vec!["tag1"], vec![]),
            (2, "bbb", vec!["tag2"], vec![]),
            (3, "", vec![], vec![1, 2]),
            (0, "", vec![], vec![3]),
        ] {
            let mut edges: Vec<NodeId> = Vec::new();
            for e_id in e {
                edges.push(e_id.into())
            }
            let content = if s.is_empty() {
                Content::Edges(edges)
            } else {
                Content::String(s.to_string())
            };
            let n = Node::new(id, tags, content);
            repo.put(&n).unwrap();
        }

        let res = repo.traverse(create_match_tag_filter("tag2")).unwrap();
        assert_eq!(res.len(), 1);

        match &res[0] {
            node::Content::String(res_string) => assert_eq!(res_string, "bbb"),
            _ => panic!("invalid content"),
        };
    }

    #[test]
    fn hash_map_repo_can_handles_a_loop() {
        let mut repo = HashMapRepo::new();

        for (id, s, tags, e) in vec![
            (1, "aaa", vec!["tag1"], vec![]),
            (2, "bbb", vec!["tag2"], vec![]),
            (3, "", vec![], vec![1, 2]),
            (0, "", vec![], vec![3]),
        ] {
            let mut edges: Vec<NodeId> = Vec::new();
            for e_id in e {
                edges.push(e_id.into())
            }
            let content = if s.is_empty() {
                Content::Edges(edges)
            } else {
                Content::String(s.to_string())
            };
            let n = Node::new(id, tags, content);
            repo.put(&n).unwrap();
        }
        repo.get(&3.into()).unwrap();

        let res = repo.traverse(create_match_tag_filter("tag2")).unwrap();
        assert_eq!(res.len(), 1);

        match &res[0] {
            node::Content::String(res_string) => assert_eq!(res_string, "bbb"),
            _ => panic!("invalid content"),
        };
    }
}
