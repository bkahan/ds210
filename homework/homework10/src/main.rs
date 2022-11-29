/*
Ben Kahan
Homework 9
DS210
20 November 2022
Collaborators: none
*/

mod read_file;
mod graph;

fn main() {

    let path = "/Users/benkahan/Documents/School/ds210/homework/homework10/src/pagerank_data.txt";
    //let path = "pagerank_data.txt";
    let a = read_file::read_file::file2vectuple(path);
    let mut ans = a.unwrap();
    let num_verts = ans.get(0).unwrap() ;
    let mut graph = graph::graph::Graph::new_graph(num_verts.0 as i16);

    graph::graph::Graph::insert_data(&mut ans, &mut graph);

    let praying = graph::graph::Graph::pagerank(& graph);

    println!("Top 5 Vertices Are: ");
    let top = graph::graph::Graph::top_five(&praying);
    for vertex in &top  {
        println!("{}", vertex);
    }
}

/*

    If v has no outgoing edges, jump to a uniformly random vertex in the entire
    graph
    If v has at least one outgoing edge:
        With probability 9/10, select uniformly at random one of them and follow it. <- use rand::distributions::Bernoulli to do this
        Otherwise, jump to a vertex selected uniformly at random from the entire graph


    pagerank of vertex v := (# walks that terminate at v) / 100 * (num of v)
    check and verify: sum of (pagerank of v_i) = 1 <- test function

    vertex_data_structs:
    - vertex id : float
    - page_rank : float
    - times visited: int

    page_rank_traverse(graph) returns vector of vertex_data_structs

    set res : vector<vertex_data_structs> => new vector

    set

    for vec in vector in vector<vertex>:
        push new vertex_data_struct with:
            vertex id = vec id
            page rank = 0
            times visited = 0

    for _ in 0..100:
        for v in vertex in graph:
            set m : vec<vertex> to be the immediate neighbors of v
            if m is empty:
                randomly pick vertex in graph
            if m.len() >= 1:
                with prob = .9 => randomly pick vertex w from m
                                    get id of m, index res[id of m], increment times visited
                otherwise => randomly pick vertex q in graph
                                    get id of q, index res[id of q], increment times visited


    page_rank_calculate(vertex_data_structs):
        set s to # of nodes in graph
        for ver in vertex_data_structs:
            set ver.page_rank to (ver.times visited / s ) : float

 */