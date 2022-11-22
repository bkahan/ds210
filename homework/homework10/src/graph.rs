/*
Ben Kahan
Homework 10
DS210
20 November 2022
Collaborators: none
*/

use rand::Rng;

pub mod graph {
    use rand::Rng;

    pub struct Graph {
        // code adapted from lec27, attempted to make it as "mine" as possible
        vert_count: i16,
        adj_list: Vec<Vec<usize>>,
        adj_matrix: Vec<Vec<bool>>,
    }

    pub struct VertexData {
        vertex_id : i16,
        page_rank: f32,
        times_visited: i16,
    }

    impl Graph {
        pub fn new_graph(num_vertex: i16) -> Graph {
            let mut list_tmp: Vec<Vec<usize>> = Vec::new();
            let mut mat_tmp: Vec<Vec<bool>> = Vec::new();

            for _ in 0..=num_vertex {
                let tmp = Vec::<usize>::new();
                let tmp_mat = Vec::<bool>::new();
                list_tmp.push(tmp);
                mat_tmp.push(tmp_mat);
            }

            return Graph { vert_count: num_vertex, adj_list: list_tmp, adj_matrix: mat_tmp };
        }

        pub fn get_vert_count(graph : Graph) -> i16 {
            graph.vert_count
        }

        pub fn insert_data(data: &Vec<(usize, usize)>, mut graph: Graph) {
            for (_source, _target) in data {
                graph.adj_list[*_source].push(*_target);
                graph.adj_matrix[*_source].push(true);
            }
        }

        pub fn traverse() { // todo: not actually sure if I need this
            todo!()

            /*
            takes a graph
            walk the graph
             */
        }

        pub fn pagerank(mut graph : Graph ) -> Vec<Vec<VertexData>> {

            let mut res : Vec<Vec<VertexData>> = Vec::new();

            for vertex in graph.adj_list {
                if vertex.len() == 0  {
                    let mut rand = rand::thread_rng();
                    let index : usize = rand.gen_range(0..graph.vert_count) as usize;

                    match res[index].len() {
                        1 => {
                            res[index].get(0).
                        }
                        _ => {}
                    }

                    res[index].push(VertexData {
                        vertex_id : index as i16,
                        page_rank: 0.0,
                        times_visited: 0
                    })




                    //let rand_vert = rand::
                }
            }
            todo!()

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

            /*
            todo: not sure what to do here
             */
        }
    }
}