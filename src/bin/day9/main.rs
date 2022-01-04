fn part1(heightmap: &Vec<Vec<usize>>) -> usize {
   let mut row: usize = 0;
   let mut risk_sum: usize = 0;

   let max_row: usize = heightmap.len();

   for heights in heightmap {
       let max_col: usize = heights.len();
       let mut column: usize = 0;
       for height in heights {
           if (row == 0 || height < &heightmap[row - 1][column]) &&
              (row == max_row  - 1 || height < &heightmap[row + 1][column]) &&
              (column == 0 || height < &heightmap[row][column - 1]) &&
              (column == max_col - 1 || height < &heightmap[row][column + 1])
           {
                risk_sum += height + 1;
           }
           column += 1;
       }
       row += 1;
   }
   risk_sum
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    let mut heightmap: Vec<Vec<usize>> = Vec::new();
    for line in input {
        let heights: Vec<usize> = line.chars()
                                      .map(|x| x.to_string()
                                                .parse::<usize>().unwrap())
                                      .collect();
        heightmap.push(heights);
    }
    println!("Part 1: Answer is {}", part1(&heightmap));
//    println!("Part 2: Answer is {}", part2(&heightmap));
}
