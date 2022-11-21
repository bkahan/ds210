use rand::Rng;
use std::env;
use std::fs::File;
use std::io::prelude::*;

type Vertex = usize;

fn generate_file(path: &str, n: usize) {
    // Generate a random file of edges for vertices 0.999
    let mut file = File::create(path).expect("Unable to create file");
    for i in 0..n {
        // How many neighbors will this node have
        let rng = rand::thread_rng().gen_range(0..20) as usize;
        for _j in 0..rng {
            // Randomly select a neighbor (even with duplicates but not to ourselves)
            let neighbor = rand::thread_rng().gen_range(0..n) as usize;
            if neighbor != i {
                let s: String = format!("{} {}\n", i, neighbor);
                file.write_all(s.as_bytes()).expect("Unable to write file");
            }
        }
    }
}

fn read_file(path: &str) -> Vec<(Vertex, Vertex)> {
    let mut result: Vec<(Vertex, Vertex)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<Vertex>().unwrap();
        let y = v[1].parse::<Vertex>().unwrap();
        result.push((x, y));
    }
    return result;
}

struct Graph {
    // number of vertices
    pub n: usize,

    // adjacency lists
    pub lists: Vec<Vec<Vertex>>,

    // adjacency matrix
    pub matrix: Vec<Vec<bool>>,
}

impl Graph {
    pub fn create(n: usize, edges: &Vec<(Vertex, Vertex)>) -> Graph {
        let mut lists = vec![vec![]; n];
        let mut matrix = vec![vec![false; n]; n];
        for (u, v) in edges {
            lists[*u].push(*v);
            lists[*v].push(*u);
            matrix[*u][*v] = true;
            matrix[*v][*u] = true;
        }
        Graph { n, lists, matrix }
    }

    // counting triangles by enumerating all pairs of neighbors
    // for every vertex (via adjacency lists) and then checking
    // which ones are connected (via adjacency matrix)
    pub fn triangles_mixed(graph: &Graph) -> usize {
        let n = graph.n;
        let mut count = 0;
        let matrix = &(graph.matrix);
        for u in 0..n {
            for v in &graph.lists[u] {
                for w in &graph.lists[u] {
                    if matrix[*v][*w] {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let n = args[2].parse::<Vertex>().unwrap();

    generate_file(file_path, n);

    // Now read the file and put it in a vector of edges
    let edges = read_file(file_path);

    let graph = Graph::create(n, &edges);
    println!();
    println!("triangles: {}", Graph::triangles_mixed(&graph));
    println!();
}
