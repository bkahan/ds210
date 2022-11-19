/*
Ben Kahan
Homework 9
DS210
16 November 2022
Collaborators: none
*/

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::tree::tree::Node;

mod readFile;
mod tree;


fn main() {
    let path = "/Users/benkahan/Documents/School/ds210/homework/homework9/src/data.txt"; // I needed to use the absolute path for my computer
    //let path = "data.txt";

    let mut treee = tree::tree::Node::new_tree() ;
    tree::tree::Node::init_tree(&mut treee, path);

    let mut tree_vec = tree::tree::Node::iterate_tree(&mut treee, 50);

    let init_node = tree_vec.pop().unwrap();
    let mut error = tree::tree::Node::get_error(&init_node);

    let mut lowest = Vec::<(f32,f32)>::new();
    let mut final_tree = Vec::<&Node>::new();

    for tree in tree_vec.iter() {
        let curr_tree_err = tree::tree::Node::get_error(tree);

         if curr_tree_err < error  {
             error = curr_tree_err ;
             lowest.push(error);
             final_tree.push(tree);
         }
    }

    let final_tree = final_tree.pop().unwrap();
    let split_point = tree::tree::Node::get_split_point(final_tree);
    println!("{}", split_point);





}
