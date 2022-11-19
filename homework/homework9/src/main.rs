/*
Ben Kahan
Homework 9
DS210
16 November 2022
Collaborators: none
*/

mod read_file;
mod tree;

fn main() {
    let path = "/Users/benkahan/Documents/School/ds210/homework/homework9/src/data.txt"; // I needed to use the absolute path for my computer
    //let path = "data.txt";

    let mut treee = tree::tree::Node::new_tree();
    tree::tree::Node::init_tree(&mut treee, path);

    let mut tree_vec = tree::tree::Node::iterate_tree(&mut treee, 75);
    let mut tree_params = tree::tree::Node::find_best_tree_param(&mut tree_vec);

    println!("If x < {}, then: ", tree_params.2);
    println!("Output:");
    println!("     output 0");
    println!("Else: ");
    println!("     output 1");
}
