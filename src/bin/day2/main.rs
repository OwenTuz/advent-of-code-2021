struct Sub {
    x: i32, // horizontal position
    z: i32, // vertical position, aka depth
    aim: i32, // angle we're pointing at, roughly
}

impl Sub {
    // Initial movement for part1
    fn travel(&mut self, direction: &str, distance: i32) {
        match direction {
            "forward" => self.x += distance,
            "down"    => self.z += distance,
            "up"      => self.z -= distance,
            _         => panic!("Unknown direction for sub move: {}", direction),
        }
    }

    // Updated movement controls for part2
    // now called do_cmd because not every command actually moves the sub
    fn do_cmd(&mut self, command: &str, distance: i32) {
        match command {
            "forward" => {
                self.x += distance;
                self.z += self.aim * distance;
            },
            "down"    => self.aim += distance,
            "up"      => self.aim -= distance,
            _         => panic!("Unknown command for sub move: {}", command),
        }
    }
}

fn part1(input: &Vec<&str>) -> i32 {
    let sub = &mut Sub { x: 0, z: 0, aim: 0 };
    for step in input {
        let mut cmd = step.split_whitespace();

        let direction: &str = cmd.nth(0).unwrap();
        let distance: i32 = cmd.nth(0).unwrap()
                               .parse().unwrap();

        sub.travel(direction, distance);
    }
    sub.x * sub.z
}

fn part2(input: &Vec<&str>) -> i32 {
    let sub = &mut Sub { x: 0, z: 0, aim: 0 };
    for step in input {
        let mut cmd = step.split_whitespace();

        let direction: &str = cmd.nth(0).unwrap();
        let distance: i32 = cmd.nth(0).unwrap()
                               .parse().unwrap();

        sub.do_cmd(direction, distance);
    }
    sub.x * sub.z
}

fn main() {
    let input: Vec<&str> = util::input_to_str_vec(include_str!("input"));
    println!("Part 1: Answer is {}", part1(&input));
    println!("Part 2: Answer is {}", part2(&input));
}
