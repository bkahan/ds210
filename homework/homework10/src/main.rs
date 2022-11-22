/*
Ben Kahan
Homework 9
DS210
20 November 2022
Collaborators: none
*/

mod read_file;
mod graph;

use rand::prelude::*;


fn main() {

    let path = "/Users/benkahan/Documents/School/ds210/homework/homework10/src/pagerank_data.txt";
    let a = read_file::read_file::file2vectuple(path);
    let ans = a.unwrap();

    let mut test = Vec::<u16>::new();
    println!("{}", test.len());

    test.push(5);
    test.pop();
    println!("{}", test.len());
    test.push(5);
    println!("{}", test.len());




    println!("Hello, world!");
}
