use std::io::BufRead;

fn build_gen(fish: &Vec<i32>) -> Vec<u64> {
    let mut gen = vec![0; 9];
    for f in fish {
        gen[*f as usize] += 1;
    }
    return gen;
}

fn evolve(gen: &mut Vec<u64>) {
    let parents = gen[0];
    for i in 1..gen.len() {
        gen[i - 1] = gen[i];
    }
    gen[6] += parents;
    gen[8] = parents;
}

fn main() {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let fish: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
    let mut gen = build_gen(&fish);

    for _i in 0..256 {
        evolve(&mut gen);
    }
    println!("{:?}", gen.iter().sum::<u64>());
}
