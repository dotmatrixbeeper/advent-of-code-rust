pub fn run(input: &str) {
    println!("======= DAY 16 ======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    let mut lines = input.lines();
    let random = lines.next().unwrap();
    let disk_size = lines.next().unwrap().parse::<u32>().unwrap();
    let mut checksum = dragon_curve(random, disk_size);
    while checksum.len().is_multiple_of(2) {
        checksum = checksum.chunks(2).map(| ch | if ch[0] == ch[1] { '1' } else { '0' }).collect::<Vec<char>>();
    }
    println!("Checksum for the dragon curve: {}", checksum.iter().collect::<String>());
    println!("---------------------");
}

fn dragon_curve(random: &str, disk_size: u32) -> Vec<char> {
    let mut curve_data = random.chars().collect::<Vec<char>>();
    while curve_data.len() < disk_size as usize {
        let mut inverted_data = curve_data 
            .iter()
            .map(|&c| 
                if c == '0' { 
                    '1'
                } else { 
                    '0' 
            })
            .collect::<Vec<char>>();
        inverted_data.reverse();
        curve_data.push('0');
        curve_data.append(&mut inverted_data);
    }
    return curve_data[..disk_size as usize].to_vec();
}

fn part_2(input: &str) {
    println!("Part 2 --------------");
    let mut lines = input.lines();
    let random = lines.next().unwrap();
    let disk_size = 35651584;
    let mut checksum = dragon_curve(random, disk_size);
    while checksum.len().is_multiple_of(2) {
        checksum = checksum.chunks(2).map(| ch | if ch[0] == ch[1] { '1' } else { '0' }).collect::<Vec<char>>();
    }
    println!("Checksum for the dragon curve: {}", checksum.iter().collect::<String>());
    println!("---------------------");
}