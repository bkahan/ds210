use std::f64::consts::PI;

pub(crate) struct Shape {
    sides: Vec<f64>,
    radius: f64,
}

impl Shape {
    fn perimeter(shape: &Shape) -> f64 {
        if shape.radius != 0.0  {
            2.0*PI*shape.radius
        }
        else {
            match shape.sides.len() {
                2 => 4.0*shape.sides[0],  // unique case of rectangle
                3 => Self::sum_sides(shape),
                4 => Self::sum_sides(shape),
                _ => Self::greater_than4(),
            }
        }
    }

    pub(crate) fn area (shape: &Shape) -> f64 {
        if shape.radius != 0.0 {
            return f64::powf(shape.radius, 2.0) * PI;
        }
        else {
            match shape.sides.len() {
                2 => f64::powf(shape.sides[0],2.0),
                3 => {
                    let s: f64 = Self::sum_sides(shape) / 2.0;
                    f64::sqrt(s * (s - shape.sides[0]) * (s - shape.sides[1]) * (s - shape.sides[2]))
                }
                _ => Self::greater_than4(),
            }
        }
    }

    pub(crate) fn sum_sides(shape : &Shape) -> f64 {
        let mut tmp: f64 = 0.0;
        for side in &shape.sides {
            tmp += side;
        }
        tmp
    }

    pub(crate) fn double_size(shape: &mut Shape) { // defining doubling as doubling each length of side ex. [2,6,10] -> [4,12,20]
        for x in 0..shape.sides.len() {
            shape.sides[x] = shape.sides[x]*2.0 ;
        }
    }

    fn greater_than4() -> f64 {
        println!("Not enough sides or not a formatted as a shape.");
        0.0 // for returns
    }

}

fn check_shape(sides : &Vec<f64>) -> bool {
    let  a:f64  = sides.iter().sum();
    return if sides.len() < 3 || a < 0.0 {
        println!("Not a shape.");
        false
    } else {
        true
    }
}

pub(crate) fn new_shape(s: Vec<f64>, r: f64) -> Shape {
    match check_shape(&s) {
        true => Shape { sides: s, radius: r },
        false => Shape { sides: vec![0.0], radius : 0.0}
    }
}