/*
Ben Kahan
Homework 9
DS210
16 November 2022
Collaborators: none
*/

/*
take the tree
use a determination function to split based on xi <- calculate gini on left, gini on right, go with lower value
get vector from that node
push new data onto that node
done

build a tree based on a random split, let's say x < 0 -> 0, x > 0 -> 1
calculate the error
    this is determined by summing the zi for each node, dividing by length
    we want the 0 node to be close to 0 and the 1 node close to 1
shift the split by +/- n, recalculate error
iterate m times, storing error each time  | Vector of trees
return lowest error

 */

pub mod tree {
    use crate::read_file;
    use crate::tree::tree;

    pub struct Node {
        error: (f32, f32),
        split_point: i32,
        left_child: Vec<(i32, i32)>,
        right_child: Vec<(i32, i32)>,
    }

    impl Node {
        pub fn get_error(node: &Node) -> (f32, f32) {
            return node.error;
        }

        pub fn new_tree() -> Node {
            Node {
                error: (0.0, 1.0),
                split_point: 0,
                left_child: Vec::<(i32, i32)>::new(),
                right_child: Vec::<(i32, i32)>::new(),
            }
        }

        fn insert(tree: &mut Node, data: &Vec<(i32, i32)>, split_point: i32) {
            let left = &mut tree.left_child;
            let right = &mut tree.right_child;

            for data_point in data.iter() {
                let x = *data_point;

                if x.0 < split_point {
                    // initialize tree to split if x < 0 -> 0, x > 0 -> 1
                    left.push(x)
                } else {
                    right.push(x)
                }
            }
        }

        pub fn init_tree(tree: &mut Node, path: &str) {
            let res = read_file::read_file::file2vectuple(path);
            let data = res.unwrap();

            tree::Node::insert(tree, &data, tree.split_point);
            tree::Node::calculate_error(tree);
        }

        pub fn calculate_error(tree: &mut Node) -> (f32, f32) {
            // return error

            let left = &tree.left_child; // try to get to 0
            let right = &tree.right_child; // try to get to 1

            let mut left_sum: f32 = 0.0;
            let mut right_sum: f32 = 0.0;

            for data_point in left.iter() {
                let tmp = *data_point;
                left_sum = left_sum + tmp.1 as f32;
            }

            for data_point in right.iter() {
                let tmp = *data_point;
                right_sum = right_sum + tmp.1 as f32;
            }

            let left_err = 1.0 - (left_sum / left.len() as f32);
            let right_err = 1.0 - (right_sum / left.len() as f32);

            tree.error = (left_err, right_err);

            return (left_err, right_err);
        }

        pub fn iterate_tree(tree: &mut Node, num_iters: i32) -> Vec<Node> {
            let mut data = Vec::<(i32, i32)>::new();
            let mut result = Vec::<Node>::new();

            for tuple in &tree.left_child {
                data.push(*tuple)
            }

            for tuple in &tree.right_child {
                data.push(*tuple)
            }

            for iter in 0..num_iters {
                let mut tmp_tree = tree::Node::new_tree();
                tree::Node::insert(&mut tmp_tree, &data, iter);
                tree::Node::calculate_error(&mut tmp_tree);
                tmp_tree.split_point = iter;
                result.push(tmp_tree)
            }

            for iter in 0..num_iters {
                // https://stackoverflow.com/questions/69170796/rust-negative-for-loops
                let mut tmp_tree = tree::Node::new_tree();
                tree::Node::insert(&mut tmp_tree, &data, -iter);
                tree::Node::calculate_error(&mut tmp_tree);
                tmp_tree.split_point = iter;
                result.push(tmp_tree)
            }

            return result;
        }
        pub fn find_best_tree_param(tree_vec: &mut Vec<Node>) -> (f32, f32, i32) {
            let init_node = tree_vec.pop().unwrap();
            let mut error = tree::tree::Node::get_error(&init_node);

            let mut lowest = Vec::<(f32, f32)>::new();
            let mut final_tree = Vec::<&Node>::new();

            for tree in tree_vec.iter() {
                let curr_tree_err = tree::tree::Node::get_error(tree);

                if curr_tree_err < error {
                    error = curr_tree_err;
                    lowest.push(error);
                    final_tree.push(tree);
                }
            }
            let tmp = final_tree.pop().unwrap();

            return (tmp.error.0, tmp.error.1, tmp.split_point);
        }
    }
}
