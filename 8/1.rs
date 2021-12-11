use std::io::BufRead;

fn get_count(line: &str) -> usize {
    return line
        .split(" ")
        .map(|s| s.trim().len())
        .filter(|&l| l == 7 as usize || l == 4 as usize || l == 3 as usize || l == 2 as usize)
        .count();
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut count = 0;
    while let Some(line) = lines.next() {
        let line_unwrapped = line.unwrap();
        let segments: Vec<&str> = line_unwrapped.split("|").map(|s| s.trim()).collect();
        count += get_count(segments[1]);
    }
    println!("{}", count);
}
