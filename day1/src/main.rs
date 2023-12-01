use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main_part1() -> io::Result<()> {
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);

    let mut strings = Vec::new();
    let mut digits: Vec<u32> = Vec::new();

    let mut counter = 0;
    let mut last_val = ' ';
    let mut is_first = true;

    for line in reader.lines() {
        strings.push(String::new());
        for charz in line?.chars() {
            if charz.is_numeric() {
                last_val = charz;
                if is_first {
                    strings[counter].push(charz);
                    is_first = false;
                }
                
                //println!("{}", strings[counter]);
            }
        }
        strings[counter].push(last_val);
        println!("{}",strings[counter].parse::<u32>().unwrap());

        digits.push(strings[counter].parse::<u32>().unwrap());

        counter += 1;
        is_first = true;
        
    }

    let sum: u32 = digits.iter().sum();

    println!("{}", sum);

    Ok(())
}

// 54573 correct

// =====================

fn main() -> io::Result<()> {
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);

    let mut strings = Vec::new();
    let mut digits: Vec<u32> = Vec::new();

    let mut counter = 0;
    let mut last_val = ' ';
    let mut substring = String::new();
    let mut is_first = true;
    let mut found_digit = false;

    let values: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    for line in reader.lines() {
        strings.push(String::new());
        for charz in line?.chars() {
            if charz.is_numeric() {
                last_val = charz;
                found_digit = true;
                substring = "".to_string();
                //println!("{}", strings[counter]);
            } else {
                substring.push(charz);
                // check each number
                for n in 0..9 {
                    if substring.contains(values[n]) {
                        // yes im going from int back to char and then converting later
                        // dont worry about it
                        last_val = (n + 1).to_string().chars().nth(0).unwrap();
                        found_digit = true;
                        // append the last character of the eaten word
                        // to solve eightwo and oneight. you NEED the second one.
                        substring = charz.to_string();
                    }
                }
                
            }

            if found_digit && is_first {
                strings[counter].push(last_val);
                is_first = false;
            }
        }
        strings[counter].push(last_val);
        //println!("{}",strings[counter].parse::<u32>().unwrap());

        digits.push(strings[counter].parse::<u32>().unwrap());

        println!("{}", digits[counter]);

        counter += 1;
        is_first = true;
        found_digit = false;
        substring = "".to_string();
        last_val = '😀';
    }

    let sum: u32 = digits.iter().sum();

    println!("");
    println!("answer: {}", sum);

    Ok(())
}

// 54623 too high
// 54591 correct