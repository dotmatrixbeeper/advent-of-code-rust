/// ### Solution for Part 1
/// This is subset sum problem: find the number of subsets that 
/// can sum upto the given target.
/// 
/// #### Rust Implementation Details
/// We use a dynamic programming approach to add upto the target 
/// sum. We start with target 0 which can be made in 1 way with 0
/// containers. 
/// We continue for each container and increments of weight
/// until we reach target weight.

pub fn solve(container: &Vec<u32>, target: u32) {
    let mut dp = vec![vec![0; target as usize + 1]; container.len() + 1];

    for i in 0..=container.len() {
        dp[i][0] = 1;
    }

    for i in 1..=container.len() {
        let current_elem = container[i - 1];
        for j in 1..=target as usize {
            dp[i][j] = dp[i - 1][j];

            if current_elem <= j as u32 {
                dp[i][j] = dp[i - 1][j] + dp[i - 1][j - current_elem as usize];
            }
        }
    }

    println!("Combination of containers: {}", dp[container.len()][target as usize]);
}