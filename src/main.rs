const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const BORDER: usize = 100;
const WORLD_SIZE: usize = 25;
const CELL_SIZE: usize = 64;

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
}

fn main() {}
