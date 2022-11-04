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

    let q2_6_rad = homework_code::Shape::poly_area(&q2_6);
    let q2_12_rad = homework_code::Shape::poly_area(&q2_12);
    let q2_24_rad = homework_code::Shape::poly_area(&q2_24);
    let q2_128_rad = homework_code::Shape::poly_area(&q2_128);
    let q2_256_rad = homework_code::Shape::poly_area(&q2_256);
    let q2_512_rad = homework_code::Shape::poly_area(&q2_512);
    let q2_1024_rad = homework_code::Shape::poly_area(&q2_1024);
    let q2_2048_rad = homework_code::Shape::poly_area(&q2_2048);
    let q2_65536_rad = homework_code::Shape::poly_area(&q2_65536);

    println!("6-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_6), q2_6_rad);
    println!("12-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_12), q2_12_rad);
    println!("24-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_24), q2_24_rad);
    println!("128-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_128), q2_128_rad);
    println!("256-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_256), q2_256_rad);
    println!("512-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_512), q2_512_rad);
    println!("1024-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_1024), q2_1024_rad);
    println!("2048-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_2048), q2_2048_rad);
    println!("65536-sided polygon with area: {} and a circle with same radius has area: {}", homework_code::Shape::poly_area(&q2_65536), q2_65536_rad);


}