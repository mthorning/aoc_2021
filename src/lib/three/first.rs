const BIT_LENGTH: usize = 12;

pub fn first() {
    let input = crate::utils::get_input("three");
    let report = get_report(&input);

    let mut totals = [0; BIT_LENGTH];
    get_totals(&report, &mut totals);

    let gamma_mask = get_mask(&totals, report.len().try_into().unwrap());
    let epsilon_mask = invert_mask(&gamma_mask);

    let gamma = get_decimal(&gamma_mask.join("")[..]);
    let epsilon = get_decimal(&epsilon_mask.join("")[..]);

    println!("{}", gamma * epsilon);
}

pub fn get_report<'a>(input: &'a Vec<String>) -> Vec<Vec<&'a str>> {
    input
        .iter()
        .map(|x| x.split("").filter(|x| *x != "").collect())
        .collect()
}

fn get_totals(report: &Vec<Vec<&str>>, totals: &mut [i32; BIT_LENGTH]) {
    for bits in report.iter() {
        for (i, bit) in bits.iter().enumerate() {
            totals[i] += bit.parse::<i32>().unwrap();
        }
    }
}

fn get_mask<'a>(totals: &'a [i32; BIT_LENGTH], data_length: i32) -> [&'a str; 12] {
    totals.map(|x| {
        if x >= (data_length / 2).try_into().unwrap() {
            "1"
        } else {
            "0"
        }
    })
}

fn invert_mask<'a>(mask: &[&'a str; 12]) -> [&'a str; 12] {
    mask.map(|x| if x == "0" { "1" } else { "0" })
}

pub fn get_decimal(binary: &str) -> i32 {
    i32::from_str_radix(binary, 2).unwrap()
}
