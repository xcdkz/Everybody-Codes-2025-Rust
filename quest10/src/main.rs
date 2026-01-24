use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum TileValue {
    Empty,
    Sheep,
    Dragon,
    Hideout,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BoardPosition {
    row: usize,
    column: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    position: BoardPosition,
    value: TileValue,
}

impl Tile {
    fn pass_turns_dragon(
        &self,
        turns: usize,
        chessboard_rows: usize,
        chessboard_columns: usize,
    ) -> Result<HashSet<Tile>, String> {
        if self.value != TileValue::Dragon {
            return Err(format!("Invalid TileValue: {:?}", self.value));
        }
        let mut result: HashSet<Tile> = HashSet::from([self.clone()]);
        for _ in 0..turns {
            let mut updated_result: HashSet<Tile> = HashSet::new();
            for tile in &result {
                if tile.position.row > 1 {
                    if tile.position.column > 0 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row - 2,
                                column: tile.position.column - 1,
                            },
                            value: self.value,
                        });
                    }
                    if tile.position.column < chessboard_columns - 1 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row - 2,
                                column: tile.position.column + 1,
                            },
                            value: self.value,
                        });
                    }
                }
                // Right side moves
                if tile.position.column < chessboard_columns - 2 {
                    if tile.position.row > 0 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row - 1,
                                column: tile.position.column + 2,
                            },
                            value: self.value,
                        });
                    }
                    if tile.position.row < chessboard_rows - 1 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row + 1,
                                column: tile.position.column + 2,
                            },
                            value: self.value,
                        });
                    }
                }
                // Downwards moves
                if tile.position.row < chessboard_rows - 2 {
                    if tile.position.column > 0 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row + 2,
                                column: tile.position.column - 1,
                            },
                            value: self.value,
                        });
                    }
                    if tile.position.column < chessboard_columns - 1 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row + 2,
                                column: tile.position.column + 1,
                            },
                            value: self.value,
                        });
                    }
                }
                // Left side moves
                if tile.position.column > 1 {
                    if tile.position.row > 0 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row - 1,
                                column: tile.position.column - 2,
                            },
                            value: self.value,
                        });
                    }
                    if tile.position.row < chessboard_rows - 1 {
                        updated_result.insert(Tile {
                            position: BoardPosition {
                                row: tile.position.row + 1,
                                column: tile.position.column - 2,
                            },
                            value: self.value,
                        });
                    }
                }
            }
            result = updated_result;
        }
        Ok(result)
    }

    fn pass_turns_sheep(
        &self,
        turns: usize,
        chessboard_rows: usize,
    ) -> Result<Option<Tile>, String> {
        if self.value != TileValue::Sheep {
            Err(format!("Invalid TileValue: {:?}", self.value))
        } else if self.position.row + turns >= chessboard_rows {
            Ok(None)
        } else {
            Ok(Some(Tile {
                position: BoardPosition {
                    row: self.position.row + turns,
                    column: self.position.column,
                },
                value: self.value,
            }))
        }
    }
}

#[derive(Debug)]
struct Chessboard {
    tiles: Vec<Tile>,
    rows: usize,
    columns: usize,
}

fn main() {
    match extract_input_from_file("input/test1.txt") {
        Ok(input) => {
            assert_eq!(part1(&input, 3), Ok(27), "Test1 FAILED");
            println!("Test1 PASSED");
        }
        Err(e) => eprintln!("Input 1 Error: {}", e),
    };
    match extract_input_from_file("input/input1.txt") {
        Ok(input) => println!("Input 1: {:?}", part1(&input, 4)),
        Err(e) => eprintln!("Input 1 Error: {}", e),
    };
    match extract_input_from_file("input/test2.txt") {
        Ok(input) => {
            assert_eq!(part2(&input, 3), Ok(27), "Test2 FAILED");
            println!("Test2 PASSED");
        }
        Err(e) => eprintln!("Input 2 Error: {}", e),
    };
    match extract_input_from_file("input/input2.txt") {
        Ok(input) => println!("Input 2: {:?}", part2(&input, 20)),
        Err(e) => eprintln!("Input 2 Error: {}", e),
    };
    match extract_input_from_file("input/test3_1.txt") {
        Ok(input) => {
            assert_eq!(part3(&input), Ok(15), "Test3_1 FAILED");
            print!("Test3_1 PASSED");
        }
        Err(e) => eprintln!("Input 3 Error: {}", e),
    };
    match extract_input_from_file("input/test3_2.txt") {
        Ok(input) => {
            assert_eq!(part3(&input), Ok(8), "Test3_2 FAILED");
            println!("Test3_2 PASSED");
        }
        Err(e) => eprintln!("Input 3 Error: {}", e),
    };
    match extract_input_from_file("input/test3_3.txt") {
        Ok(input) => {
            assert_eq!(part3(&input), Ok(44), "Test3_3 FAILED");
            println!("Test3_3 PASSED");
        }
        Err(e) => eprintln!("Input 3 Error: {}", e),
    };
    match extract_input_from_file("input/test3_4.txt") {
        Ok(input) => {
            assert_eq!(part3(&input), Ok(4406), "Test3_4 FAILED");
            println!("Test3_4 PASSED");
        }
        Err(e) => eprintln!("Input 3 Error: {}", e),
    };
    match extract_input_from_file("input/test3_5.txt") {
        Ok(input) => {
            assert_eq!(part3(&input), Ok(13033988838), "Test3_5 FAILED");
            println!("Test3_5 PASSED");
        }
        Err(e) => eprintln!("Input 3 Error: {}", e),
    };
    match extract_input_from_file("input/input3.txt") {
        Ok(input) => println!("Input 3: {:?}", part3(&input)),
        Err(e) => eprintln!("Input 3 Error: {}", e),
    };
}

