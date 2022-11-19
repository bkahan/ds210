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

fn iterate_trees(num_iter : u32, path : &str) -> Vec<Node> {

    let mut result = Vec::<Node>::new();

    let iter_vec : Vec<u32> = (1..num_iter).map(|x| x+1).collect(); // https://stackoverflow.com/questions/48021408/how-to-init-a-rust-vector-with-a-generator-function

    let mut tree = tree::tree::Node::new_tree() ;
    tree::tree::Node::init_tree(&mut tree, path);













    todo!()

}

fn main() {
    let path = "/Users/benkahan/Documents/School/ds210/homework/homework9/src/data.txt";

    let mut treee = tree::tree::Node::new_tree() ;
    tree::tree::Node::init_tree(&mut treee, path);

    let error = tree::tree::Node::calculate_error(&mut treee);





}
