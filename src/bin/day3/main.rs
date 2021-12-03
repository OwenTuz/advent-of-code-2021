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

enum RatingType {
    Oxygen,
    CO2,
}

fn find_rating(input: &Vec<u32>, rating_type: RatingType, pos: usize) -> u32 {
    let (one, zero): (Vec<u32>, Vec<u32>) = input
                      .iter()
                      .partition(|x| (**x >> pos) & 1 == 1);
    let next: &Vec<u32>;
    match rating_type {
        RatingType::Oxygen => {
            next = if one.len() >= zero.len() { & one } else { &zero }
        },
        RatingType::CO2 => {
            next = if one.len() < zero.len() { &one } else { &zero }
        }
    }
    if next.len() == 1 {
        return next[0]
    }
    return find_rating(next, rating_type, pos - 1)
}

fn part2(input: &Vec<&str>) -> u32 {
    let initial_pos = input[0].len() - 1;
    let input_b: Vec<u32> = input.iter()
                                    .map(|x| u32::from_str_radix(x, 2).unwrap())
                                    .collect();
    let oxygen: u32 = find_rating(&input_b, RatingType::Oxygen, initial_pos);
    let co2: u32 = find_rating(&input_b, RatingType::CO2, initial_pos);
    return oxygen * co2
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}
