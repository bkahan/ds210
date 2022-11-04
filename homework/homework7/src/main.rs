/*
Ben Kahan
Homework 7
DS210
2 November 2022
Collaborators: none
 */

extern crate core;

use crate::q1::Polygon;

mod q1;
mod q2;

fn main() {

    // q1
    let tri1 = vec![3.0, 4.0, 5.0];
    let mut s1 = q1::Shape::new_polygon_q1(tri1, 0.0, false);

    println!("{}", q1::Shape::sum_sides(&s1));
    println!("{}", q1::Shape::poly_area(&s1));

    q1::Shape::poly_double_size(&mut s1);
    println!("{}", q1::Shape::poly_area(&s1));
    println!("{}", q1::Shape::sum_sides(&s1));


   //let mut s2 = q1::new_shape(vec![1.0,2.0], 1.0); // will not let you run the code if it is not a shape
    // let mut s3 = q1::new_shape(vec![-1.0,2.0,4.0], 1.0); // negative side lengths
    //let mut s4 = q1::new_shape(vec![1.0,1.0,4.0], 1.0); // not a triangle


    // q2



}
