pub fn first() {
    let input = get_input();
    let increases = count_increases(&input);
    println!("{}", increases);
}

pub fn get_input() -> Vec<i32> {
    crate::utils::get_input("one")
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn count_increases(input: &Vec<i32>) -> i32 {
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
