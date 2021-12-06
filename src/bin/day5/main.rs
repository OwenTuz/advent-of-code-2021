use std::collections::BTreeMap;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn parse_line(input: &str) -> Line {
    let mut raw = input.split(" -> ");
    let start: Vec<usize> = raw.nth(0)
                               .unwrap()
                               .split(",")
                               .map(|n| n.parse::<usize>().unwrap())
                               .collect();
    let end: Vec<usize> = raw.nth(0)
                             .unwrap()
                             .split(",")
                             .map(|n| n.parse::<usize>().unwrap())
                             .collect();
    Line {
        start: Point { x: start[0], y: start[1] },
        end: Point { x: end[0], y: end[1] },
    }
}

fn draw_line(line: &Line, grid: &mut BTreeMap<(usize, usize), usize>) {
    // horizontal and vertical only for initial implementation
    for x in line.start.x..=line.end.x {
        for y in line.start.y..=line.end.y {
            *grid.entry((x,y)).or_insert(0) += 1;
        }
    }
    // lazy hack to get around the fact that rust doesn't do reverse ranges
    for x in line.end.x..=line.start.x {
        for y in line.end.y..=line.start.y {
            *grid.entry((x,y)).or_insert(0) += 1;
        }
    }
}

fn part1(input: &Vec<Line>) -> usize {
   let input: Vec<&Line> = input.iter().filter(
       |line| line.start.x == line.end.x || line.start.y == line.end.y
       ).collect();
   let mut grid: BTreeMap<(usize, usize), usize> = BTreeMap::new();
   for line in input {
       draw_line(line, &mut grid);
   }
   grid.values().fold(0, |acc, x| if x > &1 { acc + 1 } else { acc }) as usize
}

fn part2(input: &Vec<Line>) -> usize {
   let mut grid: BTreeMap<(usize, usize), usize> = BTreeMap::new();
   for line in input {
       draw_line(line, &mut grid);
   }
   grid.values().fold(0, |acc, x| if x > &1 { acc + 1 } else { acc }) as usize
}

fn main() {
    let input: Vec<Line> = util::input_to_str_vec(include_str!("input")).iter()
        .map(|line| parse_line(line))
        .collect();
    println!("Part 1: Answer is {}", part1(&input));
//    println!("Part 2: Answer is {}", part2(&input));
}
