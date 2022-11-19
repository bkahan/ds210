/*
Ben Kahan
Homework 9
DS210
16 November 2022
Collaborators: none
*/

/*
take the tree
use a determination function to split based on xi <- how do I split the data?
get vector from that node
push new data onto that node
done

 */

/*
build a tree based on a random split, let's say x < 0 -> 0, x > 0 -> 1
calculate the error
    this is determined by summing the zi for each node, dividing by length
    we want the 0 node to be close to 0 and the 1 node close to 1
shift the split by +/- n, recalculate error
iterate m times, storing error each time  | Vector of trees
return lowest error

 */

pub mod tree {
    use crate::readFile;

    pub struct Node {
        error: (i32, i32),
        left_child: Vec<(i32, i32)>, // todo fix this
        right_child: Vec<(i32, i32)>,
    }

    impl Node {
        pub fn new_tree() -> Node {
            Node {
                error: (0, 1),
                left_child: Vec::<(i32, i32)>::new(),
                right_child: Vec::<(i32, i32)>::new(),
            }
        }

        pub fn init_tree(tree: &mut Node, path: &str) {
            let res = readFile::read_file::file2vectuple(path);
            let data = res.unwrap();

            let mut left  = &mut tree.left_child;
            let mut right = &mut tree.right_child;

            for data_point in data.iter() {
                let x = *data_point;

                if x.0 < 0 {
                    // initialize tree to split if x < 0 -> 0, x > 0 -> 1
                    left.push(x)
                } else {
                    right.push(x)
                }
            }
        }

        pub fn recalculate_tree(tree: Node) {
            todo!()

        }

        pub fn calculate_error(tree: &Node) -> (f32, f32) {
            // return error

            let left = &tree.left_child; // try to get to 0
            let right = &tree.right_child; // try to get to 1

            let mut left_sum :f32 = 0.0;
            let mut right_sum : f32 = 0.0;

            for data_point in left.iter() {
                let tmp = *data_point;
                left_sum = left_sum + tmp.1 as f32;
            }

            for data_point in right.iter() {
                let tmp = *data_point;
                right_sum = right_sum + tmp.1 as f32;
            }

            let left_err = 0.0 - (left_sum / left.len() as f32);
            let right_err = 1.0 - (right_sum / left.len() as f32);

            return (left_err, right_err);
        }
    }
}