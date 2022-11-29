/*
Ben Kahan
Homework 9
DS210
20 November 2022
Collaborators: none
*/

extern crate core;
mod read_file;
mod graph;




fn main() {

    let path = "/Users/benkahan/Documents/School/ds210/homework/homework10/src/test.txt";
    let a = read_file::read_file::file2vectuple(path);
    let mut ans = a.unwrap();
    let num_verts = ans.get(0).unwrap() ;
    let mut graph = graph::graph::Graph::new_graph(num_verts.0 as i16);

    graph::graph::Graph::insert_data(&mut ans, &mut graph);

    let mut praying = graph::graph::Graph::pagerank(& graph);
    graph::graph::Graph::pagerank_calculate(&graph, &mut praying);
    for vertex in praying  {
        for v in vertex {
            println!("{}", graph::graph::Graph::get_pagerank(&v) );
        }
    }





    println!("Hello, world!");
}
