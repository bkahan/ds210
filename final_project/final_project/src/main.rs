/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

extern crate core;

mod read_file;
mod graph;

fn main() {
    let path = "/Users/benkahan/Documents/School/ds210/final_project/final_project/data/data.csv";
    //let path = "../data/data.csv"; // use for path relative to main.rs

    let csv_data = read_file::read_csv::file2node(path);

    let mut result = csv_data.unwrap();
    let mut graph = graph::graph::Graph::new_graph(&result);

    graph::graph::Graph::insert_data(&mut graph, &mut result);

    graph::graph::Graph::print_graph(&mut graph);

}

