/*
Ben Kahan
Homework 7
DS210
2 November 2022
Collaborators: none
 */

mod q1 ;

fn main() {
    let tri1 = vec![3.0, 4.0, 5.0];
    let s1 = q1::new_shape(tri1, 0.0);

    println!("{}", q1::Shape::sum_sides(&s1));
    println!("{}", q1::Shape::area(&s1));
}
