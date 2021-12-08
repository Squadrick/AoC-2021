use std::io::BufRead;

fn run(board: &mut Vec<u8>, points: &[i32], max: i32, diag: bool) -> usize {
    let mut count = 0;

    for idx in 0..points.len() / 4 {
        let x0 = points[4 * idx + 0];
        let y0 = points[4 * idx + 1];
        let x1 = points[4 * idx + 2];
        let y1 = points[4 * idx + 3];
        let (dx, dy) = (x1 - x0, y1 - y0);
        let r = dx.abs().max(dy.abs());
        let (dx, dy) = (dx.signum(), dy.signum());

        if (!diag && (dy == 0 || dx == 0)) || (diag && dx != 0 && dy != 0) {
            (0..=r).for_each(|i| {
                let pos = ((x0 + dx * i) + max * (y0 + dy * i)) as usize;
                board[pos] += 1;
            });
        }
    }
    for e in board {
        if e >= &mut 2 {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut points: Vec<i32> = Vec::new();
    while let Some(line) = lines.next() {
        let mut tokens: Vec<i32> = line
            .unwrap()
            .trim()
            .split(|c: char| !c.is_ascii_digit())
            .filter(|&s| s != "")
            .map(|s| s.parse().unwrap())
            .collect();
        points.append(&mut tokens);
    }
    let max = points.iter().max().unwrap_or(&0) + 1;

    let mut board = vec![0u8; (max * max) as usize];
    let step1 = run(&mut board, &points, max, false);
    let step2 = run(&mut board, &points, max, true);
    println!("{}, {}", step1, step2);
}
