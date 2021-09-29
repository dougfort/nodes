fn main() {
    let mut id = 0;

    id += 1;
    let s = nodes::Node::new_string(id, "aaa");

    id += 1;
    let e = nodes::Node::new_edges(id, vec![s]);

    let n = nodes::traverse(&e);
    println!("{:?}", n);
}
