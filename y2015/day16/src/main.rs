use regex::Regex;

mod part1;
mod part2;

#[derive(Debug)]
pub struct Aunt {
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
    index: usize,
}

impl Aunt {
    fn new() -> Self {
        Self { 
            children: None, 
            cats: None, 
            samoyeds: None, 
            pomeranians: None, 
            akitas: None, 
            vizslas: None, 
            goldfish: None, 
            trees: None, 
            cars: None, 
            perfumes: None,
            index: 0
        }
    }
}

fn main() {
    let data = encode(include_str!("../resources/input.txt"));
    part1::solve(&data);
    part2::solve(&data);
}

fn encode(input: &str) -> Vec<Aunt> {
    let re = Regex::new(r"(children: (?<children>\d+)|cats: (?<cats>\d+)|samoyeds: (?<samoyeds>\d+)|pomeranians: (?<pomeranians>\d+)|akitas: (?<akitas>\d+)|vizslas: (?<vizslas>\d+)|trees: (?<trees>\d+)|cars: (?<cars>\d+)|perfumes: (?<perfumes>\d+)|goldfish: (?<goldfish>\d+))").unwrap();
    let mut aunts = vec![];
    for (index, line) in input.lines().enumerate() {
        let mut aunt = Aunt::new();
        re.captures_iter(line)
            .for_each(|captures| {
                if let Some(children) = captures.name("children") {
                    aunt.children = Some(children.as_str().parse::<u8>().unwrap());
                } else if let Some(cats) = captures.name("cats") {
                    aunt.cats = Some(cats.as_str().parse::<u8>().unwrap());
                } else if let Some(samoyeds) = captures.name("samoyeds") {
                    aunt.samoyeds = Some(samoyeds.as_str().parse::<u8>().unwrap());
                } else if let Some(pomeranians) = captures.name("pomeranians") {
                    aunt.pomeranians = Some(pomeranians.as_str().parse::<u8>().unwrap());
                } else if let Some(akitas) = captures.name("akitas") {
                    aunt.akitas = Some(akitas.as_str().parse::<u8>().unwrap());
                } else if let Some(vizslas) = captures.name("vizslas") {
                    aunt.vizslas = Some(vizslas.as_str().parse::<u8>().unwrap());
                } else if let Some(trees) = captures.name("trees") {
                    aunt.trees = Some(trees.as_str().parse::<u8>().unwrap());
                } else if let Some(cars) = captures.name("cars") {
                    aunt.cars = Some(cars.as_str().parse::<u8>().unwrap());
                } else if let Some(perfumes) = captures.name("perfumes") {
                    aunt.perfumes = Some(perfumes.as_str().parse::<u8>().unwrap());
                } else if let Some(goldfish) = captures.name("goldfish") {
                    aunt.goldfish = Some(goldfish.as_str().parse::<u8>().unwrap());
                }
            }
        );
        aunt.index = index;
        aunts.push(aunt);
    }

    aunts
}