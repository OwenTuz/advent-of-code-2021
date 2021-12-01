fn part1(input: &Vec<i32>) -> i32 {
    let depths = input.windows(2);
    let res: i32 = depths.fold(0, |acc, x| if x[1] > x[0] {acc + 1 } else {acc});
   res
}

fn part2(input: &Vec<i32>) -> i32 {
    let sums: Vec<i32> = input.windows(3).map(|x| x[0] + x[1] + x[2]).collect();
    return part1(&sums);
}

fn main() {
    let input: Vec<i32> = util::input_to_vec_t_fromstr(include_str!("input"), '\n');
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}