fn part3(_chessboard: &Chessboard) -> Result<u128, String> {
    todo!()
}

fn part1(chessboard: &Chessboard, turns: usize) -> Result<usize, String> {
    let dragon_tile = chessboard
        .tiles
        .iter()
        .find(|tile| tile.value == TileValue::Dragon)
        .ok_or("No dragon found on board")?;
    let mut possible_dragon_moves: HashSet<Tile> = HashSet::from([dragon_tile.clone()]);
    for i in 1..=turns {
        possible_dragon_moves.extend(dragon_tile.pass_turns_dragon(
            i,
            chessboard.rows,
            chessboard.columns,
        )?);
    }
    Ok(chessboard
        .tiles
        .iter()
        .filter(|tile| {
            tile.value == TileValue::Sheep
                && possible_dragon_moves
                    .iter()
                    .any(|dragon_tile| dragon_tile.position == tile.position)
        })
        .count())
}

fn part2(chessboard: &Chessboard, rounds: usize) -> Result<usize, String> {
    let initial_dragon_tile = chessboard
        .tiles
        .iter()
        .find(|tile| tile.value == TileValue::Dragon)
        .ok_or("No dragon found on board")?
        .clone();

    let mut dragon_tiles: HashSet<Tile> = HashSet::from([initial_dragon_tile]);
    let mut result = 0;

    let hideouts: HashSet<Tile> = chessboard
        .tiles
        .iter()
        .filter(|tile| tile.value == TileValue::Hideout)
        .cloned()
        .collect();

    let mut sheeps: HashSet<Tile> = chessboard
        .tiles
        .iter()
        .filter(|tile| tile.value == TileValue::Sheep)
        .cloned()
        .collect();

    for _ in 1..=rounds {
        let mut tmp_container_dragon_tiles = HashSet::new();
        for tile in dragon_tiles {
            tmp_container_dragon_tiles.extend(
                tile.pass_turns_dragon(1, chessboard.rows, chessboard.columns)?
                    .iter()
                    .cloned(),
            );
        }
        dragon_tiles = tmp_container_dragon_tiles;
        for sheep in &sheeps.clone() {
            if dragon_tiles.iter().any(|dragon| {
                dragon.position == sheep.position
                    && !hideouts
                        .iter()
                        .any(|hideout| hideout.position == sheep.position)
            }) {
                sheeps.remove(sheep);
                result += 1;
            }
        }
        sheeps = sheeps
            .iter()
            .map(|sheep| sheep.pass_turns_sheep(1, chessboard.rows))
            .collect::<Result<Vec<Option<Tile>>, String>>()?
            .into_iter()
            .flatten()
            .collect();
        for sheep in &sheeps.clone() {
            if dragon_tiles.iter().any(|dragon| {
                dragon.position == sheep.position
                    && !hideouts
                        .iter()
                        .any(|hideout| hideout.position == sheep.position)
            }) {
                sheeps.remove(sheep);
                result += 1;
            }
        }
    }

    Ok(result)
}

fn extract_input_from_file(file_path: &str) -> Result<Chessboard, String> {
    let file_string = std::fs::read_to_string(file_path)
        .map_err(|e| format!("Couldn't read from file: {}", e))?;

    let chessboard_columns = &file_string
        .trim()
        .lines()
        .next()
        .ok_or(format!("No lines in a file: {}", file_path))?
        .len();
    let chessboard_rows = &file_string.trim().lines().count();
    let mut chessboard_tiles = Vec::new();

    for (row, line) in file_string.trim().lines().enumerate() {
        for (column, symbol) in line.bytes().enumerate() {
            chessboard_tiles.push(Tile {
                position: BoardPosition { row, column },
                value: match symbol {
                    b'.' => Ok(TileValue::Empty),
                    b'S' => Ok(TileValue::Sheep),
                    b'D' => Ok(TileValue::Dragon),
                    b'#' => Ok(TileValue::Hideout),
                    _ => Err(format!("Couldn't parse symbol: {}", symbol)),
                }?,
            });
        }
    }

    Ok(Chessboard {
        rows: *chessboard_rows,
        columns: *chessboard_columns,
        tiles: chessboard_tiles,
    })
}
