use std::io;

fn sum(values: &[u32]) -> Option<u32> {
    let mut total: u32 = 0;
    for &value in values {
        if let Some(new_total) = total.checked_add(value) {
            total = new_total;
        } else {
            return None;
        }
    }
    Some(total)
}

fn main() {
    println!("Enter a list of integers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    match sum(&numbers) {
        Some(total) => println!("Sum: {}", total),
        None => println!("The sum resulted in an overflow"),
    }
}
