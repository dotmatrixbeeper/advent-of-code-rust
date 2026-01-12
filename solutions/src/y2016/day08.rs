use std::fmt::Display;

pub fn run(input: &str) {
    println!("======= DAY 8 =======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

struct Screen {
    width: usize,
    height: usize,
    values: Vec<Vec<bool>>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Screen {
        let values = vec![vec![false; width]; height];
        Self { width, height, values }
    }

    fn lit_pixel_count(&self) -> usize {
        let mut count = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if self.values[i][j] {
                    count += 1;
                }
            }
        }
        count
    }

    fn light(&mut self, x: usize, y: usize) {
        self.values[x][y] = true;
    }

    fn replace_row(&mut self, row: usize, replacement: Vec<bool>) {
        self.values[row] = replacement;
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", if self.values[i][j] { '#' } else { ' ' })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    const WIDTH: usize = 50;
    const HEIGHT: usize = 6;

    let mut screen = Screen::new(WIDTH, HEIGHT);
    input
        .lines()
        .for_each(|line| {
            let line_parts = line.split(" ").collect::<Vec<&str>>();
            match line_parts[0] {
                "rect" => {
                    let coordinates = line_parts[1].split("x").collect::<Vec<&str>>();
                    for i in 0..(screen.height.min(coordinates[1].parse::<usize>().unwrap())) {
                        for j in 0..(screen.width.min(coordinates[0].parse::<usize>().unwrap())) {
                            screen.light(i, j);
                        }
                    }
                },
                "rotate" => {
                    match line_parts[1] {
                        "row" => {
                            let (y, delta) = (get_y(line_parts[2]), line_parts[4].parse::<usize>().unwrap());
                            let mut lights = vec![false; WIDTH];
                            for i in 0..WIDTH {
                                let position = (i + delta) % WIDTH;
                                lights[position] = screen.values[y][i];
                            }
                            screen.replace_row(y, lights);
                        },
                        _ => {
                            let (x, delta) = (get_x(line_parts[2]), line_parts[4].parse::<usize>().unwrap());
                            let mut lights = vec![false; HEIGHT];
                            for i in 0..HEIGHT {
                                let position = (i + delta) % HEIGHT;
                                lights[position] = screen.values[i][x];
                            }
                            (0..HEIGHT)
                                .for_each(|i| screen.values[i][x] = lights[i]);
                        },
                    }
                },
                _ => {
                    eprintln!("Error! Invalid instruction: {}", line);
                    std::process::exit(1);
                }
            };
        });

    println!("{}", screen);
    println!("Lit pixel count: {}", screen.lit_pixel_count());
    println!("---------------------");
}

/// Turn `y=a` to a
fn get_y(y_value: &str) -> usize {
    return y_value[2..].parse::<usize>().unwrap()
}

/// Turn `x=a` to a
fn get_x(x_value: &str) -> usize {
    return x_value[2..].parse::<usize>().unwrap()
}
fn part_2(_: &str) {
    println!("Part 2 --------------");
    println!("Look up!");
    println!("---------------------");
}