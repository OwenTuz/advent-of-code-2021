fn step(fishmap: &mut [usize;9]) {
    let reproducing: usize = fishmap[0];
    for i in 1..9 {
        fishmap[i-1] = fishmap[i];
    }
    fishmap[8] = reproducing;
    fishmap[6] += reproducing;
}

fn part1(fishmap: &[usize;9]) -> usize {
    let mut fish = fishmap.clone();
    for _i in 0..80 {
        step(&mut fish);
    }
    return fish.iter().sum()
}

fn part2(fishmap: &[usize;9]) -> usize {
    let mut fish = fishmap.clone();
    for _i in 0..256 {
        step(&mut fish);
    }
    return fish.iter().sum()
}

fn main() {
    let input: Vec<usize> = util::input_to_vec_t_fromstr(include_str!("input"), ',');
    let mut fishmap: [usize;9] = [0;9];
    for i in input {
        fishmap[i] += 1;
    }
    println!("Part 1: Answer is {}", part1(&fishmap));
    println!("Part 2: Answer is {}", part2(&fishmap));
}
