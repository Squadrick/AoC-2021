use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

fn board_iter(
    board: &Vec<i32>,
    moves: &HashSet<i32>,
    idxr: &dyn Fn(usize, usize) -> usize,
) -> bool {
    for i in 0..5 {
        let mut count: i32 = 0;
        for j in 0..5 {
            let elem = board[idxr(i, j)];
            if moves.contains(&elem) {
                count += 1;
            }
        }
        if count == 5 {
            return true;
        }
    }
    return false;
}

fn unmarked_sum(board: &Vec<i32>, moves: &HashSet<i32>) -> i32 {
    let mut sum = 0;
    for i in board {
        if !moves.contains(i) {
            sum += i
        }
    }
    sum
}

fn bingo(board: &Vec<i32>, moves: &[i32]) -> i32 {
    let moves_set: HashSet<i32> = HashSet::from_iter(moves.iter().cloned());

    // go over 5 rows
    if board_iter(&board, &moves_set, &|i, j| i + 5 * j) {
        return unmarked_sum(&board, &moves_set);
    }

    // go over 5 cols
    if board_iter(&board, &moves_set, &|i, j| 5 * i + j) {
        return unmarked_sum(&board, &moves_set);
    }
    return -1;
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let moves_line = lines.next().unwrap().unwrap();
    let moves: Vec<i32> = moves_line
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<i32>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.unwrap() != "" {
            continue;
        }

        let mut board: Vec<i32> = Vec::new();
        for _ in 0..5 {
            let board_line = lines.next().unwrap().unwrap();
            let board_row_split = board_line.trim().split(" ");
            for elem in board_row_split {
                if elem != "" {
                    board.push(elem.parse().unwrap());
                }
            }
        }
        boards.push(board);
    }

    let mut excluded_boards: HashSet<usize> = HashSet::new();
    'outer: for i in 1..moves.len() {
        for board_num in 0..boards.len() {
            if excluded_boards.contains(&board_num) {
                continue;
            }
            let board_sum = bingo(&boards[board_num], &moves[..=i]);
            if board_sum != -1 {
                if excluded_boards.len() == boards.len() - 1 {
                    println!("{} * {} = {}", board_sum, moves[i], board_sum * moves[i]);
                    break 'outer;
                }
                excluded_boards.insert(board_num);
            }
        }
    }
}
