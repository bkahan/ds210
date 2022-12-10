/*
Ben Kahan
Final Project
DS210
5 December 2022
Collaborators: none
*/


pub(crate) mod graph {

    #[derive(Debug)]
    pub struct NodeData {
        pub node_index: i16,
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
        adj_list : Vec<Vec<NodeData>>,
    }

    impl Graph  {

        /*
        The real question is wtf do we do here

        connect based on hash of director name




         */



    }

}