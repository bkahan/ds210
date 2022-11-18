/*
Ben Kahan
Homework 9
DS210
16 November 2022
Collaborators: none
*/

pub mod tree {
    use std::ops::Deref;
    use crate::readFile;
    use std::ptr;


    struct Node {
        error: (i32, i32),
        left_child: Vec<(i32, i32)>, // todo fix this
        right_child: Vec<(i32, i32)>,
    }

    impl Node {

        fn new_tree() -> Node {
            Node {
                error: (0, 1),
                left_child: Vec::<(i32,i32)>::new(),
                right_child: Vec::<(i32,i32)>::new()
            }
        }

        fn init_tree(tree: Node, path : &str) {

            let mut res = readFile::read_file::file2vectuple(path);
            let data = res.unwrap();

            let mut left = tree.left_child;
            let mut right = tree.right_child;

            for data_point in data.iter() {
                let x = *data_point;

                if x.0 < 0 { // initialize tree to split if x < 0 -> 0, x > 0 -> 1
                    left.push(x)
                } else {
                    right.push(x)
                }
            }
        }

        fn insert_node(tree: Node, data : Vec<(i32, i32)>) {

            /*
            take the tree
            use a determination function to split based on xi <- how do I split the data?
            get vector from that node
            push new data onto that node
            done

             */



            todo!()
        }

        fn determine(data: (i32, i32)) -> bool {


            /*
            build a tree based on a random split, let's say x < 0 -> 0, x > 0 -> 1
            calculate the error
                this is determined by summing the zi for each node, dividing by length
                we want the 0 node to be close to 0 and the 1 node close to 1
            shift the split by +/- n, recalculate error
            iterate m times, storing error each time  | Vector of trees
            return lowest error

             */

            todo!()
        }
    }
}
