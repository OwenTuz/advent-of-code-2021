use std::collections::HashSet;

fn part1(input: &Vec<Vec<&str>>) -> usize {
    input.iter()
         .map(|n| n.iter().filter(|n| n.len() == 2 || n.len() == 3 || n.len() == 4 || n.len() == 7).count())
         .sum()
}

fn pop_if<T: Clone, F: Fn(&&T) -> bool + Copy>(input: &mut Vec<T>, predicate: F) -> T {
    let mut results = input.iter().filter(predicate);
    let result = results.nth(0).unwrap().clone();
    // safety check, panic if predicate matches too many results
    assert!(results.nth(0).is_none());
    input.retain(|x| !predicate(&x));
    result
}

fn part2(patterns: &Vec<Vec<&str>>, outputs: &Vec<Vec<&str>>) -> usize {
    let mut sum: usize = 0;
    for (entry, items) in patterns.iter().enumerate() {
        let mut pattern_sets: Vec<HashSet<char>> =
            items.iter()
                 .map(|x| x.chars().collect::<HashSet<char>>())
                 .collect();
        let no_1 = pop_if(&mut pattern_sets, |x| x.len() == 2);
        let no_4 = pop_if(&mut pattern_sets, |x| x.len() == 4);
        let no_7 = pop_if(&mut pattern_sets, |x| x.len() == 3);
        let no_8 = pop_if(&mut pattern_sets, |x| x.len() == 7);

        let no_3 = pop_if(&mut pattern_sets, |x| (&no_1 ^ *x).len() == 3);
        let no_9 = pop_if(&mut pattern_sets, |x| &(&no_3 | &no_4) == *x);
        let no_5 = pop_if(&mut pattern_sets, |x|
            x.len() == 5 && (&no_4 ^ &no_1).is_subset(x)
        );
        let no_6 = pop_if(&mut pattern_sets, |x| &(*x | &no_5) == *x);
        let no_0 = pop_if(&mut pattern_sets, |x| x.len() == 6);
        let no_2 = pop_if(&mut pattern_sets, |x| x.len() == 5);

        let p = vec![ no_0, no_1, no_2, no_3, no_4, no_5, no_6, no_7, no_8, no_9 ];

        let mut output: usize = 0;
        for digit in outputs[entry].clone() {
            let d_set: HashSet<char> = digit.chars().collect();
            let d: usize = p.iter().position(|x| x == &d_set).unwrap();
            if output == 0 { output += d } else { output = output * 10 + d };
        }
        sum += output
    }
    sum
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    let mut patterns: Vec<Vec<&str>> = Vec::new();
    let mut outputs: Vec<Vec<&str>> = Vec::new();

    for line in input {
        let split: Vec<&str> = line.split(" | ")
                                   .collect();
        patterns.push(split[0].split_whitespace().collect());
        outputs.push(split[1].split_whitespace().collect());
    }
    println!("Part 1: Answer is {}", part1(&outputs));
    println!("Part 2: Answer is {}", part2(&patterns, &outputs));
}
