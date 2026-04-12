use regex::Regex;

pub fn run(input: &str) {
    println!("======= DAY 15 ======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

struct Disc {
    slots: u8,
    start: u8 
}

impl Disc {
    fn new(slots: u8, start: u8) -> Self {
        Disc { slots, start }
    }

    fn position_at(&self, time: usize) -> u8 {
        (((time % self.slots as usize) + self.start as usize) % self.slots as usize) as u8
    }
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    let discs = extract_discs(input);
    let mut n: usize = 0;

    loop {
        let mut travelled = 0;
        for i in 0..discs.len() {
            if discs[i].position_at(n + i + 1) == 0 {
                travelled += 1;
            }
        }

        if travelled == discs.len() {
            println!("Earliest release time: {}", n);
            break;
        }

        n += 1;
    } 
    println!("---------------------");
}

fn extract_discs(input: &str) -> Vec<Disc> {
    let re = Regex::new(r"Disc #\d+ has (?<slots>\d+) positions; at time=0, it is at position (?<position>\d+).").unwrap();
    input.lines()
        .map(| line | {
            let captures = re.captures(line).unwrap();
            Disc::new(
                captures["slots"].parse::<u8>().unwrap(),
                captures["position"].parse::<u8>().unwrap()
            )
        })
        .collect()
}

fn part_2(input: &str) {
    println!("Part 2 --------------");
    let mut discs = extract_discs(input);
    discs.push(Disc::new(11, 0));
    let mut n: usize = 0;

    loop {
        let mut travelled = 0;
        for i in 0..discs.len() {
            if discs[i].position_at(n + i + 1) == 0 {
                travelled += 1;
            }
        }

        if travelled == discs.len() {
            println!("Earliest release time: {}", n);
            break;
        }

        n += 1;
    } 
    println!("---------------------");
}