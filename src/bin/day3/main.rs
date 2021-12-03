fn part1(input: &Vec<&str>) -> u32 {
    let mut ones_count = vec![0; input[0].len()];
    let length: usize = input.len();

    for entry in input {
        for (i, c) in entry.chars().enumerate() {
            if c == '1' {
                ones_count[i] += 1
            }
        }
    }

    let mut gamma: u32 = 0;
    for i in ones_count {
        let most_common: u32 = (i > (length - i)) as u32;
        gamma = gamma << 1 | most_common;
    }

    let mask: u32 = 0b111111111111;
    let epsilon: u32 = (gamma & !mask) | (!gamma & mask);

    return gamma * epsilon
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
//    println!("Part 2: Answer is {}", part2(&input));
}
