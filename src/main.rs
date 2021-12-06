use std::env;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod reader;

fn main() {
    let day_result = {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            Ok(1)
        } else {
            args[1].parse::<u32>()
        }
    };

    let now = Instant::now();
    match day_result {
        Ok(day) => match day {
            1 => day1::main(),
            2 => day2::main(),
            3 => day3::main(),
            4 => day4::main(),
            5 => day5::main(),
            _ => eprintln!("No such day ({})", day),
        },
        Err(e) => eprintln!("Could not parse input {:?}", e),
    }
    println!("Running took {} ms", now.elapsed().as_millis());
}