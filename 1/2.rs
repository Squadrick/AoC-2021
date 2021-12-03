use std::io::BufRead;
use std::collections::VecDeque;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines();
  let mut count: i32 = 0;
  let mut triplet: VecDeque<i64> = VecDeque::new();
  
  for _ in 0..3 {
    let value: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    triplet.push_back(value);
  }

  let mut prev: i64 = triplet.iter().sum();

  while let Some(line) = lines.next() {
    let value: i64 = line.unwrap().trim().parse().unwrap();
    triplet.pop_front();
    triplet.push_back(value);
    let sum = triplet.iter().sum();

    if sum > prev {
      count += 1;
    }
    prev = sum;
  }
  println!("Count: {}", count);
}
