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
        pub node_id : i16,
        pub movie_title: String,
        pub year: i16,
        pub director: String,
        pub main_actors: Vec<&'static str>,
        pub rating: f32,
        pub total_gross: f32,
        pub genres: (String, String),
    }


    impl NodeData {

        fn insert_data() {
            todo!()
        }

    }
}