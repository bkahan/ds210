/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

pub mod point {
    use std::ops::Neg;

    pub struct XYpoint<T> {
        x : T,
        y : T,
    }

    pub trait Point: Copy + Neg {
        fn clockwise(&self) -> Box<Self>;
        fn counterclockwise(&self) -> Box<Self>;
        fn to_string(&self) -> String;
    }

    impl<T> Clone for XYpoint<T> {
        fn clone(&self) -> Self {
            todo!()
        }
    }

    impl<T> Neg for XYpoint<T> {
        type Output = ();

        fn neg(self) -> Self::Output {
            todo!()
        }
    }

    impl<T> Point for XYpoint<T> {
        fn clockwise(&self) -> Box<Self> {
            todo!()
        }

        fn counterclockwise(&self) -> Box<Self> {
            todo!()
        }

        fn to_string(&self) -> String {
            todo!()
        }
    }



}
