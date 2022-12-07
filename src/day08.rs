#![feature(test)]

extern crate test;
const _INPUT: &str = include_str!("../inputs/input08.txt");

#[derive(Debug)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

#[derive(Debug)]
struct NodeZipper<T> {
    node: Node<T>,
    parent: Option<Box<NodeZipper<T>>>,
    index_in_parent: usize,
}

impl<T> NodeZipper<T> {
    fn child(mut self, index: usize) -> NodeZipper<T> {
        // Remove the specified child from the node's children.
        // A NodeZipper shouldn't let its users inspect its parent,
        // since we mutate the parents
        // to move the focused nodes out of their list of children.
        // We use swap_remove() for efficiency.
        let child = self.node.children.swap_remove(index);

        // Return a new NodeZipper focused on the specified child.
        NodeZipper {
            node: child,
            parent: Some(Box::new(self)),
            index_in_parent: index,
        }
    }

    fn parent(self) -> NodeZipper<T> {
        // Destructure this NodeZipper
        let NodeZipper {
            node,
            parent,
            index_in_parent,
        } = self;

        // Destructure the parent NodeZipper
        let NodeZipper {
            node: mut parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        } = *parent.unwrap();

        // Insert the node of this NodeZipper back in its parent.
        // Since we used swap_remove() to remove the child,
        // we need to do the opposite of that.
        parent_node.children.push(node);
        let len = parent_node.children.len();
        parent_node.children.swap(index_in_parent, len - 1);

        // Return a new NodeZipper focused on the parent.
        NodeZipper {
            node: parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        }
    }

    fn finish(mut self) -> Node<T> {
        while let Some(_) = self.parent {
            self = self.parent();
        }

        self.node
    }
}

impl<T> Node<T> {
    fn zipper(self) -> NodeZipper<T> {
        NodeZipper {
            node: self,
            parent: None,
            index_in_parent: 0,
        }
    }
}

fn part1() -> u32 {
    0
}

fn part2() -> u32 {
    0
}

pub fn main() {
    println!("Part 1: Answer {}", part1());
    println!("Part 2: Answer {} ", part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 0);
    }
    #[test]
    fn part2_test() {
        assert_eq!(part2(), 0);
    }
    #[bench]
    fn part1_bench(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        b.iter(part2);
    }
}
