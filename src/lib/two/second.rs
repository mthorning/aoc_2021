enum Direction {
    Up,
    Down,
    Forward,
}

struct Instruction {
    direction: Direction,
    units: i32,
}

impl Instruction {
    fn from(input: &Vec<String>) -> Vec<Instruction> {
        input
            .iter()
            .map(|horizontal| {
                let parts: Vec<&str> = horizontal.split(" ").collect();
                Instruction {
                    direction: Self::get_direction(parts[0]),
                    units: parts[1].parse::<i32>().unwrap(),
                }
            })
            .collect()
    }

    fn get_direction(direction: &str) -> Direction {
        match direction {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Input includes unknown instruction"),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            aim: 0,
            horizontal: 0,
            depth: 0,
        }
    }

    fn plot_route(&mut self, instructions: &Vec<Instruction>) {
        for Instruction { direction, units } in instructions {
            match direction {
                Direction::Up => self.aim -= units,
                Direction::Down => self.aim += units,
                Direction::Forward => {
                    self.horizontal += units;
                    self.depth += self.aim * units;
                }
            }
        }
    }

    fn multiply(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn second() {
    let input = crate::utils::get_input("two");
    let instructions = Instruction::from(&input);

    let mut submarine = Submarine::new();
    submarine.plot_route(&instructions);
    println!("{:?}", submarine.multiply());
}
