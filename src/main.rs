use std::env;

mod day1;
mod day2;
mod day3;
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
    match day_result {
        Ok(day) => match day {
            1 => day1::main(),
            2 => day2::main(),
            _ => eprintln!("No such day ({})", day),
        },
        Err(e) => eprintln!("Could not parse input {:?}", e),
    }
}
