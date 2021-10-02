use anyhow::Error;
use nodes::node::{Content, Node};
use nodes::repo::{HashMapRepo, NodeRepo};

fn main() -> Result<(), Error> {
    let mut repo = HashMapRepo::new();

    let root_id = repo.root();
    let mut id = root_id.into();

    let mut root = Node::new(id, None);
    repo.put(&root)?;

    id += 1;
    let s = Node::new(id, Some(Content::String("aaa".to_string())));
    repo.put(&s)?;

    id += 1;
    let mut e = Node::new(id, None);
    e.edges = vec![s.id];
    repo.put(&e)?;

    root.edges.push(e.id);
    repo.put(&root)?;

    Ok(())
}
