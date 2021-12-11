use std::io::BufRead;

fn step(board: &mut Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> u32 {
    if visited[i][j] {
        return 0;
    }
    if i == 0 || j == 0 || i == board.len() - 1 || j == board[i].len() - 1 {
        return 0;
    }
    board[i][j] += 1;
    if board[i][j] > 9 {
        visited[i][j] = true;
        board[i][j] = 0;
        return 1
            + step(board, visited, i, j + 1)
            + step(board, visited, i, j - 1)
            + step(board, visited, i - 1, j - 1)
            + step(board, visited, i - 1, j)
            + step(board, visited, i - 1, j + 1)
            + step(board, visited, i + 1, j - 1)
            + step(board, visited, i + 1, j)
            + step(board, visited, i + 1, j + 1);
    }
    return 0;
}

fn simulate(board: &mut Vec<Vec<i32>>) -> u32 {
    let mut score = 0;
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    for i in 1..board.len() - 1 {
        for j in 1..board[i].len() - 1 {
            score += step(board, &mut visited, i, j);
        }
    }
    return score;
}

fn add_boundary(board: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_board = vec![vec![i32::MIN; board[0].len() + 2]; board.len() + 2];
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
    let mut board: Vec<Vec<i32>> = Vec::new();
    while let Some(line) = lines.next() {
        board.push(
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        );
    }
    let mut new_board = add_boundary(&board);
    let sync_score = board.len() as u32 * board[0].len() as u32;
    let mut i = 1;
    loop {
        let score = simulate(&mut new_board);
        if score == sync_score {
            break;
        }
        i += 1;
    }
    println!("{}", i);
}
