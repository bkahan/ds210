/*
Ben Kahan
Homework 10
DS210
20 November 2022
Collaborators: none
*/

pub mod graph {
    pub struct Graph {
        // code adapted from lec27, attempted to make it as "mine" as possible
        vert_count: i16,
        adj_list: Vec<Vec<usize>>,
        adj_matrix: Vec<Vec<bool>>,
        /*
        standard graph rep
        1D array of vec references
        indexed by number (no need for fancy label stuff)
         */
    }

    impl Graph {
        pub fn new_graph(num_vertex: i16) -> Graph {
            let mut list_tmp: Vec<Vec<usize>> = Vec::new();
            let mut mat_tmp: Vec<Vec<bool>> = Vec::new();

            for _ in 0..=num_vertex {
                let tmp = Vec::<usize>::new();
                let tmp_mat = Vec::<bool>::new();
                list_tmp.push(tmp);
                mat_tmp.push(tmp_mat);
            }

            return Graph { vert_count: num_vertex, adj_list: list_tmp, adj_matrix: mat_tmp };
        }

        pub fn insert_dedge(dedge: &Vec<(usize, usize)>, mut graph: Graph) {

            for (_source, _target) in dedge {
                graph.adj_list[*_source].push(*_target);
                graph.adj_matrix[*_source].push(true);

            }

            /*
            takes in a tuple (edge1, edge2) and a vertex
            inserts by going to index of edge1 and distance to edge2
            return (optional) true <- look into this
             */
        }

        pub fn traverse() {
            todo!()

            /*
            takes a graph
            walk the graph
             */
        }

        pub fn pagerank() {
            todo!()

            /*
            todo: not sure what to do here
             */
        }
    }
}