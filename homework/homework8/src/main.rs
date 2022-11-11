/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

use std::ops::Neg;
use crate::point::point::{Point, XYpoint};

mod point;

fn main() {

    let a: XYpoint<f64> = XYpoint { x: 0.0, y: 1.0 };
    let b = a.neg();
    let c = XYpoint { x:-1, y : 1};
    let d = c.neg();
    println!("Hello, world!");
}
