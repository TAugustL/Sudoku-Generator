use rand::{self, seq::SliceRandom, Rng};
use std::{collections::HashMap, env::args};

const SUDOKU_FIELD: &str = "
┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┠───┼───┼───╂───┼───┼───╂───┼───┼───┨
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┠───┼───┼───╂───┼───┼───╂───┼───┼───┨
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┠───┼───┼───╂───┼───┼───╂───┼───┼───┨
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┠───┼───┼───╂───┼───┼───╂───┼───┼───┨
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┠───┼───┼───╂───┼───┼───╂───┼───┼───┨
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┠───┼───┼───╂───┼───┼───╂───┼───┼───┨
┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃ 0 │ 0 │ 0 ┃
┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛
";

const NUMBERS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

const FIELDS: [[usize; 9]; 9] = [
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

struct Grid {
    sudoku: String,
    state: [[char; 9]; 9],
    difficulty: f32,
    solving: bool,
}

impl Grid {
    fn solve(&mut self) -> bool {
        if let Some((r, c)) = self.find_empty_cell() {
            let mut rng = rand::thread_rng();
            let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
            let nums: Vec<&usize> = nums.choose_multiple(&mut rng, 9).collect();

            for i in 0..9 {
                if is_move_valid(self.state, (c + r * 9, NUMBERS[*nums[i]])) {
                    self.state[r][c] = NUMBERS[*nums[i]];
                    if self.solve() {
                        return true;
                    }
                    self.state[r][c] = '0';
                }
            }
            return false;
        }
        true
    }
    fn find_empty_cell(&mut self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if self.state[i][j] == '0' {
                    return Some((i, j));
                }
            }
        }
        return None;
    }
    fn create_grid(&mut self) -> () {
        let mut index = 0;
        for (i, ch) in self.sudoku.clone().char_indices() {
            if ch.is_numeric() {
                let (c, r) = to_col_row(index);
                self.sudoku
                    .replace_range(i..i + 1, &self.state[r][c].to_string());
                index += 1;
            }
        }
    }
    fn make_game(&mut self) -> () {
        let mut rng = rand::thread_rng();

        for (i, ch) in self.sudoku.clone().char_indices() {
            if ch.is_numeric() && rng.gen_range(0.0..1.0) > self.difficulty {
                self.sudoku.replace_range(i..i + 1, " ");
            }
        }
    }
}

fn main() {
    let mut solving: bool = false;
    let mut difficulty: f32 = 0.5;

    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        difficulty = args[1].parse().unwrap_or(0.5);
    }

    let sudoku: String = match std::fs::read_to_string("./to_be_solved.txt") {
        Ok(text) => {
            solving = true;
            text
        }
        _ => SUDOKU_FIELD.to_string(),
    };

    let state = get_state_from_grid(sudoku.clone());

    let mut sudoku_game = Grid {
        sudoku,
        state,
        difficulty,
        solving,
    };

    if sudoku_game.solve() {
        sudoku_game.create_grid();
        println!("{}", sudoku_game.sudoku);
        if !sudoku_game.solving {
            sudoku_game.make_game();
            println!("{}", sudoku_game.sudoku);
        }
    } else {
        println!("No solution was found!");
    }
}

fn get_state_from_grid(sudoku: String) -> [[char; 9]; 9] {
    let mut indices: HashMap<usize, usize> = HashMap::new();
    let mut state: [[char; 9]; 9] = [['0'; 9]; 9];
    {
        let mut index = 0;
        for (i, ch) in sudoku.char_indices() {
            if ch.is_numeric() {
                let (c, r) = to_col_row(index);
                indices.insert(i, index);
                state[r][c] = ch;
                index += 1;
            }
        }
    }
    state
}

fn to_col_row(i: usize) -> (usize, usize) {
    let r = (i as f32 / 9.0) as usize;
    let c = i % 9;
    (c, r)
}

fn is_move_valid(state: [[char; 9]; 9], (index, chr): (usize, char)) -> bool {
    let (c, r) = to_col_row(index);

    if c > 0 {
        for x in 0..c {
            if state[r][x] == chr {
                return false;
            }
        }
    }
    if c < 8 {
        for x in c + 1..=8 {
            if state[r][x] == chr {
                return false;
            }
        }
    }
    if r > 0 {
        for y in 0..r {
            if state[y][c] == chr {
                return false;
            }
        }
    }
    if r < 8 {
        for y in r + 1..=8 {
            if state[y][c] == chr {
                return false;
            }
        }
    }

    for i in FIELDS {
        if i.contains(&index) {
            for j in i {
                let (ct, rt) = to_col_row(j);
                if state[rt][ct] == chr {
                    return false;
                }
            }
        }
    }
    true
}
