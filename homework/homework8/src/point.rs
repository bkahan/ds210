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
        fn ccw(self) -> Self;
        fn cw(self) -> Self;
    }

    impl<T:Neg<Output = T>> Neg for XYpoint<T> {
        type Output = XYpoint<T>;

        fn neg(self) -> Self {
            return XYpoint {x: - self.x, y: - self.y};
        }
    }

    impl Point for XYpoint<f64> {

        fn ccw(self) -> Self {
            // matrix: [[0,1],[-1,0]]
            let tmp = self::XYpoint::neg(self);

            return Self { x: self.y, y : tmp.x };
        }

        fn cw(self) -> Self {
            // matrix: [[0,-1],[1,0]]
            return Self { x: self.y, y : self.x };
        }
    }

    impl Point for XYpoint<i32> {

        fn ccw(self) -> Self {
            // matrix: [[0,1],[-1,0]]
            let tmp = self::XYpoint::neg(self);

            return Self { x: self.y, y : tmp.x };
        }

        fn cw(self) -> Self {
            // matrix: [[0,-1],[1,0]]
            let tmp = self::XYpoint::neg(self);

            return Self { x: tmp.y, y : self.x };
        }
    }

    #[test]
    fn test_neg() {
        let a = XYpoint {x: 1, y: -1};
        let b = a.neg();
        assert_eq!(- a.x, b.x);
    }

}