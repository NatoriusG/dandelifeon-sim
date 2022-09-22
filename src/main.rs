extern crate console;

use console::{Color, Key, Term};

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

type CellChange = (usize, usize, Cell);

impl World {
    fn new() -> World {
        let empty_board: CellBoard = [[Cell {
            alive: false,
            age: 0,
        }; WORLD_SIZE]; WORLD_SIZE];

        return World { cells: empty_board };
    }

    // return list of cells applied by template as well as world with template state
    fn from_template() -> (World, Vec<CellChange>) {
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
        let mut template_cells: Vec<CellChange> = Vec::with_capacity(WORLD_SIZE * WORLD_SIZE);
        let mut byte_count = 0;
        for char_byte in data {
            let current_char: char = char_byte as char;

            if current_char == 'x' {
                let y_loc: usize = byte_count / WORLD_SIZE;
                let x_loc: usize = byte_count % WORLD_SIZE;
                let template_cell: Cell = Cell {
                    alive: true,
                    age: 0,
                };

                world.cells[x_loc][y_loc] = template_cell;
                template_cells.push((x_loc, y_loc, template_cell));
            }

            // only update byte_count on valid encoded char
            if current_char == 'x' || current_char == '-' {
                byte_count += 1;
            }
        }

        return (world, template_cells);
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

        let alive_count: usize =
            neighbors
                .into_iter()
                .fold(0, |alive_count: usize, neighbor: Cell| {
                    if neighbor.alive {
                        alive_count + 1
                    } else {
                        alive_count
                    }
                });

        return alive_count;
    }

    // returns a vector of x,y,cell tuples to track which cells have changed
    fn tick(&mut self) -> Vec<CellChange> {
        // max possible changes is every cell, and there are WORLD_SIZE * WORLD_SIZE total cells
        let mut changes: Vec<CellChange> = Vec::with_capacity(WORLD_SIZE * WORLD_SIZE);

        for y in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let current_cell = self.cells[x][y];
                let alive_neighbors = self.get_adjacent_alive_count(x, y);

                if current_cell.alive {
                    // surviving
                    if alive_neighbors == 2 || alive_neighbors == 3 {
                        changes.push((
                            x,
                            y,
                            Cell {
                                alive: true,
                                age: std::cmp::min(current_cell.age + 1, 100),
                            },
                        ));
                    }
                    // dying
                    else {
                        changes.push((
                            x,
                            y,
                            Cell {
                                alive: false,
                                age: 0,
                            },
                        ))
                    }
                }
                // reviving
                else if alive_neighbors == 3 {
                    let new_age = self.get_max_age_adjacent(x, y);
                    changes.push((
                        x,
                        y,
                        Cell {
                            alive: true,
                            age: std::cmp::min(new_age, 100),
                        },
                    ));
                }
            }
        }

        // propagate changes
        for change in changes.iter() {
            let x: usize = change.0;
            let y: usize = change.1;
            let new_state: Cell = change.2;

            self.cells[x][y] = new_state;
        }

        return changes;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Tile {
    value: char,
    color: Color,
}

type RenderBuffer = [[Tile; WORLD_SIZE]; WORLD_SIZE];

struct Renderer {
    terminal: Term,
    buffer: [RenderBuffer; 2],
}

impl Renderer {
    fn new() -> Renderer {
        let init_buffers: [RenderBuffer; 2] = [[[Tile {
            value: '-',
            color: Color::White,
        }; WORLD_SIZE]; WORLD_SIZE]; 2];

        let init_term: Term = Term::stdout();
        // unwrap still bad
        init_term.hide_cursor().unwrap();
        init_term.clear_screen().unwrap();

        let mut renderer: Renderer = Renderer {
            terminal: init_term,
            buffer: init_buffers,
        };
        renderer.render(true);

        return renderer;
    }

    fn update(&mut self, x: usize, y: usize, value: char, color: Color) {
        self.buffer[1][x][y] = Tile { value, color };
    }

    // render changes to console, optionally ignoring buffer differences to write everything
    fn render(&mut self, force: bool) {
        for y in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                if self.buffer[0][x][y] != self.buffer[1][x][y] || force == true {
                    // write new data to terminal
                    let tile_to_write: Tile = self.buffer[1][x][y];
                    // nasty unwrap, fix!
                    // multiply by 2 to maintain a space between each cell for nice rendering
                    self.terminal.move_cursor_to(x * 2, y).unwrap();
                    println!(
                        "{}",
                        self.terminal
                            .style()
                            .fg(tile_to_write.color)
                            .apply_to(tile_to_write.value)
                    );

                    // update buffer
                    self.buffer[0][x][y] = self.buffer[1][x][y];
                }
            }
        }
    }

    // ignores whether buffers are different
    /*fn force_render(&mut self) {
        // more nasty unwrap to fix
        self.terminal.clear_screen().unwrap();
        self.terminal.move_cursor_to(0, 0).unwrap();

        for y in 0..WORLD_SIZE {
            for x in 0..WORLD_SIZE {
                let tile_to_write: Tile = self.buffer[1][x][y];
                // ditto on unwwrap
                self.terminal.move_cursor_to(x * 4, y * 2).unwrap();
                print!(
                    "{}",
                    self.terminal
                        .style()
                        .fg(tile_to_write.color)
                        .apply_to(tile_to_write.value)
                );

                // handle newline
                if x == 24 {
                    print!("\n");
                }

                self.buffer[0][x][y] = self.buffer[1][x][y];
            }
        }
    }*/
}

fn main() {
    let mut world: World;
    let template_cells: Vec<(usize, usize, Cell)>;
    (world, template_cells) = World::from_template();

    let mut output: Renderer = Renderer::new();
    let mut changed_cells: Vec<CellChange> = template_cells;

    let mut running: bool = true;
    while running {
        for change in changed_cells.iter() {
            let x: usize = change.0;
            let y: usize = change.1;
            let new_cell: Cell = change.2;

            let cell_char: char = {
                if new_cell.alive {
                    'X'
                } else {
                    '-'
                }
            };
            let color: Color = {
                if !new_cell.alive {
                    Color::Red
                } else {
                    let color_index: usize = 34 + (new_cell.age / 20);
                    // unwrap bad
                    Color::Color256(color_index.try_into().unwrap())
                }
            };

            output.update(x, y, cell_char, color);
        }

        output.render(false);

        let mut ages: Vec<(usize, usize, usize)> = world
            .cells
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(index, cell)| {
                if cell.alive {
                    Some((cell.age, index % 25, index / 25))
                } else {
                    None
                }
            })
            .collect();
        ages.sort_by(|a, b| b.0.cmp(&a.0));

        // you know the drill
        output.terminal.move_cursor_to(0, WORLD_SIZE).unwrap();
        for (count, age_data) in ages.iter().take(25).enumerate() {
            let age: usize = age_data.0;
            let x: usize = age_data.1;
            let y: usize = age_data.2;

            // aaaaaa
            output
                .terminal
                .move_cursor_to((WORLD_SIZE + 1) * 2, count + 1)
                .unwrap();
            println!("({x}, {y}): {age}");
        }

        // unwrap bad
        let key: Key = output.terminal.read_key().unwrap();
        if key == Key::Escape {
            running = false;
        } else {
            changed_cells = world.tick();
        }
    }

    // just for testing so the program ending doesn't overwrite part of the board
    output.terminal.move_cursor_to(0, WORLD_SIZE).unwrap();
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
