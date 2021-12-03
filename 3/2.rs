use std::io::BufRead;
use std::convert::TryInto;

fn oxy_bit(one_count: i32, total_count: i32) -> char {
  if 2 * one_count >= total_count {
    return '1';
  } else {
    return '0';
  }
}

fn co2_bit(one_count: i32, total_count: i32) -> char {
  match oxy_bit(one_count, total_count) {
    '0' => return '1',
    _ => return '0',
  }
}

fn filter(input: &Vec<String>, bit_field: usize, f: &dyn Fn(i32, i32)->char) -> Vec<String> {
  let mut output = Vec::<String>::new();
  let mut one_count = 0;
  for i in input.iter() {
    if i.chars().nth(bit_field).unwrap() == '1' {
      one_count += 1;
    }
  }
  let parity = f(one_count, input.len().try_into().unwrap());
  for i in input.iter() {
    if i.chars().nth(bit_field).unwrap() == parity {
      output.push(i.to_string());
    }
  }
  return output;
}

fn get_rating(input: &Vec<String>, f: &dyn Fn(i32, i32)->char) -> isize {
  let mut input_copy = input.clone();
  let mut bit_field = 0;
  loop {
    let lines_new = filter(&input_copy, bit_field, f);
    if lines_new.len() == 1 {
      return isize::from_str_radix(&lines_new[0], 2).unwrap();
    }
    bit_field += 1;
    input_copy = lines_new;
  }
}

fn main() {
  let stdin = std::io::stdin();
  let lines: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
  let oxy_rate = get_rating(&lines, &oxy_bit);
  let co2_rate = get_rating(&lines, &co2_bit);
  println!("{}", oxy_rate * co2_rate);
}
