const BIT_LENGTH: usize = 12;

pub fn first() {
    let report = crate::utils::get_input("three");
    let report_as_bits: Vec<Vec<&str>> = report
        .iter()
        .map(|x| x.split("").filter(|x| *x != "").collect())
        .collect();

    let mut totals: [i32; BIT_LENGTH] = [0; BIT_LENGTH];
    for bits in report_as_bits.iter() {
        for (i, bit) in bits.iter().enumerate() {
            totals[i] += bit.parse::<i32>().unwrap();
        }
    }

    let gamma_as_bits = totals.map(|x| if x > (report.len() / 2).try_into().unwrap() { "1" } else { "0" });
    let epsilon_as_bits = gamma_as_bits.map(|x| if x == "0" { "1" } else { "0" });

    let gamma = get_decimal(&gamma_as_bits.join("")[..]);
    let epsilon = get_decimal(&epsilon_as_bits.join("")[..]);

    println!("{}", gamma * epsilon);
}

fn get_decimal(binary: &str) -> isize {
    isize::from_str_radix(binary, 2).unwrap()
}

