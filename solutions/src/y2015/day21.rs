use regex::Regex;

mod part1;
mod part2;

struct Item {
    cost: i16,
    damage: i16,
    armor: i16
}

pub fn run(input: &str) {
    println!("======= DAY 21 =======");
    let (hp, dmg, armor) = get_boss_stats(input);
    let weapons = vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0
        }
    ];

    let armors = vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5
        }
    ];

    let rings = vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3
        }
    ];

    part1::solve(&weapons, &armors, &rings, hp, dmg, armor);
    part2::solve(&weapons, &armors, &rings, hp, dmg, armor);
    println!("======================\n");
}

fn get_boss_stats(input: &str) -> (i16, i16, i16) {
    let hp_re = Regex::new(r"Hit Points: (?<hp>\d+)").unwrap();
    let dmg_re = Regex::new(r"Damage: (?<dmg>\d+)").unwrap();
    let armor_re = Regex::new(r"Armor: (?<armor>\d+)").unwrap();
    let mut lines = input.lines();
    let hp = hp_re.captures(lines.next().unwrap()).unwrap()["hp"].parse::<i16>().unwrap();
    let dmg = dmg_re.captures(lines.next().unwrap()).unwrap()["dmg"].parse::<i16>().unwrap();
    let armor = armor_re.captures(lines.next().unwrap()).unwrap()["armor"].parse::<i16>().unwrap();
    
    (hp, dmg, armor)
}
