pub fn solve() {
    let mut house_count = 0;

    let mut calculated_sum = 0;
    while calculated_sum < 33100000 {
        house_count += 1;
        calculated_sum = factorize(house_count) * 11;
    }

    println!("House Number: {house_count}");
}

fn factorize(n: u32) -> u32 {

    let val = (1..=(n.isqrt()))
        .filter(|i| n % i == 0 )
        .map(|i| {
            if n == i * i {
                return 0;
            } else {
                return n / i;
            }
        })
        .filter(|i| n <= i * 50)
        .sum::<u32>();

    let val2 = (1..=(n.isqrt()))
        .filter(|i| n % i == 0 && n / i <= 50)
        .sum::<u32>();

    val + val2
}