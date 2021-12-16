type Grid = [[i32; 5]; 5]; 
#[derive(Debug)]
struct Board {
    numbers: Grid,
    matched: Grid,
}

impl Board {
    fn from(numbers: [[i32; 5]; 5]) -> Board {
        Board {
            numbers,
            matched: [[-1; 5]; 5],
        }
    }
}

#[derive(Debug)]
struct Bingo {
    balls: Vec<i32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn new() -> Bingo {
        Bingo {
            balls: Vec::new(),
            boards: Vec::new(),
        }
    }

    fn set_up_new(&mut self, input: &Vec<String>) {
        let mut split_inputs = input.split(|x| *x == "");

        let balls_str: Vec<&str> = split_inputs.next().unwrap()[0].split(",").collect();

        self.balls = balls_str
            .into_iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        while let Some(grid_nums) = split_inputs.next() {
            let mut grid: [[i32; 5]; 5] = [[-1; 5]; 5];

            for (x, row) in grid_nums.iter().enumerate() {
                for (y, cell) in row.split_whitespace().enumerate() {
                    if let Ok(num) = cell.parse::<i32>() {
                        grid[y][x] = num;
                    }
                }

            }
            self.boards.push(Board::from(grid));
        }
    }
}

pub fn first() {
    let input = crate::utils::get_input("four");
    let mut game = Bingo::new();
    game.set_up_new(&input);
}
