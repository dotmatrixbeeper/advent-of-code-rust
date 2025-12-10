use regex::Regex;

mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 22 =======");
    let (hp, dmg) = get_boss_stats(input);
    part1::solve(hp, dmg);
    part2::solve(hp, dmg);
    println!("======================\n");
}

fn get_boss_stats(input: &str) -> (i32, i32) {
    let hp_re = Regex::new(r"Hit Points: (?<hp>\d+)").unwrap();
    let dmg_re = Regex::new(r"Damage: (?<dmg>\d+)").unwrap();
    let mut lines = input.lines();
    let hp = hp_re.captures(lines.next().unwrap()).unwrap()["hp"].parse::<i32>().unwrap();
    let dmg = dmg_re.captures(lines.next().unwrap()).unwrap()["dmg"].parse::<i32>().unwrap();

    (hp, dmg)
}