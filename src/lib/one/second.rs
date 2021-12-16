pub fn second() {
    let input = super::first::get_input();
    let increases = count_summed_increases(&input);
    println!("{}", increases);
}

fn count_summed_increases(input: &Vec<i32>) -> i32 {
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
