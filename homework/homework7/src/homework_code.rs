/*
Ben Kahan
Homework 7
DS210
2 November 2022
Collaborators: none
 */

use std::f64::consts::PI;

pub(crate) struct Shape {
    sides: Vec<f64>,
    radius: f64,
    is_standard_polygon: bool,
}

pub(crate) trait Polygon {
    // question 2 implementation
    fn new_polygon_q1(sides: Vec<f64>, radius: f64, is_standard_polygon: bool) -> Shape;
    fn poly_area(shape: &Shape) -> f64;
    fn poly_perimeter(shape: &Shape) -> f64;
    fn poly_radius(shape: &Shape) -> f64;
    fn poly_double_size(shape: &mut Shape);
    fn sum_sides(shape: &Shape) -> f64;
    fn new_polygon_q2(side_length: f64, num_sides: u64, is_standard_polygon: bool) -> Shape;
}

impl Polygon for Shape {
    fn new_polygon_q1(s: Vec<f64>, r: f64, is_standard_polygon: bool) -> Shape {
        match check_shape(&s) {
            true => Shape { sides: s, radius: r, is_standard_polygon },
            false => Shape { sides: vec![0.0], radius: 0.0, is_standard_polygon } // return "empty" shape
        }
    }

    fn poly_area(shape: &Shape) -> f64 {
        if shape.radius != 0.0 {
            return f64::powf(shape.radius, 2.0) * PI;
        }
        match shape.is_standard_polygon {
            false => {
                match shape.sides.len() {
                    2 => return f64::powf(shape.sides[0], 2.0),
                    3 => {
                        let s: f64 = Self::sum_sides(shape) / 2.0;
                        return f64::sqrt(s * (s - shape.sides[0]) * (s - shape.sides[1]) * (s - shape.sides[2]));
                    }
                    4 => shape.sides[0] * shape.sides[1],
                    _ => panic!("Can't calculate anything higher than a rectangle. Use new_polygon_q2.")
                }
            }
            true => {
                let n = shape.sides.len();
                let s = shape.sides[0];
                let omega = PI / n as f64;
                let tan_omega = omega.tan();
                n as f64 * f64::powf(s, 2.0) * (0.25) * (1.0 / tan_omega)
            }
        }
    }

    fn poly_perimeter(shape: &Shape) -> f64 {
        if shape.radius != 0.0 {
            2.0 * PI * shape.radius
        } else {
            match shape.sides.len() {
                2 => 4.0 * shape.sides[0],  // unique case of rectangle
                _ => Self::sum_sides(shape),
            }
        }
    }

    fn poly_radius(shape: &Shape) -> f64 {
        return if shape.radius != 0.0 {
            let s: f64 = shape.sides[0];
            let n: f64 = shape.sides.len() as f64;
            let m = 180.0 / n;
            s / (2.0 * m.sin())
        } else {
            shape.radius
        }
    }

    fn poly_double_size(shape: &mut Shape) { // defining doubling as doubling each length of side ex. [2,6,10] -> [4,12,20]
        for x in 0..shape.sides.len() {
            shape.sides[x] = shape.sides[x] * 2.0;
        }
    }

    fn sum_sides(shape: &Shape) -> f64 {
        let mut tmp: f64 = 0.0;
        for side in &shape.sides {
            tmp += side;
        }
        tmp
    }

    fn new_polygon_q2(side_length: f64, num_sides: u64, is_standard_polygon: bool) -> Shape {
        if num_sides < 3 || side_length < 0.0 { // quick check for shape validity
            panic!("Not a shape.")
        }
        let mut tmp: Vec<f64> = Vec::new();
        for _ in 0..=num_sides {
            tmp.push(side_length) // make a shape with n-side lengths, I know, this is bad
        }
        return Shape::new_polygon_q1(tmp, 0.0, is_standard_polygon);
    }
}

fn check_shape(sides: &Vec<f64>) -> bool {
    let a: f64 = sides.iter().sum();
    if sides.len() < 3 || a < 0.0 {
        panic!("Not a shape.")
    }
    for x in sides {
        if *x < 0.0 {
            panic!("Negative Side Lengths")
        }
    }
    if sides.len() == 3 {
        if sides[0] + sides[1] <= sides[2] {
            panic!("Not a triangle.")
        }
    }
    true
}

#[test]
#[should_panic]
fn invalid_shape1() { // will not let you run the code if it is not a shape
    Shape::new_polygon_q1(vec![0.0,-2.0], -100.0, true);
}

#[test]
#[should_panic]
fn invalid_shape2() { // negative side lengths
    Shape::new_polygon_q1(vec![-1.0,2.0,4.0], 1.0, true);
}

#[test]
#[should_panic]
fn invalid_shape3() { // not a triangle
    Shape::new_polygon_q1(vec![1.0,1.0,4.0], 1.0, true);
}

#[test]
#[should_panic]
fn invalid_shape4() { // not a triangle
    Shape::new_polygon_q2(-1.0, 0, true);
}