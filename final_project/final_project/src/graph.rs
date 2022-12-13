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
                println!("Index: {}\n", node.node_index);
                graph.adj_list[node.node_index].push_front(&node);
            }
        }

        pub fn bfs<'c>(graph: &'a mut Graph<'a>) {


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
                 7          if v is the goal then
                 8              return v
                 9          for all edges from v to w in G.adjacentEdges(v) do
                10              if w is not labeled as explored then
                11                  label w as explored
                12                  w.parent := v
                13                  Q.enqueue(w)




             */



        }
    }

}