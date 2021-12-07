fn part1(input: &Vec<usize>) -> usize {
    let mut crabs: Vec<usize> = input.clone();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    crabs.iter().fold(
        0, |acc, x| if median >= *x { acc + (median - x) }
                    else { acc + (x - median ) }
    )
}

fn main() {
    let input: Vec<usize> = util::input_to_vec_t_fromstr(include_str!("input"), ',');
    println!("Part 1: Answer is {}", part1(&input));
//    println!("Part 2: Answer is {}", part2(&input));
}
