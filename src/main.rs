extern crate console;

use std::io::BufRead;

use console::{Key, Term};

const WORLD_SIZE: usize = 25;

#[derive(Clone, Copy, Debug)]
struct Cell {
    alive: bool,
    age: usize,
}

type CellBoard = [[Cell; WORLD_SIZE]; WORLD_SIZE];

struct World {
    cells: CellBoard,
}

impl World {
    fn new() -> World {
        let empty_board: CellBoard = [[Cell {
            alive: false,
            age: 0,
        }; WORLD_SIZE]; WORLD_SIZE];

        return World { cells: empty_board };
    }

    fn from_template() -> World {
        let mut data: Vec<u8> = Vec::new();

        // nasty unwrap
        // fix
        {
            let template_path = std::path::Path::new("board.dat");
            let mut file = std::fs::File::open(&template_path).unwrap();
            std::io::Read::read_to_end(&mut file, &mut data).unwrap();
        }

        // assuming template is valid
        // need to add error checking/handling later
        let mut world: World = World::new();
        let mut byte_count = 0;
        for char_byte in data {
            let current_char: char = char_byte as char;

            if current_char == 'x' {
                let y_loc: usize = byte_count / WORLD_SIZE;
                let x_loc: usize = byte_count % WORLD_SIZE;
                world.cells[x_loc][y_loc].alive = true;
            }

            // only update byte_count on valid encoded char
            if current_char == 'x' || current_char == 'o' {
                byte_count += 1;
            }
        }

        return world;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<Cell> {
        let mut neighbor_coords: [(isize, isize); 8] = [(0, 0); 8];
        let mut neighbor_index: usize = 0;
        for y_offset in -1..2 {
            for x_offset in -1..2 {
                // handle case of self
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                neighbor_coords[neighbor_index] = (x as isize + x_offset, y as isize + y_offset);
                neighbor_index += 1;
            }
        }

        let mut neighbors: Vec<Cell> = Vec::with_capacity(8);
        for cell in 0..8 {
            let neighbor_x: isize = neighbor_coords[cell].0;
            let neighbor_y: isize = neighbor_coords[cell].1;
            if neighbor_x >= 0
                && neighbor_y >= 0
                && neighbor_x < WORLD_SIZE as isize
                && neighbor_y < WORLD_SIZE as isize
            {
                neighbors.push(self.cells[neighbor_x as usize][neighbor_y as usize]);
            }
        }

        return neighbors;
    }

    fn get_max_age_adjacent(&self, x: usize, y: usize) -> usize {
        let neighbors: Vec<Cell> = self.get_neighbors(x, y);

        let num_neighbors: usize = neighbors.len();
        let mut max_age: usize = 0;
        for neighbor in 0..num_neighbors {
            let neighbor_age: usize = neighbors[neighbor].age;
            if neighbors[neighbor].age > max_age {
                max_age = neighbor_age;
            }
        }

        return max_age;
    }

    fn get_adjacent_alive_count(&self, x: usize, y: usize) -> usize {
        let neighbors: Vec<Cell> = self.get_neighbors(x, y);

        let num_neighbors: usize = neighbors.len();
        let mut alive_count: usize = 0;
        for neighbor in 0..num_neighbors {
            if neighbors[neighbor].alive {
                alive_count += 1;
            }
        }

        return alive_count;
    }

    fn tick(&self) {}
}

fn main() {
    let world: World = World::from_template();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbors_default() {
        let mut test_world: World = World::new();
        test_world.cells[0][0].age = 1;
        test_world.cells[1][0].age = 2;
        test_world.cells[2][0].age = 3;
        test_world.cells[0][1].age = 4;
        test_world.cells[2][1].age = 5;
        test_world.cells[0][2].age = 6;
        test_world.cells[1][2].age = 7;
        test_world.cells[2][2].age = 8;

        let neighbors: Vec<Cell> = test_world.get_neighbors(1, 1);
        let expected_result: Vec<Cell> = vec![
            Cell {
                alive: false,
                age: 1,
            },
            Cell {
                alive: false,
                age: 2,
            },
            Cell {
                alive: false,
                age: 3,
            },
            Cell {
                alive: false,
                age: 4,
            },
            Cell {
                alive: false,
                age: 5,
            },
            Cell {
                alive: false,
                age: 6,
            },
            Cell {
                alive: false,
                age: 7,
            },
            Cell {
                alive: false,
                age: 8,
            },
        ];

        assert_eq!(neighbors.len(), expected_result.len());
        for i in 0..8 {
            assert_eq!(neighbors[i].alive, expected_result[i].alive);
            assert_eq!(neighbors[i].age, expected_result[i].age);
        }
    }

    #[test]
    fn test_get_neighbors_bottom_left() {
        let mut test_world: World = World::new();
        test_world.cells[1][0].age = 1;
        test_world.cells[0][1].age = 2;
        test_world.cells[1][1].age = 3;

        let neighbors: Vec<Cell> = test_world.get_neighbors(0, 0);
        let expected_result: Vec<Cell> = vec![
            Cell {
                alive: false,
                age: 1,
            },
            Cell {
                alive: false,
                age: 2,
            },
            Cell {
                alive: false,
                age: 3,
            },
        ];

        assert_eq!(neighbors.len(), expected_result.len());
        for i in 0..3 {
            assert_eq!(neighbors[i].alive, expected_result[i].alive);
            assert_eq!(neighbors[i].age, expected_result[i].age);
        }
    }

    #[test]
    fn test_get_neighbors_top_right() {
        let mut test_world: World = World::new();
        test_world.cells[WORLD_SIZE - 2][WORLD_SIZE - 2].age = 1;
        test_world.cells[WORLD_SIZE - 1][WORLD_SIZE - 2].age = 2;
        test_world.cells[WORLD_SIZE - 2][WORLD_SIZE - 1].age = 3;

        let neighbors: Vec<Cell> = test_world.get_neighbors(WORLD_SIZE - 1, WORLD_SIZE - 1);
        let expected_result: Vec<Cell> = vec![
            Cell {
                alive: false,
                age: 1,
            },
            Cell {
                alive: false,
                age: 2,
            },
            Cell {
                alive: false,
                age: 3,
            },
        ];

        assert_eq!(neighbors.len(), expected_result.len());
        for i in 0..3 {
            assert_eq!(neighbors[i].alive, expected_result[i].alive);
            assert_eq!(neighbors[i].age, expected_result[i].age);
        }
    }

    #[test]
    fn test_get_max_age_adjacent() {
        let mut test_world: World = World::new();
        test_world.cells[0][0] = Cell {
            alive: true,
            age: 2,
        };
        test_world.cells[2][1] = Cell {
            alive: true,
            age: 1,
        };
        test_world.cells[1][2] = Cell {
            alive: true,
            age: 42,
        };

        let max_age: usize = test_world.get_max_age_adjacent(1, 1);
        assert_eq!(max_age, 42);
    }

    #[test]
    fn test_get_adjacent_alive_count() {
        let mut test_world: World = World::new();
        test_world.cells[0][0] = Cell {
            alive: true,
            age: 2,
        };
        test_world.cells[2][1] = Cell {
            alive: true,
            age: 1,
        };
        test_world.cells[1][2] = Cell {
            alive: true,
            age: 42,
        };

        let alive_count: usize = test_world.get_adjacent_alive_count(1, 1);
        assert_eq!(alive_count, 3);
    }
}
