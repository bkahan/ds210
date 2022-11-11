/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

pub mod point {
    use std::ops::Neg;

    #[derive(Copy, Clone)]
    pub struct XYpoint<T> {
        pub x : T,
        pub y : T,
    }

    pub trait Point {
        fn clockwise(self) -> Self;
        fn counterclockwise(self) -> Self;
        fn to_string(self) -> String;
    }

    impl<T:Neg<Output = T>> Neg for XYpoint<T> {
        type Output = XYpoint<T>;

        fn neg(self) -> Self {
            return XYpoint {x: - self.x, y: - self.y};
        }
    }

    impl Point for XYpoint<f64> {

        fn clockwise(self) -> Self {
            // matrix: [[0,1],[-1,0]]
            let tmp = self::XYpoint::neg(self);

            return Self { x: self.y, y : tmp.x };
        }

        fn counterclockwise(self) -> Self {
            // matrix: [[0,-1],[1,0]]
            return Self { x: self.y, y : self.x };
        }

        fn to_string(self) -> String {
            todo!()
        }

    }
}