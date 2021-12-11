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

fn get_closing_score(c: char) -> u32 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
}

fn get_score(line: &str) -> u32 {
    let mut stack = Vec::<char>::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else {
            let matcher = stack.pop().unwrap();
            if get_closing(matcher) != c {
                return get_closing_score(c);
            }
        }
    }
    return 0;
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut score = 0;
    while let Some(line) = lines.next() {
        score += get_score(&line.unwrap());
    }
    println!("{}", score);
}
