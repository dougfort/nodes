//! nodes: a graph based data organization
//! 

#[derive(Debug)]
pub struct Node{
    pub edges: std::vec::Vec<Node>,
}

impl Node {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = 2 +2;
        assert_eq!(x, 4);
    }
}
