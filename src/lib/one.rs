pub fn first() {
    let input = get_input();
    let increases = count_increases(&input);
    println!("{}", increases);
}

pub fn second() {
    let input = get_input();
    let increases = count_summed_increases(&input);
    println!("{}", increases);
}

fn get_input() -> Vec<usize> {
    crate::utils::get_input("one")
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn count_increases(input: &Vec<usize>) -> usize {
    let mut increases = 0;

    for (i, num) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if num > &input[i - 1] {
            increases += 1;
        }
    }
    increases
}

fn count_summed_increases(input: &Vec<usize>) -> usize {
    let mut increases = 0;
    let mut previous_sum = 0;

    for i in 1..input.len() - 2 {
        let sum = &input[i] + &input[i + 1] + &input[i + 2];
        if sum > previous_sum {
            increases += 1;
        }
        previous_sum = sum;
    }
    increases
}

#[test]
fn test_count_summed_increases() {
    let input = vec![607, 618, 618, 617, 647, 716, 769, 792];
    assert_eq!(count_summed_increases(&input), 5);
}
