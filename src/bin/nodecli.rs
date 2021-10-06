use anyhow::Error;
use nodes::node::{Content, Node, NodeId};
use nodes::repo::{create_accept_all_filter, HashMapRepo, NodeRepo};

fn main() -> Result<(), Error> {
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
        repo.put(&n)?;
    }

    repo.bfs_dump()?;

    let res = repo.traverse(create_accept_all_filter())?;

    for r in res {
        println!("{:?}", r);
    }

    Ok(())
}
