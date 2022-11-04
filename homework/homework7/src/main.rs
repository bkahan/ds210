/*
Ben Kahan
Homework 7
DS210
2 November 2022
Collaborators: none
 */

extern crate core;

use crate::homework_code::Polygon;

mod homework_code;

fn main() {

    // q1
    let tri1 = vec![3.0, 4.0, 5.0];
    let mut s1 = homework_code::Shape::new_polygon_q1(tri1, 0.0, false);

    println!("{}", homework_code::Shape::sum_sides(&s1));
    println!("{}", homework_code::Shape::poly_area(&s1));

    homework_code::Shape::poly_double_size(&mut s1);
    println!("{}", homework_code::Shape::poly_area(&s1));
    println!("{}", homework_code::Shape::sum_sides(&s1));

    //let mut s2 = q1::new_shape(vec![1.0,2.0], 1.0); // will not let you run the code if it is not a shape
    // let mut s3 = q1::new_shape(vec![-1.0,2.0,4.0], 1.0); // negative side lengths
    //let mut s4 = q1::new_shape(vec![1.0,1.0,4.0], 1.0); // not a triangle

    // q2

    let q2_6 = homework_code::Shape::new_polygon_q2(2.0, 6, true);

    let q2_12 = homework_code::Shape::new_polygon_q2(2.0, 12, true);
    let q2_24 = homework_code::Shape::new_polygon_q2(2.0, 24, true);
    let q2_128 = homework_code::Shape::new_polygon_q2(2.0, 128, true);
    let q2_256 = homework_code::Shape::new_polygon_q2(2.0, 256, true);
    let q2_512 = homework_code::Shape::new_polygon_q2(2.0, 512, true);
    let q2_1024 = homework_code::Shape::new_polygon_q2(2.0, 1024, true);
    let q2_2048 = homework_code::Shape::new_polygon_q2(2.0, 2048, true);
    let q2_65536 = homework_code::Shape::new_polygon_q2(2.0, 65536, true);

    println!("6-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_6));
    println!("12-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_12));
    println!("24-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_24));
    println!("128-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_128));
    println!("256-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_256));
    println!("512-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_512));
    println!("1024-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_1024));
    println!("2048-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_2048));
    println!("65536-sided polygon with area: {}", homework_code::Shape::poly_area(&q2_65536));

}