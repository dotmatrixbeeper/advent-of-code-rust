use std::{env, path::PathBuf};

const YEARS_DAYS: [(u16, (u16, u16)); 2] = [(2015, (1, 25)), (2016, (1, 11))];

fn main() {
    let args = env::args().collect::<Vec<String>>();
    
    if args.len() == 2 && args[1] == "all"{
        run_all();
    } else if args.len() == 5 && (args[1] == "--year" && args[3] == "--day") {
        let year = args[2].parse::<u16>().expect("Invalid year");
        let day = args[4].parse::<u16>().expect("Invalid day");
        run_specific_day(year, day);
    } else {
        eprintln!("Usage:\n{} --year <year> --day <day>\nOR\n{} all", args[0], args[0]);
        std::process::exit(1);
    }  
}

fn run_all() {
    YEARS_DAYS.iter()
        .for_each(|(year, (first_day, last_day))| {
            (*first_day..=*last_day)
                .for_each(|day| run_specific_day(*year, day));
        });
}

fn run_specific_day(year: u16, day: u16) {
    let input = read_input(year, day);
    match (year, day) {
        (2015, 1) => solutions::y2015::day01::run(&input),
        (2015, 2) => solutions::y2015::day02::run(&input),
        (2015, 3) => solutions::y2015::day03::run(&input),
        (2015, 4) => solutions::y2015::day04::run(&input),
        (2015, 5) => solutions::y2015::day05::run(&input),
        (2015, 6) => solutions::y2015::day06::run(&input),
        (2015, 7) => solutions::y2015::day07::run(&input),
        (2015, 8) => solutions::y2015::day08::run(&input),
        (2015, 9) => solutions::y2015::day09::run(&input),
        (2015, 10) => solutions::y2015::day10::run(&input),
        (2015, 11) => solutions::y2015::day11::run(&input),
        (2015, 12) => solutions::y2015::day12::run(&input),
        (2015, 13) => solutions::y2015::day13::run(&input),
        (2015, 14) => solutions::y2015::day14::run(&input),
        (2015, 15) => solutions::y2015::day15::run(&input),
        (2015, 16) => solutions::y2015::day16::run(&input),
        (2015, 17) => solutions::y2015::day17::run(&input),
        (2015, 18) => solutions::y2015::day18::run(&input),
        (2015, 19) => solutions::y2015::day19::run(&input),
        (2015, 20) => solutions::y2015::day20::run(&input),
        (2015, 21) => solutions::y2015::day21::run(&input),
        (2015, 22) => solutions::y2015::day22::run(&input),
        (2015, 23) => solutions::y2015::day23::run(&input),
        (2015, 24) => solutions::y2015::day24::run(&input),
        (2015, 25) => solutions::y2015::day25::run(&input),
        (2016, 1) => solutions::y2016::day01::run(&input),
        (2016, 2) => solutions::y2016::day02::run(&input),
        (2016, 3) => solutions::y2016::day03::run(&input),
        (2016, 4) => solutions::y2016::day04::run(&input),
        (2016, 5) => solutions::y2016::day05::run(&input),
        (2016, 6) => solutions::y2016::day06::run(&input),
        (2016, 7) => solutions::y2016::day07::run(&input),
        (2016, 8) => solutions::y2016::day08::run(&input),
        (2016, 9) => solutions::y2016::day09::run(&input),
        (2016, 10) => solutions::y2016::day10::run(&input),
        (2016, 11) => solutions::y2016::day11::run(&input),
        _ => {
            eprintln!("No solution avaliable for {}/{}", year, day);
            std::process::exit(1);
        }
    }
}

fn get_input_path(year: u16, day: u16) -> PathBuf {
    PathBuf::from(format!("inputs/{}/day{:02}.txt", year, day))
}

fn read_input(year: u16, day: u16) -> String {
    std::fs::read_to_string(get_input_path(year, day))
        .unwrap_or_else(|_| panic!("Failed to read input for {}/{}", year, day))
}