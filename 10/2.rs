use std::io::BufRead;

fn is_opening(c: char) -> bool {
    return c == '(' || c == '[' || c == '{' || c == '<';
}

fn get_closing(c: char) -> char {
    return match c {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => ' ',
    };
}

fn get_closing_score(c: char) -> u64 {
    return match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    };
}

fn get_score(line: &str) -> u64 {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if get_closing(stack.pop().unwrap()) != c {
            return 0;
        }
    }
    return stack
        .iter()
        .rev()
        .fold(0, |acc, c| acc * 5 + get_closing_score(*c));
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut scores: Vec<u64> = Vec::new();
    while let Some(line) = lines.next() {
        let score = get_score(&line.unwrap());
        if score != 0 {
            scores.push(score);
        }
    }
    scores.sort();
    println!("{:?}", scores[scores.len() / 2]);
}
