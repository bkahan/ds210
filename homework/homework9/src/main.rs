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
    let data = fs::read(path).expect("Unable to read file");

    fn read_file_vec(filepath: &str) -> Result<Vec<(u32, u32)>, Box<dyn std::error::Error>> {
        let data = File::open(filepath)?;
        let reader = BufReader::new(data);

        let tmp_res: Vec<(u32, u32)> = Vec::new();

        for line in reader.lines() {
            println!("{}", line.unwrap())
        }

        todo!()
    }

    let res = read_file_vec(path);
}
