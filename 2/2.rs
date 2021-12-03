use std::io::BufRead;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines();
  let mut _x: i32 = 0;
  let mut _y: i32 = 0;
  let mut _aim: i32 = 0;

  while let Some(line) = lines.next() {
    let line_unwrapped = line.unwrap();
    let s: Vec<&str> = line_unwrapped.trim().split(" ").collect();
    let direction = s[0];
    let delta: i32 = s[1].parse::<i32>().unwrap();
    
    match direction {
      "forward" => {
          _x += delta;
          _y += _aim * delta;
      },
      "down" => _aim += delta,
      "up" => _aim -= delta,
      _ => panic!("error direction"),
    }

  }
  println!("{}", _x * _y);
}
