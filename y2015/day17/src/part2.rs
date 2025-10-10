pub fn solve(container: &Vec<u32>, target: u32) {
    let mut dp = vec![vec![0; target as usize + 1]; container.len() + 1];

    for i in 0..=container.len() {
        dp[i][0] = 1;
    }

    let mut min_size = 0;
    let mut combinations = 0;
    for subset_size in 1..=container.len() {
        for subset in all_subsets_of_size(subset_size, container.len()) {
            if format!("{:b}", subset)
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, c)| *c == '1')
                .map(|(i, _)| container[i])
                .sum::<u32>() == target {
                if min_size == 0 || min_size == subset_size {
                    min_size = subset_size;
                    combinations += 1;
                } else if subset_size > min_size {
                    break;
                }
            }
        }
    }

    println!("Min size to get target: {min_size}");
    println!("combinations with the size: {combinations}");
}

fn all_subsets_of_size(subset_size: usize, container_size: usize) -> impl Iterator<Item = u32> {
    (1..(1u32 << container_size))
        .filter(move | &subset | subset.count_ones() as usize == subset_size)
}