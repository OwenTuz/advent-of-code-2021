fn part1(input: &Vec<Vec<&str>>) -> usize {
    input.iter()
         .map(|n| n.iter().filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7).count())
         .sum()
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    let mut _patterns: Vec<Vec<&str>> = Vec::new();
    let mut outputs: Vec<Vec<&str>> = Vec::new();

    for line in input {
        let split: Vec<&str> = line.split(" | ")
                                   .collect();
        _patterns.push(split[0].split_whitespace().collect());
        outputs.push(split[1].split_whitespace().collect());
    }
    println!("Part 1: Answer is {}", part1(&outputs));
    //println!("Part 2: Answer is {}", part2(&patterns, &outputs));
}
