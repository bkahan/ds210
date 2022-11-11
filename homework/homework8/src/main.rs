/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

use crate::point::point::{Point, XYpoint};

mod point;

fn main() {

    let a: XYpoint<f64> = XYpoint { x: 3.1, y: 5.3 };
    let c: XYpoint<i32> = XYpoint{ x:1, y : 1};
    let d = a.ccw();
    let e = a.cw();
    let f = c.ccw();
    let g = c.cw();

    println!("Point A: ({}, {}):", a.x, a.y);
    println!("CW: ({},{}), CCW: ({},{})", d.x, d.y, e.x, e.y);

    println!("Point B: ({}, {}):", c.x, c.y);
    println!("CW: ({},{}), CCW: ({},{})", f.x, f.y, g.x, g.y);
    println!("Hello, world!");
}
