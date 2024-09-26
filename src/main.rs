use rand::{self, Rng};
use std::collections::HashMap;
use std::env::args;

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

#[allow(dead_code)]
#[derive(Debug)]
enum Difficulty {
    HARD,
    MEDIUM,
    EASY,
}

fn main() {
    // println!("Sudoku-game:{}", SUDOKU_FIELD);
    let mut sudoku: String = SUDOKU_FIELD.to_string();
    let mut state: [[char; 9]; 9] = [['0'; 9]; 9];
    fill_grid(&mut sudoku, &mut state);
}

fn make_game(difficulty: Difficulty, game: &mut String, game_indices: HashMap<usize, usize>) -> () {
    let chance: f32;
    match difficulty {
        Difficulty::HARD => chance = 0.275,
        Difficulty::MEDIUM => chance = 0.389,
        Difficulty::EASY => chance = 0.563,
    }
    let mut random = rand::thread_rng();

    println!("Solved: {game}");

    for i in game_indices.keys() {
        if random.gen_range(0.0..1.0) > chance {
            game.replace_range(i..&(i + 1), " ");
        }
    }
    println!("Difficulty: {:?} {game}", difficulty);
}

fn fill_grid(sudoku: &mut String, state: &mut [[char; 9]; 9]) -> () {
    let mut random = rand::thread_rng();

    let mut indices: HashMap<usize, usize> = HashMap::new();

    {
        let mut index = 0;
        for (i, chr) in SUDOKU_FIELD.char_indices() {
            if chr == '0' {
                let (c, r) = to_col_row(index);
                indices.insert(i, index);
                state[r][c] = chr;
                index += 1;
            }
        }
    }
    for (i, ch) in SUDOKU_FIELD.char_indices() {
        if ch == '0' {
            let true_index = *indices.get(&i).unwrap();
            let mut num = NUMBERS[random.gen_range(0..=8)];
            let mut used_num: Vec<char> = Vec::new();
            while !is_move_valid(*state, (true_index, num)) {
                if used_num.len() >= 9 {
                    fill_grid(sudoku, state);
                    break;
                }
                while used_num.contains(&num) {
                    num = NUMBERS[random.gen_range(0..=8)];
                }
                used_num.push(num);
            }
            sudoku.replace_range(i..(i + 1), &String::from(num));
            let (c, r) = to_col_row(true_index);
            state[r][c] = num;
        }
    }
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("No arguments given!");
        std::process::exit(0x0100);
    }
    let difficulty: Difficulty = match args[1].trim() {
        "e" => Difficulty::EASY,
        "m" => Difficulty::MEDIUM,
        "h" => Difficulty::HARD,
        _ => {
            println!("Invalid argument '{}', using easy as difficulty!", args[1]);
            Difficulty::EASY
        }
    };

    make_game(difficulty, sudoku, indices);
    std::process::exit(0x0100);
}

fn to_col_row(i: usize) -> (usize, usize) {
    let r = (i as f32 / 9.0) as usize;
    let c = i % 9;
    (c, r)
}

fn is_move_valid(field_state: [[char; 9]; 9], (index, chr): (usize, char)) -> bool {
    let (c, r) = to_col_row(index);

    if c > 0 {
        for x in 0..c {
            if field_state[r][x] == chr {
                return false;
            }
        }
    }
    if c < 8 {
        for x in c + 1..=8 {
            if field_state[r][x] == chr {
                return false;
            }
        }
    }
    if r > 0 {
        for y in 0..r {
            if field_state[y][c] == chr {
                return false;
            }
        }
    }
    if r < 8 {
        for y in r + 1..=8 {
            if field_state[y][c] == chr {
                return false;
            }
        }
    }

    for i in FIELDS {
        if i.contains(&index) {
            for j in i {
                let (ct, rt) = to_col_row(j);
                if field_state[rt][ct] == chr {
                    return false;
                }
            }
        }
    }

    true
}
