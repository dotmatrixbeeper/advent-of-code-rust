mod part1;
mod part2;

pub const RANGE: [(u32, u32); 5] = [(1, 200_000), (200_001, 400_000), (400_001, 600_000), (600_001, 800_000), (800_001, 1_000_000)];

fn main() {
    part1::solve("ckczppom");
    part2::solve("ckczppom");
}
