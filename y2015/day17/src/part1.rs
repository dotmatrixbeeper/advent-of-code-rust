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