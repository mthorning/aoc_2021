pub fn first() {
    let input = get_input();
    let increases = count_increases(&input);
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
