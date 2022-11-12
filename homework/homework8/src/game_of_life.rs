/*
Ben Kahan
Homework 8
DS210
9 November 2022
Collaborators: none
 */

pub mod gol {

    pub mod game_engine {
        /*
        general idea:
        make a board with is a graph with each square connected to each other
        set each square to be unreachable
        init game with init coords, set those squares as reachable and set as alive
        calculate neighbors for each cell, skipping unreachable only (ie, count dead)
        update board by adjusting based on if statement in problem
        make new board from old one being updated
        iterate 9 more times

        fancy thing to do:
        print the board in ascii
         */
    }

    pub mod game {

        struct Cell {
            neighbors: i32, // -1 = not calculated
            is_reachable: bool,
            is_alive: bool,
        }

        pub struct Board {
            graph: Vec<Vec<Cell>>,
            iteration_num: i32,
        }

        impl Cell {
            fn set_neighbors(&mut self, num: i32) {
                self.neighbors = num;
            }

            fn set_is_alive(&mut self, is_alive: bool) {
                self.is_alive = is_alive;
            }

            fn set_is_reachable(&mut self, is_reachable: bool) {
                self.is_reachable = is_reachable;
            }

            fn init_cell(&mut self) {
                self.is_reachable = true;
                self.is_alive = true;
                self.neighbors = -1;
            }
        }

        impl Board {
            pub fn new_game(size: i32) -> Board {
                let mut tmp = Vec::new();
                for _ in 0..=size {
                    let mut cells = Vec::new();
                    for _ in 0..=size {
                        cells.push(Cell {
                            neighbors: -1,
                            is_reachable: false,
                            is_alive: false,
                        });
                    }
                    tmp.push(cells);
                }

                tmp.push(Vec::new());

                return Board {
                    graph: tmp,
                    iteration_num: 0,
                };
            }

            pub fn init_game(coordinates: Vec<Vec<i32>>, mut game: Board) {
                for mut coord in coordinates {
                    let y = coord.pop().unwrap();
                    let x = coord.pop().unwrap();

                    let game_index = game.graph.get_mut(x as usize);

                    let cell = game_index.unwrap().get_mut(y as usize);

                    Cell::init_cell(cell.unwrap());
                }
            }
        }
    }
}
