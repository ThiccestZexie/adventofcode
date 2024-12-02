use std::fs::File;
use std::io::{self, BufRead};
use std::mem::swap;
use std::path::Path;


fn main() -> std::io::Result<()> {
    let path = "src/input.txt";

    let file = File::open(path)?;

    let reader = io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
    let line = line?; // Handle errors and unwrap the line
    let parts: Vec<&str> = line.split_whitespace().collect(); // create a list of current line

    let mut prev = 0;
    let mut isSafe = false;
    let mut first_loop = true;
        for &part in &parts {
            let  number: i32 = part.parse().unwrap();
            let diff = (number - prev).abs();
            println!("curr {:?}, prev {:?}, diff {:?}", number, prev, diff);
            if diff <=3 && diff >= 1 && prev != 0  && (number > prev || number < prev){
                isSafe = true;
            }
            else if first_loop ==false  {
                isSafe = false;
                continue
            }
            first_loop = false;
            prev = number;
        }


        if isSafe == true{

            total += 1;
        }
    }

    println!("Total: {}", total);
    Ok(())
}
