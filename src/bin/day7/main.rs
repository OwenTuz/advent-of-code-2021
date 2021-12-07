fn part1(input: &Vec<isize>) -> isize {
    let mut crabs: Vec<isize> = input.clone();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    crabs.iter().fold(0, |acc, x| acc + (median - x).abs())
}

fn part2(input: &Vec<isize>) -> isize {
    let mut crabs: Vec<isize> = input.clone();
    crabs.sort();
    let sum: isize = crabs.iter().sum();
    let mean: isize = sum / crabs.len() as isize;
    crabs.iter().fold(
        0, |acc, x| acc + ((mean - x).abs() * ((mean - x).abs() + 1)) / 2
    )
}

fn main() {
    let input: Vec<isize> = util::input_to_vec_t_fromstr(include_str!("input"), ',');
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}
