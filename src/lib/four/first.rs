const GRID_SIZE: usize = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Number {
    Empty,
    Unmarked(i32),
    Marked(i32),
}

type Grid = [[Number; GRID_SIZE]; GRID_SIZE];

#[derive(Debug)]
pub struct Player {
    pub card: Grid,
    pub score: Option<i32>,
    pub final_called_ball: Option<i32>,
}

impl Player {
    fn from(card: Grid) -> Player {
        Player {
            card,
            score: None,
            final_called_ball: None,
        }
    }

    fn check_card(&mut self, called_ball: &i32) -> bool {
        let mut winner = false;
        'outer: for row in self.card.iter_mut() {
            for number in row.iter_mut() {
                match number {
                    Number::Unmarked(num) if num == called_ball => {
                        *number = Number::Marked(*num);

                        if self.has_winning_row() || self.has_winning_column() {
                            winner = true;
                        }
                        break 'outer;
                    }
                    _ => continue,
                };
            }
        }
        winner
    }

    fn has_winning_row(&self) -> bool {
        for row in self.card {
            let mut winning_row = true;
            for num in row {
                if let Number::Marked(_) = num {
                } else {
                    winning_row = false;
                }
            }
            if winning_row {
                return true;
            }
        }
        false
    }

    fn has_winning_column(&self) -> bool {
        for i in 0..GRID_SIZE - 1 {
            let mut winning_column = true;
            for row in self.card {
                if let Number::Marked(_) = row[i] {
                } else {
                    winning_column = false;
                }
            }
            if winning_column {
                return true;
            }
        }
        false
    }
    pub fn calculate_score(&mut self) {
        let mut total = 0;
        for row in self.card.iter() {
            for numbers in row {
                if let Number::Unmarked(num) = numbers {
                    total += num
                }
            }
        }
        self.score = Some(total);
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    NotPlaying,
    Playing,
}

#[derive(Debug)]
pub struct Bingo {
    pub status: Status,
    pub players: Vec<Player>,
    pub winners: Vec<Player>,
    pub balls: Vec<i32>,
}

impl Bingo {
    pub fn from(balls: Vec<i32>, players: Vec<Player>) -> Bingo {
        Bingo {
            balls,
            players,
            winners: Vec::new(),
            status: Status::NotPlaying,
        }
    }

    pub fn call_the_balls(&mut self) {
        self.status = Status::Playing;
        for ball in self.balls.iter() {
            match self.status {
                Status::Playing => {
                    let mut winners: Vec<usize> = Vec::new();
                    for (i, player) in self.players.iter_mut().enumerate() {
                        if player.check_card(&ball) {
                            winners.push(i);
                        }
                    }

                    while let Some(winner) = winners.pop() {
                        let mut winning_player = self.players.remove(winner);
                        winning_player.calculate_score();
                        winning_player.final_called_ball = Some(*ball);
                        self.winners.push(winning_player);
                        if self.players.len() == 0 {
                            self.status = Status::NotPlaying;
                        }
                    }
                }
                Status::NotPlaying => break,
            }
        }
    }
}

pub fn first() {
    let winner = &run("four")[0];
    match (winner.score, winner.final_called_ball) {
        (Some(score), Some(called)) => println!("{}", score * called),
        _ => panic!("Something gone wrong"),
    }
}

pub fn run(input_file: &str) -> Vec<Player> {
    let input = crate::utils::get_input(input_file);
    let (balls, players) = get_balls_and_players(&input);
    let mut game = Bingo::from(balls, players);
    game.call_the_balls();

    game.winners
}

pub fn get_balls_and_players(input: &Vec<String>) -> (Vec<i32>, Vec<Player>) {
    let mut split_inputs = input.split(|x| *x == "");

    let balls_str: Vec<&str> = split_inputs.next().unwrap()[0].split(",").collect();

    let balls = balls_str
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut players = Vec::new();
    while let Some(card_nums) = split_inputs.next() {
        let mut card: Grid = [[Number::Empty; GRID_SIZE]; GRID_SIZE];

        for (x, row) in card_nums.iter().enumerate() {
            for (y, cell) in row.split_whitespace().enumerate() {
                if let Ok(num) = cell.parse::<i32>() {
                    card[x][y] = Number::Unmarked(num);
                }
            }
        }
        players.push(Player::from(card));
    }
    (balls, players)
}

mod tests {
    use super::*;
    use Number::*;

    #[test]
    fn four_first_calls_balls_in_order() {
        let input = crate::utils::get_input("four_test");
        let (balls, players) = get_balls_and_players(&input);
        let game = Bingo::from(balls.clone(), players);
        assert_eq!(game.balls, balls);
    }

    #[test]
    fn four_first_matches() {
        let mut player = Player {
            score: None,
            final_called_ball: None,
            card: [
                [Empty; 5],
                [Empty, Empty, Empty, Unmarked(1), Empty],
                [Empty; 5],
                [Empty, Unmarked(2), Empty, Unmarked(3), Empty],
                [Empty; 5],
            ],
        };
        player.check_card(&1);
        player.check_card(&2);
        player.check_card(&3);
        assert_eq!(player.card[1][3], Marked(1));
        assert_eq!(player.card[3][1], Marked(2));
        assert_eq!(player.card[3][3], Marked(3));
    }

    #[test]
    fn four_first_detects_winning_row() {
        let mut player = Player {
            score: None,
            final_called_ball: None,
            card: [
                [Empty; 5],
                [Marked(0), Marked(0), Marked(0), Empty, Marked(0)],
                [Empty; 5],
                [Empty; 5],
                [Empty; 5],
            ],
        };
        assert!(!player.has_winning_row());
        player.card[1][3] = Marked(0);
        assert!(player.has_winning_row());
    }

    #[test]
    fn four_first_detects_winning_column() {
        let mut player = Player {
            score: None,
            final_called_ball: None,
            card: [
                [Empty, Marked(0), Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Marked(0), Empty, Empty, Empty],
                [Empty, Marked(0), Empty, Empty, Empty],
                [Empty, Marked(0), Empty, Empty, Empty],
            ],
        };
        assert!(!player.has_winning_column());
        player.card[1][1] = Marked(0);
        assert!(player.has_winning_column());
    }

    #[test]
    fn four_first_correct_winning_score() {
        let winners = run("four_test");
        assert_eq!(winners[0].score, Some(188));
    }

    #[test]
    fn four_first_correct_called() {
        let winners = run("four_test");
        assert_eq!(winners[0].final_called_ball, Some(24));
    }
}
