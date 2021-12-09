use std::io::BufRead;

fn get_risk(board: &Vec<Vec<u32>>, low_points: &Vec<(usize, usize)>) -> u32 {
    return low_points
        .iter()
        .map(|point| board[point.0][point.1])
        .sum::<u32>()
        + low_points.len() as u32;
}

fn find_low_points(board: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for i in 1..board.len() - 1 {
        for j in 1..board[0].len() - 1 {
            let elem = board[i][j];
            if elem < board[i + 1][j]
                && elem < board[i][j + 1]
                && elem < board[i - 1][j]
                && elem < board[i][j - 1]
            {
                low_points.push((i, j));
            }
        }
    }
    return low_points;
}

fn fill(board: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    if board[i][j] != 9 {
        board[i][j] = 9;
        return 1
            + fill(board, i + 1, j)
            + fill(board, i - 1, j)
            + fill(board, i, j + 1)
            + fill(board, i, j - 1);
    }
    return 0;
}

fn find_basins(board: &mut Vec<Vec<u32>>, low_points: &Vec<(usize, usize)>) -> u32 {
    let mut basins: Vec<u32> = low_points.iter().map(|p| fill(board, p.0, p.1)).collect();
    basins.sort();
    return basins.iter().rev().take(3).product();
}

fn add_boundary(board: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut new_board = vec![vec![9; board[0].len() + 2]; board.len() + 2];
    for i in 1..=board.len() {
        for j in 1..=board[0].len() {
            new_board[i][j] = board[i - 1][j - 1];
        }
    }
    return new_board;
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut board: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next() {
        let row: Vec<u32> = line
            .unwrap()
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        board.push(row);
    }
    let mut new_board = add_boundary(&board);
    let low_points = find_low_points(&new_board);
    let risk: u32 = get_risk(&new_board, &low_points);
    println!("Risk: {}", risk);
    let basin_size = find_basins(&mut new_board, &low_points);
    println!("Basin: {}", basin_size);
}
