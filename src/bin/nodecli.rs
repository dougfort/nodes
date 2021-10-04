use anyhow::Error;
use nodes::node::{Content, Node};
use nodes::repo::{HashMapRepo, NodeRepo};

fn main() -> Result<(), Error> {
    let mut repo = HashMapRepo::new();

    let root_id = repo.root();
    let mut id = root_id.into();

    id += 1;
    let s = Node::new(id, Content::String("aaa".to_string()));
    repo.put(&s)?;

    id += 1;
    let e = Node::new(id, Content::Edges(vec![s.id]));
    repo.put(&e)?;

    let root = Node::new(root_id.into(), Content::Edges(vec![e.id]));
    repo.put(&root)?;

    Ok(())
}
