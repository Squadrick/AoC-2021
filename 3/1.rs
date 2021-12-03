use std::io::BufRead;
use std::collections::HashMap;

fn main() {
  let stdin = std::io::stdin();
  let mut lines = stdin.lock().lines();
  let mut bin_count = HashMap::new();
  let mut num_lines = 0;
  
  while let Some(line) = lines.next() {
    num_lines += 1;
    let line_unwrapped = line.unwrap();
    let s: Vec<&str> = line_unwrapped.trim().split("").collect();

    for (i, &b) in s[1..s.len()-1].iter().enumerate() {
      let bcount = bin_count.entry(i).or_insert(0);
      if b == "1" {
        *bcount += 1;
      }
    }
  }

  let mut keys: Vec<&usize> = bin_count.keys().collect();
  keys.sort();
  let mut gamma_rate = String::new();
  let mut eps_rate = String::new();

  for key in keys.iter() {
    let one_count = bin_count.get(key).unwrap();
    if one_count > &(num_lines / 2) {
      gamma_rate.push_str("1");
      eps_rate.push_str("0");
    } else {
      gamma_rate.push_str("0");
      eps_rate.push_str("1");
    }
  }
  let int_gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
  let int_eps_rate = isize::from_str_radix(&eps_rate, 2).unwrap();
  println!("{:?}", int_gamma_rate * int_eps_rate);
}
