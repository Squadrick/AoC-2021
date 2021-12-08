use std::io::BufRead;

fn check_distance(crab: &Vec<i32>, pos: i32) -> i32 {
    let mut sum: i32 = 0;
    for c in crab {
        sum += (c - pos).abs();
    }
    return sum;
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let crab: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
    let start = *crab.iter().min().unwrap();
    let end = *crab.iter().max().unwrap();
    let mut min = i32::MAX;
    for i in start..=end {
        let d = check_distance(&crab, i);
        min = min.min(d);
    }
    println!("{:?}", min);
}
