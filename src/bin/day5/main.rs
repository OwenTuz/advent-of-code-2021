use std::collections::BTreeMap;

struct Point {
    x: usize,
    y: usize,
}

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
    let mut x = line.start.x;
    let mut y = line.start.y;

    loop {
            *grid.entry((x,y)).or_insert(0) += 1;

            if x == line.end.x && y == line.end.y {
                break
            }
            if line.end.x > x { x += 1 };
            if line.end.x < x { x -= 1 };
            if line.end.y > y { y += 1 };
            if line.end.y < y { y -= 1 };
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
    println!("Part 2: Answer is {}", part2(&input));
}
