/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

pub mod point {
    pub struct XYpoint<T> {
        x : T,
        y : T,
    }

    pub trait Point {
        fn clockwise(&self) -> Box<Self>;
        fn counterclockwise(&self) -> Box<Self>;
        fn to_string(&self) -> String;
    }

    impl<T> Point for XYpoint<T> {
        pub fn clockwise(&self) -> Box<Self> {
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
