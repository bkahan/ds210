/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod graph {
    use std::collections::{HashMap, VecDeque};
    use std::ops::{Deref};

    #[derive(Copy, Clone, Default, PartialEq)]
    pub enum IsVisited {
        _YES,
        #[default] _NO,
    }

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct NodeData {
        // each csv line is converted to this
        pub movie_title: String,
        pub year: i16,
        pub director: String,
        pub main_actors: Vec<String>,
        pub genres: (String, String),
    }

    pub struct Graph<'a> {
        adj_list: HashMap<String, Vec<&'a NodeData>>,
    }

    impl<'a> Graph<'a> {
        pub fn new_graph<'b>(data: &Vec<NodeData>) -> Graph<'a> {
            let mut tmp: HashMap<String, Vec<&'a NodeData>> = HashMap::new();

            for node in data {
                for actor in &node.main_actors {
                    tmp.insert(actor.deref().parse().unwrap(), Vec::<&'a NodeData>::new());
                }
            }

            return Graph {
                adj_list: tmp
            };
        }

        pub fn insert_data<'b>(graph: &mut Graph<'a>, data: &'a mut Vec<NodeData>) {
            for node in data {
                for actor in &node.main_actors {
                    let adj_list = graph.adj_list.get_mut(actor).unwrap();
                    adj_list.push(node);
                }
            }
        }

        pub fn bfs(graph: &Graph) {
            // let num_verts = graph.adj_list.len() ;
            //
            // let mut queue = VecDeque::<&LinkedList<&NodeData>>::new();
            //
            // let mut is_visited : Vec<IsVisited> = vec![Default::default() ; num_verts];
            //
            // for (key, value) in graph.adj_list.iter() {
            //     println!("Actor: {}", key);
            //     for movie in value {
            //         println!("Movie Title: {}", movie.movie_title)
            //     }
            //     println!();
            // }

            // for vert in 0..num_verts {
            //
            //     if *is_visited.get(vert).unwrap() == IsVisited::_NO {
            //
            //         queue.push_front(&graph.adj_list[vert]);
            //         is_visited[vert] = IsVisited::_YES;q
            //
            //         while !queue.is_empty() {
            //             let mut tmp = queue.pop_back().unwrap();
            //
            //             for node in tmp {
            //                 println!("Movie Title: {}", node.movie_title)
            //             }
            //             println!();
            //
            //         }
            //     }
            //
            // }

            /*

            from wikipedia: https://en.wikipedia.org/wiki/Breadth-first_search
            using to my implementation

                Input: A graph G and a starting vertex root of G

                Output: Goal state. The parent links trace the shortest path back to root[8]

                 1  procedure BFS(G, root) is
                 2      let Q be a queue
                 3      label root as explored
                 4      Q.enqueue(root)
                 5      while Q is not empty do
                 6          v := Q.dequeue()
                 9          for all edges from v to w in G.adjacentEdges(v) do
                10              if w is not labeled as explored then
                11                  label w as explored
                12                  w.parent := v
                13                  Q.enqueue(w)

             */
        }

        pub fn print_graph(graph: &Graph) { // todo: need to fix the data to graph issue (ex not a graph)

            for (key, value) in graph.adj_list.iter() {
                println!("Actor: {}", key);
                for movie in value {
                    println!("Movie Title: {}", movie.movie_title)
                }
                println!();
            }
        }
    }
}