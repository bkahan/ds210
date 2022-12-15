/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod graph {

    use std::collections::{HashSet, LinkedList, VecDeque};
    use std::ops::{Deref, DerefMut};

    use crate::read_file;
    use crate::read_file::read_csv;

    #[derive(Copy, Clone, Default, PartialEq)]
    pub enum IsVisited {
        _YES,
        #[default] _NO
    }

    #[derive(PartialEq, Eq, Hash, Clone)]
    pub struct NodeData { // each csv line is converted to this
        pub node_index: usize,
        pub node_id : usize,
        pub movie_title: String,
        pub year: i16,
        pub director: String,
        pub main_actors: Vec<String>,
        // pub rating: f32,
        // pub total_gross: f32,
        pub genres: (String, String),
    }

    pub struct Graph<'a> {
        adj_list : Vec<LinkedList<&'a NodeData>>,
    }

    impl<'a> Graph<'a>  {

        pub fn new_graph<'b>(data : &Vec<NodeData>) -> Graph<'a> {

            let mut all_actors = HashSet::<&String>::new();

            for node in data {
                for actor in &node.main_actors {
                    all_actors.insert(actor);
                }
            }

            let tmp = vec![LinkedList::<&'a NodeData>::new(); all_actors.len()]; // this sets up that data as: row length = # of actors total

            return Graph {
                adj_list: tmp
            };
        }

        pub fn insert_data<'b>(graph: &mut Graph<'a>, data: &'a mut Vec<NodeData>) {

            /*
                what do I wanna do here:

                take each movie title, get the index for it
                for each actor in each movie:
                    push index of movie

             */

            for node in data {
                if node.node_index == 0  {
                    node.node_index = node.node_index % graph.adj_list.len(); // calculate node index, should only need to do this once
                }
                for actor in &node.main_actors {

                    let index = read_file::read_csv::calculate_id(actor) % graph.adj_list.len() as i16 ;

                    graph.adj_list[index as usize].push_front(node);

                }




                //println!("Index: {}\n", node.node_index);
                //graph.adj_list[node.node_index].push_front(node);
            }
        }

        pub fn bfs(graph: &Graph) { // todo: need to fix the data to graph issue (ex not a graph)

            let num_verts = graph.adj_list.len() ;

            let mut queue = VecDeque::<&LinkedList<&NodeData>>::new();

            let mut is_visited : Vec<IsVisited> = vec![Default::default() ; num_verts];

            for vert in 0..num_verts {

                if *is_visited.get(vert).unwrap() == IsVisited::_NO {

                    queue.push_front(&graph.adj_list[vert]);
                    is_visited[vert] = IsVisited::_YES;

                    while !queue.is_empty() {
                        let mut tmp = queue.pop_back().unwrap();

                        for node in tmp {
                            println!("Movie Title: {}", node.movie_title)
                        }
                        println!();

                    }
                }

            }





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
    }

}