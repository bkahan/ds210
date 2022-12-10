/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

mod read_file;
mod graph;

fn main() {

    let path = "/Users/benkahan/Documents/School/ds210/final_project/final_project/data/data.csv";
    //let path = "../data/data.csv";

    let result = read_file::read_csv::file2node(path);

    result.unwrap();




    println!("Hello, world!");
}

/*

proj:

pt1: parse data
read csv, init graph (based on what??)
insert data into graph

pt2: analyze data
what graph analysis can be done on this graph?
    how I choose for things to be connected matters

q: what does each node hold? -> struct of data
q: how are nodes connected?
q: what defines connected?

about graph:
- non directional
- weighted by....

Connect graph by:
- director?
- main actors?
-





 */
