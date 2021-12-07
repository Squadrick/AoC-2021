use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut count: i32 = 0;
    let mut prev: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    while let Some(line) = lines.next() {
        let value: i64 = line.unwrap().trim().parse().unwrap();
        if value > prev {
            count += 1;
        }
        prev = value;
    }
    println!("Count: {}", count);
}
