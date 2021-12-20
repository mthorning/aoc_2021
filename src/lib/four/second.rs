use super::first::*;

pub fn second() {
    let winners = &run("four");
    let winner = &winners[winners.len() -1];
    match (winner.score, winner.final_called_ball) {
        (Some(score), Some(called)) => println!("{}", score * called),
        _ => panic!("Something gone wrong"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_winning_score() {
        let winners = crate::four::first::run("four_test");
        assert_eq!(winners[winners.len() - 1].score, Some(148));
    }

    #[test]
    fn correct_called() {
        let winners = crate::four::first::run("four_test");
        assert_eq!(winners[winners.len() - 1].final_called_ball, Some(13));
    }
}
