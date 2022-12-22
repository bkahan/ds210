use std::collections;


fn main() {
    fn find_210(vec: Vec<i32>) -> Option<Vec<i32>> { // question 10

        let mut indices = Vec::<i32>::new();

        let mut count = 0;

        for elem in vec {
            count += 1;
            if elem == 210 {
                indices.push(count);
            }
        }

        return if !indices.is_empty() {
            Some(indices)
        } else {
            None
        };
    }

    fn degree_of_vertex(adj_list: Vec<(usize, usize)>, v_id: usize) -> i32 { // (from a, to b) -> return number of connections // question 12
        let mut degree = 0;
        for (_source, _target) in adj_list {
            if _source == v_id {
                degree += 1;
            }
        }
        degree
    }

    fn median_of_5(mut arr: [i32; 5]) -> i32 { // question 13

        //median is the middle of the set,
        // sort the array
        //pull the middle index which will always be  [0,1,2,3,4] index 2
        arr.sort();
        arr[2]
    }

    // question 14

    pub enum ToyType {
        Caterpillar,
        Puzzle,
        Doll,
    }

    pub struct Toy {
        toy_type: ToyType,
        name: Option<String>,
        is_landscape: Option<bool>,
        num_pieces: Option<i32>,
        num_legs: Option<i32>,
    }

    impl Toy {
        fn num_legs(toy: Toy) -> i32 {
            return match toy.toy_type {
                ToyType::Caterpillar => {
                    toy.num_legs.unwrap()
                }
                ToyType::Puzzle => {
                    0
                }
                ToyType::Doll => {
                    2
                }
            };
        }

        fn total_legs(toy_list: Vec<Toy>) -> i32 {
            let mut legs = 0;

            for toy in toy_list {
                legs += Toy::num_legs(toy);
            }
            legs
        }
    }

    fn triple_zero_check(vec: Vec<i32>) -> bool {

        for _x in vec.clone() {
            for _y in vec.clone() {
                for _z in vec.clone() {

                     if _x + _y + _z == 0 {
                         return true;
                     }
                }
            }
        }
        false
    }

    println!("Hello, world!");

    let mut q10 = Vec::<i32>::new();

    q10.push(100);
    q10.push(211);
    q10.push(-1);
    q10.push(210);

    let b = find_210(q10).unwrap();

    for element in b {
        println!("{}", element)
    }

    println!();

    let mut arr = [1, 4, 2, 100, 10]; // 1 2 4 10 100
    let mut arr2 = [0, 0, 0, 2, 1]; // 0 0 0 1 2
    let mut arr3 = [0, 1, 2, 3, 4]; // 0 1 2 3 4

    println!("{}", median_of_5(arr));
    println!("{}", median_of_5(arr2));
    println!("{}", median_of_5(arr3));


    let mut t1 = Toy {
        toy_type: ToyType::Caterpillar,
        name: None,
        is_landscape: None,
        num_pieces: None,
        num_legs: Option::from(120),
    };

    let mut t2 = Toy {
        toy_type: ToyType::Caterpillar,
        name: None,
        is_landscape: None,
        num_pieces: None,
        num_legs: Option::from(1000),

    };

    let mut t3 = Toy {
        toy_type: ToyType::Puzzle,
        name: None,
        is_landscape: Option::from(true),
        num_pieces: Option::from(1000),
        num_legs: None,
    };

    println!();

    // println!("{}", Toy::num_legs(t1));
    // println!("{}", Toy::num_legs(t2));
    // println!("{}", Toy::num_legs(t3));

    let mut toy_list = Vec::<Toy>::new();
    toy_list.push(t1);
    toy_list.push(t2);
    toy_list.push(t3);

    println!("{}", Toy::total_legs(toy_list));


    let test = vec![-4, 2];

    println!("{}", triple_zero_check(test));


}
