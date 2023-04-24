#![allow(dead_code)]
use rand::{thread_rng, Rng};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "test.csv";
    let grid = read_csv(file_path);
    let top_squares = find_top_five_prosperous_squares(&grid);
    top_squares.iter().for_each(|s| println!("{}", s));
}

fn read_csv(file_path: &str) -> [[u8; 1000]; 1000] {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => {
            println!("test.csv dataset not found, generating random dataset");
            let mut rng = thread_rng();
            let mut grid: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
            const SIZE: usize = 1000;
            for i in 0..SIZE {
                let mut row = [0; 1000];
                for j in 0..SIZE {
                    row[j] = rng.gen_range(0..=100);
                }
                grid[i] = row;
            }
            return grid;
        }
    };
    let reader = BufReader::new(file);
    let mut grid: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut values: [u8; 1000] = [0; 1000];

        for (col, s) in line.split(',').enumerate() {
            values[col] = s.parse().expect("Unable to parse number");
        }

        grid[row] = values;
    }
    grid
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Square {
    value: u32,
    row: usize,
    col: usize,
}
impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let end_row = self.row + 9;
        let end_col = self.col + 9;
        write!(
            f,
            "[Square] start row: {}, start column: {}, end row: {}, end column: {}, value: {}",
            self.row, self.col, end_row, end_col, self.value
        )
    }
}

pub fn find_top_five_prosperous_squares(grid: &[[u8; 1000]; 1000]) -> Vec<Square> {
    let mut top_squares = Vec::with_capacity(5);
    for row in 0..990 {
        let mut current_value = calculate_square_value(&grid, row, 0);

        for col in 0..991 {
            if col > 0 {
                let mut removed_value = 0;
                let mut added_value = 0;
                for r in row..row + 10 {
                    removed_value += grid[r][col - 1] as u32;
                    added_value += grid[r][col + 9] as u32;
                }
                current_value = current_value - removed_value + added_value;
            }

            let square = Square {
                value: current_value,
                row,
                col,
            };

            if top_squares.len() < 5 {
                top_squares.push(square.clone());
                top_squares.sort_unstable_by(|a, b| b.value.cmp(&a.value));
            } else if square.value > top_squares[4].value {
                top_squares[4] = square;
                top_squares.sort_unstable_by(|a, b| b.value.cmp(&a.value));
            }
        }
    }
    top_squares
}

fn calculate_square_value(grid: &[[u8; 1000]; 1000], row: usize, col: usize) -> u32 {
    let mut value = 0_u32;
    for r in row..row + 10 {
        for c in col..col + 10 {
            value += grid[r][c] as u32;
        }
    }
    value
}
