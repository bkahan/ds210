/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/


pub(crate) mod graph {
    use std::collections::LinkedList;

    #[derive(Debug)]
    pub struct NodeData {
        pub node_index: usize,
        pub node_id : i16,
        pub movie_title: String,
        pub year: i16,
        pub director: String,
        pub main_actors: Vec<String>,
        pub rating: f32,
        pub total_gross: f32,
        pub genres: (String, String),
    }

    pub struct Graph {
        adj_list : LinkedList<LinkedList<NodeData>>,
    }

    impl Graph  {

        pub fn new_graph(num_verts : i16) -> Graph {
            let mut tmp_list = LinkedList::<LinkedList<NodeData>>::new();

            for x in 0..num_verts {
                tmp_list.push_front(LinkedList::<NodeData>::new())
            }

            return Graph {
                adj_list: tmp_list
            };
        }

        /*
        The real question is wtf do we do here

        connect based on hash of director name




         */



    }

}