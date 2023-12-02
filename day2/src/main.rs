use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input1.txt")?;
    let reader = BufReader::new(f);

    let mut sum: u32 = 0;
    let mut counter: u32 = 1;

    let mut sum2: u32 = 0;

    for line in reader.lines() {
        let l = line?;
        let s_line2: Vec<&str> = l.split(':').collect();
        let games = s_line2[1].trim().split(';')
            .map(|x| x.trim()).collect::<Vec<_>>();

        //let games_details: Vec<HashMap<&str,u32>>;
        let mut h = HashMap::from([
            ("red", 0), 
            ("green", 0), 
            ("blue", 0)]
        );
        
        for g in games {
            let game_colors: Vec<_> = g.split(',').map(|x| x.trim()).collect();
            
            for gc in game_colors {
                let color = gc.split(' ').nth(1).expect("msg");
                let v: u32 = gc.split(' ').nth(0).expect("y").parse().unwrap();
                if h[color] < v {
                    h.insert(color, v);
                }
            }
            
            //games_details.append();
        }

        println!("{} {} {}", h["red"], h["green"], h["blue"]);

        if h["red"] <= 12 && h["green"] <= 13 && h["blue"] <= 14 {
            sum += counter;
        }

        sum2 += h["red"] * h["green"] * h["blue"];

        counter += 1;
        //println!("!{}", games[1])
    }

    println!();
    println!("{}", sum);
    println!("{}", sum2);
    Ok(())
}

// 3510 too high
// left in an if statement that meant only red mattered
// 2331 correct

// part2: 71585 correct