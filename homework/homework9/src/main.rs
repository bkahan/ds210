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

mod readFile;
mod tree;

fn main() {
    let path = "/Users/benkahan/Documents/School/ds210/homework/homework9/src/pagerank_data.txt";

    let res = readFile::read_file::file2vectuple(path);
}
