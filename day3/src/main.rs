use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);

    let mut sum: u32 = 0;
    let mut counter: u32 = 0;
    let mut sum2: u32 = 0;

    // load everything into a single 2D array
    let manual: Vec<Vec<char>> = reader.lines().map(|x| x.expect("whatevs").chars().collect()).collect();

    //println!("{}", manual[0][7]);

    //manual[0][0].mat

    // go through and check 
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut in_number: bool = false;
    let mut cur_number: String = "".to_string();
    let mut is_part: bool = false;

    // y is row starting at top
    // x is col

    //println!("{}", manual[0].len());

    while y < manual.len().try_into().unwrap() {
        while x < manual[0].len().try_into().unwrap() {

            if manual[a(y)][a(x)].is_numeric() {
                in_number = true;

                cur_number.push(manual[a(y)][a(x)]);

                // check around 
                for cx in (x-1)..(x+1+1) {
                    for cy in (y-1)..(y+1+1) {
                        

                        if !(cx == x && cy == y) {

                            
                            if check_symbol(manual.clone(), cx, cy) {
                                is_part = true;
                                println!("{} {} {}", cy, cx, check_symbol(manual.clone(), cx, cy));
                            }

                        }
                    }
                }
            } else {
                
                in_number = false;
            }

            x += 1;

            if x >= manual[0].len().try_into().unwrap() {
                in_number = false;
            }

            if !in_number {

                if is_part {
                    sum += cur_number.parse::<u32>().unwrap();
                }
                cur_number = "".to_string();
                is_part = false;
                in_number = true;
            }
        }
        x = 0;
        y += 1;
        
    }

    println!();
    println!("{}", sum);
    println!("{}", sum2);
    Ok(())
}


// 536202 correct


fn a(a: i32) -> usize {
    return usize::try_from(a).unwrap();
}


fn check_symbol(manual: Vec<Vec<char>>, x: i32, y: i32) -> bool {

    let r: bool;

    if x < 0 || y < 0 || y >= manual.len().try_into().unwrap() || x >= manual[0].len().try_into().unwrap() {
        r = false;
        return r;
    } 


    if manual[a(y)][a(x)].is_numeric() || manual[a(y)][a(x)] == '.' {
        r = false;
    } else {
        r = true;
    }

    return r;

}