/*
Ben Kahan
Homework 7
DS210
2 November 2022
Collaborators: none
 */

use std::f64::consts::PI;

enum ShapeType {
    Rectangle(l : f64, w : f64), 
    Circle,
    Triangle,
}

struct Shape {
    sides: Vec<f64>,
    radius: f64,
    shape_type: ShapeType
}

impl Shape {
    fn perimeter(shape: Shape) -> f64 {
        match shape.shape_type {
            ShapeType::Circle => {
                return 2.0 * shape.radius * PI ;
            }
            _ => Self::sum_sides(shape)
        }
    }

    fn area (shape: Shape) -> f64 {
        match shape.shape_type {
            ShapeType::Circle => {
                return f64::powf(shape.radius, 2.0) * PI ;
            }
            ShapeType::Triangle => { // Heron's Formula
                let mut s : f64 = Self::sum_sides(shape);
                s = s / 2.0 ;
                return f64::sqrt(s * (s - &shape.sides[0]) * (s - &shape.sides[1]) * (s - &shape.sides[2]));
            }
            _ => Self::sum_sides(shape)
        }
    }

    fn sum_sides(shape : Shape) -> f64 {
        let mut tmp: f64 = 0.0;
        for side in shape.sides {
            tmp += &side;
        }
        return tmp;
    }

    fn doubleSize(shape: Shape) {

    }

}

fn new_shape(s: Vec<f64>, r: f64, t: ShapeType) -> Shape {
    Shape { sides: s, radius: r, shape_type: t }
}


fn main() {
    println!("Hello, world!");
}
