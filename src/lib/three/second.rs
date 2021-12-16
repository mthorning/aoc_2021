use super::first;

struct LifeSupport {
    o2: i32,
    co2: i32,
}

impl LifeSupport {
    fn new() -> LifeSupport {
        LifeSupport { o2: 0, co2: 0 }
    }

    fn print_rating(&self) {
        println!("{}", self.o2 * self.co2);
    }
}

pub fn second() {
    let input = crate::utils::get_input("three");
    let report: Vec<Vec<i32>> = first::get_report(&input)
        .iter()
        .map(|x| x.iter().map(|y| y.parse::<i32>().unwrap()).collect())
        .collect();

    let mut life_support = LifeSupport::new();

    if let Some(output) = reduce_down(report.clone(), 0, |x, y| x == y) {
        life_support.o2 = get_decimal(output);
    }

    if let Some(output) = reduce_down(report, 0, |x, y| x != y) {
        life_support.co2 = get_decimal(output);
    }

    life_support.print_rating();
}

fn reduce_down<F>(report: Vec<Vec<i32>>, i: usize, test: F) -> Option<Vec<i32>>
where
    F: Fn(i32, i32) -> bool,
{
    let mut filtered_report = Vec::new();
    for bits in report.iter() {
        if test(bits[i], one_is_dominant(&report, i) as i32) {
            filtered_report.push(bits.to_owned())
        }
    }

    match filtered_report.len() {
        0 => None,
        1 => Some(filtered_report[0].to_owned()),
        _ => reduce_down(filtered_report, i + 1, test),
    }
}

fn one_is_dominant(report: &Vec<Vec<i32>>, i: usize) -> bool {
    let mut total = 0;
    for bits in report.iter() {
        total += bits[i];
    }
    total as f32 >= (report.len() as f32) / 2.0
}

fn get_decimal(numbers: Vec<i32>) -> i32 {
    let stringified_output: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    first::get_decimal(stringified_output.join("").as_str())
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn gets_decimal() {
        assert_eq!(get_decimal(vec![1,0,1,0,1,0]), 42);
    }

    #[test]
    fn finds_one_is_dominant_even() {
        let report = vec![
            vec![1,0,1],
            vec![1,0,1],
            vec![1,0,0],
            vec![0,1,0],
        ];
        assert!(one_is_dominant(&report, 0));
        assert!(!one_is_dominant(&report, 1));
        assert!(one_is_dominant(&report, 2));
    }

    #[test]
    fn finds_one_is_dominant_odd() {
        let report = vec![
            vec![1,0],
            vec![1,0],
            vec![0,1],
        ];
        assert!(one_is_dominant(&report, 0));
        assert!(!one_is_dominant(&report, 1));
    }
}
