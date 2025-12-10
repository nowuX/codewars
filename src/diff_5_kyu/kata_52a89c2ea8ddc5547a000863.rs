#![expect(unused)]

struct Node {
    next_node: Option<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node { next_node: None }
    }

    pub fn next(&self) -> Option<&Self> {
        self.next_node.as_deref()
    }
}

fn loop_size(node: Node) -> usize {
    todo!("Your code here!")
}

#[cfg(test)]
mod sample_tests {
    use super::{Node, loop_size};

    // fn assert_loop_size(tail_size: usize, loop_size: usize) {
    //     assert_eq!(
    //         todo!();
    //         super::loop_size(Node::gen_cycle(tail_size, loop_size)),
    //         loop_size
    //     );
    // }

    // #[test]
    // fn four_nodes_with_a_loop_of_3() {
    //     assert_loop_size(1, 3);
    // }

    // #[test]
    // fn no_tail_and_a_loop_of_4() {
    //     assert_loop_size(0, 4);
    // }

    // #[test]
    // fn tiny_loop() {
    //     assert_loop_size(3, 1);
    // }

    // #[test]
    // fn single_node() {
    //     assert_loop_size(0, 1);
    // }
}
