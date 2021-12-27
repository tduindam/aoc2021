use std::env;
use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod neighbors;
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
            6 => day6::main(),
            7 => day7::main(),
            8 => day8::main(),
            9 => day9::main(),
            10 => day10::main(),
            11 => day11::main(),
            14 => day14::main(),
            15 => day15::main(),
            _ => eprintln!("No such day ({})", day),
        },
        Err(e) => eprintln!("Could not parse input {:?}", e),
    }
    println!("Running took {} ms", now.elapsed().as_millis());
}
