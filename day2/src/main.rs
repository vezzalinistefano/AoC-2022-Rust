use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "items_list";

fn main() {
    // Open the file for reading the items list
    let file = File::open(FILE_NAME).unwrap();
    let reader = BufReader::new(file);

    let mut total_calories: u32 = 0;
    let mut v_calories = Vec::new();

    // Read the file checking for empty lines
    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            v_calories.push(total_calories);
            total_calories = 0;
        } else {
            if let Ok(n) = line.trim().parse::<u32>() {
                total_calories = total_calories + n;
            }
        }
    }
    v_calories.sort();
    v_calories.reverse();

    println!("---------------");
    println!("Top 3 number of calories is:",);

    let mut answer: u32 = 0;
    for e in &v_calories[..3] {
        println!("{}", e);
        answer += e;
    }

    println!("Answer is: {}", answer);
}
