use std::fs;

type Board = [[u8; 9]; 9];

fn main() {
    let mut board = load_file("grilles/facile.txt");
    println!("Grille initiale :");
    print_board(&board);

    if solve(&mut board, 0) {
        println!("\nSolution trouvée :");
        print_board(&board);
    } else {
        println!("Pas de solution.");
    }
}

fn load_file(path: &str) -> Board {
    let mut board = [[0u8; 9]; 9];
    let contents = fs::read_to_string(path).expect("Impossible de lire le fichier");

    for (r, line) in contents.lines().enumerate().take(9) {
        let tokens: Vec<&str> = line.split('|').collect();
        for (c, t) in tokens.iter().enumerate().take(9) {
            board[r][c] = match *t {
                "." => 0,
                x => x.parse().unwrap_or(0),
            };
        }
    }
    board
}

fn print_board(board: &Board) {
    for row in board {
        for &cell in row {
            print!(
                "{} ",
                if cell == 0 {
                    '.'
                } else {
                    (b'0' + cell) as char
                }
            );
        }
        println!();
    }
}

fn is_valid(board: &Board, r: usize, c: usize, val: u8) -> bool {
    for i in 0..9 {
        if board[r][i] == val || board[i][c] == val {
            return false;
        }
    }
    // bloc 3×3
    let br = (r / 3) * 3;
    let bc = (c / 3) * 3;
    for i in br..br + 3 {
        for j in bc..bc + 3 {
            if board[i][j] == val {
                return false;
            }
        }
    }
    true
}

fn solve(board: &mut Board, idx: usize) -> bool {
    if idx == 81 {
        return true;
    }
    let r = idx / 9;
    let c = idx % 9;

    if board[r][c] != 0 {
        return solve(board, idx + 1);
    }

    for val in 1..=9 {
        if is_valid(board, r, c, val) {
            board[r][c] = val;
            if solve(board, idx + 1) {
                return true;
            }
            board[r][c] = 0; // backtrack
        }
    }

    false
}
