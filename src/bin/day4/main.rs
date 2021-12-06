#[derive(Clone,Debug)]
struct BingoBoard {
    numbers: Vec<usize>,
    called_map: usize,
    sum_unmarked: usize,
}

impl BingoBoard {
    fn has_won(&self) -> bool {
        let win_states: Vec<usize> = vec![
            0b1111100000000000000000000, // first row complete
            0b0000011111000000000000000, // second row...
            0b0000000000111110000000000,
            0b0000000000000001111100000,
            0b0000000000000000000011111,
            0b1000010000100001000010000, // first column complete...
            0b0100001000010000100001000,
            0b0010000100001000010000100,
            0b0001000010000100001000010,
            0b0000100001000010000100001,
        ];
        return win_states.iter().any(|x| x & self.called_map == *x)
    }

    fn call(&mut self, number: &usize) {
        if let Some(pos) = self.numbers.iter().position(|n| n == number) {
            self.called_map = self.called_map | (0b1000000000000000000000000 >> pos);
            self.sum_unmarked = self.sum_unmarked - number;
        }
    }
}

fn parse_boards(input: &str) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();
    let grids: Vec<usize> = input.replace("\n\n", " ")
                                 .replace("\n", " ")
                                 .split_whitespace()
                                 .map(|n| n.parse::<usize>().unwrap())
                                 .collect();

    for grid in grids.chunks(25) {
        let board = BingoBoard {
            numbers: grid.iter().map(|x| *x).collect::<Vec<usize>>(),
            called_map: 0,
            sum_unmarked: grid.iter().map(|x| *x).sum(),
        };
        boards.push(board);
    }
    return boards
}

fn part1(numbers: &Vec<usize>, boards: &Vec<BingoBoard>) -> usize {
    let mut b = boards.clone();
    for number in numbers {
        for board in b.iter_mut() {
            board.call(number);
            if board.has_won() {
                return board.sum_unmarked * number
            }
        }
    }
    panic!("No board won");
}

fn part2(numbers: &Vec<usize>, boards: &Vec<BingoBoard>) -> usize {
    let mut wins: usize = 0;
    let num_boards = boards.len();

    let mut b = boards.clone();
    for number in numbers {
        for board in b.iter_mut() {
            if board.has_won() {
                continue
            } else {
                board.call(number);
                if board.has_won() {
                    wins += 1;
                    if wins == num_boards {
                        return board.sum_unmarked * number
                    }
                }
            }
        }
    }
    panic!("No board won");
}

fn main() {
    let (numbers, boards): (&str, &str) = include_str!("input").split_once('\n').unwrap();
    let numbers: Vec<usize> = util::input_to_vec_t_fromstr(numbers, ',');
    let boards: Vec<BingoBoard> = parse_boards(boards);

    println!("Part 1: Answer is {}", part1(&numbers, &boards));
    println!("Part 2: Answer is {}", part2(&numbers, &boards));
}
