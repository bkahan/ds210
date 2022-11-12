/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

pub mod gol {

    pub mod game_engine {


    }

    pub mod game {

        struct Cell {
            is_alive: bool,
        }

        pub struct Board {
            graph: Vec<Vec<Cell>>,
            iteration_num: i32,
        }

        pub fn new_game(size: i32) -> Board {
            let mut tmp = Vec::new();
            for _ in 0..=size {
                let mut cells = Vec::new();
                for _ in 0..=size {
                    cells.push(Cell { is_alive: false });
                }
                tmp.push(cells);
            }

            tmp.push(Vec::new());

            return Board {
                graph: tmp,
                iteration_num: 0,
            };
        }

        

    }
}
