use std::io::BufRead;

fn evolve(fish: &mut Vec<i32>) {
    for i in 0..fish.len() {
        fish[i] -= 1;
    }
    let mut baby_fish = 0;
    for i in 0..fish.len() {
        if fish[i] == -1 {
            fish[i] = 6;
            baby_fish += 1;
        }
    }

    for _ in 0..baby_fish {
        fish.push(8);
    }
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut fish: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
    for _i in 0..80 {
        evolve(&mut fish);
    }
    println!("{}", fish.len());
}
