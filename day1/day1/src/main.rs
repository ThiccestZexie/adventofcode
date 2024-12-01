use std::fs::File;
use std::io::{self, BufRead};
use std::mem::swap;
use std::path::Path;

// Bubble sort function
fn bubble_sort(arr: &mut Vec<i32>) {
    let mut n = arr.len();
    while n > 1 {
        let mut new_n = 0;
        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_n = i;
            }
        }
        n = new_n;
    }
}

fn main() -> io::Result<()> {
    let path = "src/input.txt";

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    let mut left_numbers = Vec::new();
    let mut right_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle errors and unwrap the line
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            // Parse and push the numbers into respective vectors
            if let (Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left_numbers.push(left);
                right_numbers.push(right);
            } else {
                eprintln!("Failed to parse line: {}", line);
            }
        } else {
            eprintln!("Unexpected line format: {}", line);
        }
    }


    bubble_sort(&mut left_numbers);
    bubble_sort(&mut right_numbers);


    // check similarity
    let mut total = 0;
    for i in 0..left_numbers.len() {
        for j in 0..right_numbers.len() {


        }
    }
    println!("{}", total);
    Ok(())
}
