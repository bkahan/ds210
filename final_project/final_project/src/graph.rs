/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/

pub(crate) mod graph {
    use std::collections::LinkedList;

    #[derive(Clone)]
    pub struct NodeData {
        pub node_index: usize,
        pub node_id : usize,
        pub movie_title: String,
        pub year: i16,
        pub director: String,
        pub main_actors: Vec<String>,
        pub rating: f32,
        pub total_gross: f32,
        pub genres: (String, String),
    }

    pub struct Graph<'a> {
        adj_list : Vec<LinkedList<&'a NodeData>>,
    }

    impl<'a> Graph<'a>  {

        pub fn new_graph<'b>(num_verts : i16) -> Graph<'a> {
            let tmp = vec![LinkedList::<&'a NodeData>::new(); num_verts as usize];

            return Graph {
                adj_list: tmp
            };
        }

        pub fn insert_data<'b>(graph: &'a mut Graph<'a>, data: &'a Vec<NodeData>) {

            for node in data {
                //println!("Index: {}\n", node.node_index);
                graph.adj_list[node.node_index].push_front(&node);
            }
        }

        /*
        The real question is wtf do we do here

        connect based on hash of director name




         */



    }

}