enum Direction {
    Up,
    Down,
    Forward,
}

struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn from(input: &Vec<String>) -> Vec<Instruction> {
        input
            .iter()
            .map(|x| {
                let parts: Vec<&str> = x.split(" ").collect();
                Instruction {
                    direction: Self::get_direction(parts[0]),
                    distance: parts[1].parse::<usize>().unwrap(),
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
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new() -> Coordinates {
        Coordinates { x: 0, y: 0 }
    }

    fn plot_route(&mut self, instructions: &Vec<Instruction>) {
        for Instruction {
            direction,
            distance,
        } in instructions
        {
            match direction {
                Direction::Up => self.y -= distance,
                Direction::Down => self.y += distance,
                Direction::Forward => self.x += distance,
            }
        }
    }

    fn multiply(&self) -> usize {
       self.x * self.y 
    }
}

pub fn first() {
    let input = crate::utils::get_input("two");
    let instructions = Instruction::from(&input);

    let mut coordinates = Coordinates::new();
    coordinates.plot_route(&instructions);
    println!("{:?}", coordinates.multiply());
}
