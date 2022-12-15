/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

extern crate core;

use std::borrow::Borrow;

mod read_file;
mod graph;

fn main() {
    let path = "/Users/benkahan/Documents/School/ds210/final_project/final_project/data/data.csv";
    //let path = "../data/data.csv";

    let csv_data = read_file::read_csv::file2node(path);

    let mut result = csv_data.unwrap();
    let mut graph = graph::graph::Graph::new_graph(&result);

    graph::graph::Graph::insert_data(&mut graph, &mut result);

    graph::graph::Graph::bfs(&mut graph);


}

/*

proj:

pt1: parse data
read csv, init Graph (based on what??)
insert data into Graph

pt2: analyze data
what Graph analysis can be done on this Graph?
    how I choose for things to be connected matters

q: what does each node hold? -> struct of data
q: how are nodes connected?
q: what defines connected?

about Graph:
- non directional
- weighted by....

Connect Graph by:
- director?
- main actors?
-





 */
