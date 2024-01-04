use rand::Rng;
use std::io::{self, Read};

struct Board {
    board: [i64; 16],
}

impl Board {
    fn new() -> Self {
        Board {
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
    fn spawn(&mut self) {
        let mut rng = rand::thread_rng();
        let num: i64 = rng.gen_range(0..101);
        let mut index: usize = rng.gen_range(0..15);

        while self.board[index] != 0 {
            index = rng.gen_range(0..15);
        }

        if num > 90 {
            self.board[index] = 4;
        } else {
            self.board[index] = 2;
        }
    }
    fn compress(&self, direction: char, row: Vec<i64>) -> Vec<i64> {
        let mut compressed = vec![];
        let mut i = 0;

        while i < row.len() {
            if i + 1 < row.len() && row[i] == row[i + 1] && row[i] != 0 {
                compressed.push(row[i] * 2);
                i += 1;
            } else if row[i] != 0 {
                compressed.push(row[i]);
            }
            i += 1;
        }
        while compressed.len() != 4 {
            if "nw".contains(direction) {
                compressed.push(0);
            } else {
                compressed.insert(0, 0);
            }
        }
        compressed
    }
    fn mov(&mut self, direction: char) {
        if "ns".contains(direction) {
            for c in 0..4 {
                let mut col = Vec::new();

                for n in 0..4 {
                    let idx = 4 * n + c;
                    col.push(self.board[idx]);
                }
                let compressed_col = self.compress(direction, col);
                for n in 0..4 {
                    let idx = 4 * n + c;
                    self.board[idx] = compressed_col[n];
                }
            }
        } else if "ew".contains(direction) {
            for r in 0..4 {
                let mut row = Vec::new();

                for n in 0..4 {
                    let idx = 4 * r + n;
                    row.push(self.board[idx]);
                }
                let compressed_row = self.compress(direction, row);
                for n in 0..4 {
                    let idx = 4 * r + n;
                    self.board[idx] = compressed_row[n];
                }
            }
        }
    }
    fn check_win(&self) -> bool {
        self.board.contains(&2048)
    }
}

fn print_array(arr: [i64; 16]) {
    for (i, &elem) in arr.iter().enumerate() {
        print!("{} ", elem);
        if (i + 1) % 4 == 0 {
            println!();
        }
    }
    println!();
}

fn main() {
    let mut game_board = Board::new();
    let mut input = String::new();
    let mut direction: char;

    loop {
        game_board.spawn();
        print_array(game_board.board);
        io::stdin().read_line(&mut input).expect("Failed");
        direction = input.trim().chars().next().unwrap();
        game_board.mov(direction);
        input = "".to_string();
        if game_board.check_win() {
            break;
        }
    }

    println!("You Won!");
}
