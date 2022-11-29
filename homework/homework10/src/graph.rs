/*
Ben Kahan
Homework 10
DS210
20 November 2022
Collaborators: none
*/

pub mod graph {
    use std::fmt;
    use std::fmt::Formatter;
    use rand::Rng;
    use crate::graph::graph;
    use crate::read_file::read_file;

    pub struct Graph {
        // code adapted from lec27, attempted to make it as "mine" as possible
        vert_count: i16,
        adj_list: Vec<Vec<usize>>,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct VertexData(i16, f32, i16);

    //     vertex_id : i16,
    //     page_rank: f32,
    //     times_visited: i16,

    impl fmt::Display for VertexData {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Vertex #{}: PageRank: {}, times_visited: {}", self.0, self.1, self.2)
        }
    }

    impl Graph {
        pub fn new_graph(num_vertex: i16) -> Graph {
            return Graph {
                vert_count: num_vertex,
                adj_list: vec![Vec::<usize>::new(); num_vertex as usize],
            };
        }

        pub fn insert_data(data: &mut Vec<(usize, usize)>, graph: &mut Graph) {
            data.remove(0); // removing the created tuple that defines the number of vertices
            for (_source, _target) in data {
                graph.adj_list[*_source].push(*_target);
            }
        }

        fn pagerank_calculate(graph: &Graph, vertex_data_list: &mut Vec<Vec<VertexData>>) {
            let s: f32 = graph.vert_count as f32;
            for vertex in vertex_data_list {
                for vert in vertex {
                    vert.1 = vert.2 as f32 / (s * 100.0);
                }
            }
        }

        fn pagerank_helper(result: &mut Vec<Vec<VertexData>>, index: usize) {
            match result[index].len() {
                0 => {
                    result.get_mut(index).unwrap().push(VertexData(index as i16, 0.0, 1));
                }
                1 => {
                    let mut tmp = result[index].pop().unwrap(); // this is so stupid there must be a better way
                    tmp.2 = tmp.2 + 1;
                    result[index].push(tmp);
                }
                _ => {
                    panic!("WTF")
                }
            }
        }

        pub fn pagerank(original_graph: &Graph) -> Vec<VertexData> { // this should be done recursively, this is shit code :(

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
            graph::Graph::pagerank_calculate(original_graph, &mut res);

            return graph::Graph::sort_by_pagerank(&mut res);
        }

        fn sort_by_pagerank(vector: &mut Vec<Vec<VertexData>>) -> Vec<VertexData> {
            let mut res = Vec::<VertexData>::new();

            for vec in vector {
                res.push(vec.pop().unwrap());
            }

            res.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            res.reverse();

            return res;
        }

        pub fn top_five(vector: &Vec<VertexData>) -> Vec<VertexData> {
            let top = vector[0..5].to_vec();
            return top;
        }
    }

    #[test]
    fn sum_pagerank_test() {
        let path = "/Users/benkahan/Documents/School/ds210/homework/homework10/src/test.txt";
        //let path = "test.txt";
        let a = read_file::file2vectuple(path);
        let mut ans = a.unwrap();
        let num_verts = ans.get(0).unwrap();
        let mut graph = graph::graph::Graph::new_graph(num_verts.0 as i16);

        graph::graph::Graph::insert_data(&mut ans, &mut graph);

        let vector = graph::graph::Graph::pagerank(&graph);

        let mut sum: f32 = 0.0;

        for v in vector {
            sum += v.1;
        }
        println!("{}", sum);

        let range = 0.9..1.0;
        assert!(range.contains(&sum));
    }

    #[test]
    fn sum_pagerank_test2() {
        let path = "/Users/benkahan/Documents/School/ds210/homework/homework10/src/test2.txt";
        //let path = "test2.txt";
        let a = read_file::file2vectuple(path);
        let mut ans = a.unwrap();
        let num_verts = ans.get(0).unwrap();
        let mut graph = graph::graph::Graph::new_graph(num_verts.0 as i16);

        graph::graph::Graph::insert_data(&mut ans, &mut graph);

        let vector = graph::graph::Graph::pagerank(&graph);

        let mut sum: f32 = 0.0;

        for v in vector {
            sum += v.1;
        }
        println!("{}", sum);

        let range = 0.9..1.0;
        assert!(range.contains(&sum));
    }
}