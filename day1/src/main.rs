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
    let mut lastVal = ' ';
    let mut isFirst = true;

    for line in reader.lines() {
        strings.push(String::new());
        for charz in line?.chars() {
            if charz.is_numeric() {
                lastVal = charz;
                if isFirst {
                    strings[counter].push(charz);
                    isFirst = false;
                }
                
                //println!("{}", strings[counter]);
            }
        }
        strings[counter].push(lastVal);
        println!("{}",strings[counter].parse::<u32>().unwrap());

        digits.push(strings[counter].parse::<u32>().unwrap());

        counter += 1;
        isFirst = true;
        
    }

    let sum: u32 = digits.iter().sum();

    println!("{}", sum);

    Ok(())
}

fn main() -> io::Result<()> {
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);

    let mut strings = Vec::new();
    let mut digits: Vec<u32> = Vec::new();

    let mut counter = 0;
    let mut lastVal = ' ';
    let mut isFirst = true;

    let values: [String, 9] = ["one", "two", "three"];

    for line in reader.lines() {
        strings.push(String::new());
        for charz in line?.chars() {
            if charz.is_numeric() {
                lastVal = charz;
                if isFirst {
                    strings[counter].push(charz);
                    isFirst = false;
                }
                
                //println!("{}", strings[counter]);
            }
        }
        strings[counter].push(lastVal);
        println!("{}",strings[counter].parse::<u32>().unwrap());

        digits.push(strings[counter].parse::<u32>().unwrap());

        counter += 1;
        isFirst = true;
        
    }

    let sum: u32 = digits.iter().sum();

    println!("{}", sum);

    Ok(())
}
