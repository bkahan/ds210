/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod graph {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;
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

        pub fn bfs(graph: &mut Graph) {
            let all_actors = graph.adj_list.keys();

            let mut actors = Vec::<&String>::new();

            for a in all_actors {
                actors.push(a);
            }

            let num = graph.adj_list.len();

            let mut is_visited: HashMap<String, IsVisited> = HashMap::with_capacity(num);

            for actor in actors {
                if is_visited.get(actor).unwrap().eq(&IsVisited::_NO) {
                    //graph::graph::Graph::bfs_helper(&mut graph.adj_list.get_mut(&*actor).unwrap(), &mut is_visited);
                }
            }
        }

        fn bfs_helper(graph: &mut Vec<&NodeData>, is_visited: &mut HashMap<String, IsVisited>) {
            let mut queue = VecDeque::<&NodeData>::new();

            for node in graph {
                queue.push_front(node);
            }

            while !queue.is_empty() {
                let tmp_node = queue.pop_back().unwrap();

                for actors in &tmp_node.main_actors {
                    let mut tmp_actor = is_visited.get_mut(&*actors).unwrap();
                    tmp_actor = &mut IsVisited::_YES;



                    // for movie_actor in &movie.main_actors {
                    //     queue.push_front(graph.adj_list.get(&*movie_actor).unwrap()); // pushes movies that actors are in
                    //
                    // }
                }
                println!("{}", tmp_node.movie_title);
            }
        }

        pub fn print_graph(graph: &Graph) {
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