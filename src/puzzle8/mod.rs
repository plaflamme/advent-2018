use std::str::FromStr;

fn parse(input: String) -> Vec<u32> {
    input.split_ascii_whitespace().map(|x| u32::from_str(x).expect(format!("invalid number {}", x).as_str())).collect()
}
pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle8 { nodes: parse(input) })
}

struct Puzzle8 {
    nodes: Vec<u32>
}

struct Node {
    metadata: Vec<u32>,
    children: Vec<Node>
}

impl Node {

    fn new(flat: &mut Vec<u32>) -> Node {
        flat.reverse();
        Node::mk_tree(flat)
    }

    fn mk_tree(flat: &mut Vec<u32>) -> Node {
        let n_children = flat.pop().expect("missing number of children");
        let n_meta = flat.pop().expect("missing number of metadata");

        let mut children = Vec::new();
        for _ in 0..n_children {
            children.push(Node::mk_tree(flat));
        }
        let mut metadata = Vec::new();
        for _ in 0..n_meta {
            let v = flat.pop().expect("missing metadata");
            metadata.push(v);
        }

        Node { metadata, children }
    }

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter()
                .map(|m| {
                    let m_size = *m as usize;
                    if m_size == 0 || m_size > self.children.len() {
                        0
                    } else {
                        self.children[m_size - 1].value()
                    }
                })
                .sum()
        }
    }

    fn iter(&self) -> NodeIterator {
        let a = std::iter::once(self);
        let more = self.children.iter().flat_map(|x| x.iter());
        NodeIterator { iter: Box::new(a.chain(more)) }
    }
}

// https://amos.me/blog/2019/recursive-iterators-rust/
struct NodeIterator<'a> {
    iter: Box<dyn Iterator<Item = &'a Node> + 'a>
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl crate::Puzzle for Puzzle8 {
    fn part1(&self) -> String {
        let root = Node::new(&mut self.nodes.clone());

        let sum: u32 = root.iter().flat_map(|x| x.metadata.iter()).sum();
        sum.to_string()
    }

    fn part2(&self) -> String {
        let root = Node::new(&mut self.nodes.clone());
        root.value().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;

    fn example_input() -> Puzzle8 {
        Puzzle8 { nodes: vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2] }
    }

    #[test]
    fn test_part1() {
        assert_eq!(example_input().part1(), "138");
    }

    #[test]
    fn test_part2() {
        assert_eq!(example_input().part2(), "66");
    }
}
