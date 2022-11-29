/*
Ben Kahan
Homework 10
DS210
20 November 2022
Collaborators: none
*/

pub mod graph {
    use rand::Rng;
    use crate::graph::graph;


    pub struct Graph {
        // code adapted from lec27, attempted to make it as "mine" as possible
        vert_count: i16,
        adj_list: Vec<Vec<usize>>,
        adj_matrix: Vec<Vec<bool>>,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct VertexData(i16, f32, i16);

    //     vertex_id : i16,
    //     page_rank: f32,
    //     times_visited: i16,

    impl Graph {
        pub fn new_graph(num_vertex: i16) -> Graph {
            return Graph {
                vert_count: num_vertex,
                adj_list: vec![Vec::<usize>::new() ; num_vertex as usize],
                adj_matrix: vec![Vec::<bool>::new() ; num_vertex as usize]
            };
        }

        pub fn insert_data(data: &mut Vec<(usize, usize)>, graph: &mut Graph) {
            data.remove(0); // removing the created tuple that defines the number of vertices
            for (_source, _target) in data {
                graph.adj_list[*_source].push(*_target);
                graph.adj_matrix[*_source].push(true);
            }
        }

        pub fn get_pagerank(vertex: &VertexData) -> f32 {
            return vertex.1;
        }

        pub fn pagerank_calculate(graph: &Graph, vertex_data_list: &mut Vec<Vec<VertexData>>) {
            let s: f32 = graph.vert_count as f32;

            for vertex in vertex_data_list {
                for vert in vertex {
                    vert.1 = vert.2 as f32 / (s * 100.0) ;
                }
            }
        }


        pub fn pagerank_helper(result: &mut Vec<Vec<VertexData>>, index: usize) {

            print!("index: {}, result[index].len(): {}", index, result[index].len());
            println!("\n");
            println!("len of vec[0] :{}",result.get_mut(0).unwrap().len() );
            println!("\n");

            match result[index].len() {

                0 => {
                    result.get_mut(index).unwrap().push(VertexData(index as i16, 0.0, 1));
                },
                1 => {
                    let mut tmp = result[index].pop().unwrap(); // this is so stupid there must be a better way
                    tmp.2 = tmp.2 + 1;
                    result[index].push(tmp);
                },
                _ => {
                    panic!("WTF")
                }
            }
        }

        pub fn pagerank(original_graph: &Graph) -> Vec<Vec<VertexData>> { // this should be done recursively, this is shit code :(

            let mut res: Vec<Vec<VertexData>> = vec![Vec::<VertexData>::new(); original_graph.vert_count as usize];
            let graph = &original_graph.adj_list;

            for _ in 0..100 {
                for vertex in graph {
                    if vertex.len() == 0 {
                        let index: usize = rand::thread_rng().gen_range(0..original_graph.vert_count) as usize;
                        graph::Graph::pagerank_helper(&mut res, index);
                    } else {
                        let probability: bool = rand::thread_rng().gen_ratio(9, 10);
                        match probability {
                            true => {
                                // select from vertex.len
                                let rand_neighbor = rand::thread_rng().gen_range(0..vertex.len());
                                graph::Graph::pagerank_helper(&mut res, rand_neighbor);
                            }
                            false => {
                                let rand_vert = rand::thread_rng().gen_range(0..original_graph.vert_count) as usize;
                                graph::Graph::pagerank_helper(&mut res, rand_vert);
                            }
                        }
                    }
                }
            }
            return res;

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
        }
    }
}