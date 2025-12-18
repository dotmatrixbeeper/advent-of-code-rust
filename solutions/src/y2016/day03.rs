use regex::Regex;

pub fn run(input: &str) {
    println!("======= DAY 3 =======");
    let data = input.lines().collect::<Vec<&str>>();
    part_1(&data);
    part_2(&data);
    println!("=====================\n");
}

/// First part is pretty simple: take each line and find 
/// if the sides can make a valid triangle by checking if 
/// sum of shorter sides is greater than the largest side.
fn part_1(data: &Vec<&str>) {
    println!("Part 1 --------------");
    let re = Regex::new(r"^ *(?<side1>\d+) *(?<side2>\d+) *(?<side3>\d+)$").unwrap();
    let triangle_count = data
        .iter()
        .filter_map(|sides| {
            let caps = re.captures(&sides).unwrap();
            let mut sides = Vec::new();
            sides.push(caps["side1"].parse::<u32>().unwrap());
            sides.push(caps["side2"].parse::<u32>().unwrap());
            sides.push(caps["side3"].parse::<u32>().unwrap());
            
            return is_triangle(&mut sides);
        })
        .count();
    println!("Only {} triangles are possible.", triangle_count);
    println!("---------------------")
}

fn is_triangle(sides: &mut Vec<u32>) -> Option<()> {
    sides.sort();

    if sides[0] + sides[1] > sides[2] {
        return Some(())
    }
    return None;
}

/// part 2 is more of a file reading exercise since it asks to
/// group the sides vertically from the file. 
fn part_2(data: &Vec<&str>) {
    println!("Part 2 --------------");
    let mut col_1_sides = Vec::new();
    let mut col_2_sides = Vec::new();
    let mut col_3_sides = Vec::new();
    let re = Regex::new(r"^ *(?<side1>\d+) *(?<side2>\d+) *(?<side3>\d+)$").unwrap();
    data
        .iter()
        .for_each(|sides| {
            let caps = re.captures(&sides).unwrap();
            col_1_sides.push(caps["side1"].parse::<u32>().unwrap());
            col_2_sides.push(caps["side2"].parse::<u32>().unwrap());
            col_3_sides.push(caps["side3"].parse::<u32>().unwrap());
        });
    let triangle_count = [col_1_sides, col_2_sides, col_3_sides]
        .concat()
        .chunks(3)
        .filter_map(|chunk| {
            let mut sides = Vec::from(chunk);
            return is_triangle(&mut sides);
        })
        .count();
    println!("Only {} triangles are possible.", triangle_count);
    println!("---------------------")
}