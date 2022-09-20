const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const BORDER: usize = 100;
const WORLD_SIZE: usize = 25;
const CELL_SIZE: usize = 64;

struct Cell {
    alive: bool,
    age: usize,
}

type CellBoard = [[Cell; WORLD_SIZE]; WORLD_SIZE];

struct World {
    cells: CellBoard,
}

fn main() {}
