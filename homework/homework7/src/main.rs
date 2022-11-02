/*
Ben Kahan
Homework 7
DS210
2 November 2022
Collaborators: none
 */

enum ShapeType {
    Rectangle,
    Circle,
    Triangle,
}

struct Shape {
    sides: Vec<u16>,
    radius: u16,
    shape_type: ShapeType
}

impl Shape {
    fn perimeter(shape: Shape) -> u16 {

    }

    fn area (shape: Shape) -> u16 {

    }

}

fn new_shape(s: Vec<u16>, r: u16, t: ShapeType) -> Shape {
    Shape { sides: s, radius: r, shape_type: t }
}


fn main() {
    println!("Hello, world!");
}
