use std::f64::consts::PI;

pub(crate) struct Shape {
    sides: Vec<f64>,
    radius: f64,
}

impl Shape {
    fn perimeter(shape: Shape) -> f64 {
        if shape.radius != 0.0  {
            return 2.0*PI*shape.radius;
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

    fn area (shape: Shape) -> f64 {
        if shape.radius != 0.0 {
            return f64::powf(shape.radius, 2.0) * PI;
        }
        else {
            match shape.sides.len() {
                2 => f64::powf(shape.sides[0],2.0),
                3 => {
                    let s: f64 = Self::sum_sides(shape) / 2.0;
                    return f64::sqrt(s * (s - shape.sides[0]) * (s - shape.sides[1]) * (s - shape.sides[2]));
                }
                _ => Self::greater_than4(),
            }
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

    fn greater_than4() -> f64 {
        println!("Not enough sides.");
        return 0.0;
    }

}

pub(crate) fn new_shape(s: Vec<f64>, r: f64) -> Shape {
    Shape { sides: s, radius: r }
}