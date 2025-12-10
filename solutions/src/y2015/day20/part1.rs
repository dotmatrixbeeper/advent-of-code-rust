/// ### Solution for Part 1
/// This problem requries us to find the factors for 
/// the house numbers. The factors then can be added 
/// together and multiplied by 10 to find the number
/// of gifts for the house.
/// We need to do this 
/// 
/// #### Rust Implementation Details
/// The main part of the solution is the factorization
/// function. 
/// This function takes the house count and finds
/// the unqiue factors of this house by first 
/// finding the factors of the square root of house count.
/// These are then summed and multiplied by 10 and 
/// checked if it crosses the target.

pub fn solve(data: u32) {
    let mut house_count = 0;

    let mut calculated_sum = 0;
    while calculated_sum < data {
        house_count += 1;
        calculated_sum = factorize(house_count) * 10;
    }

    println!("House Number: {house_count}");
}

fn factorize(n: u32) -> u32 {
    (1..=(n.isqrt()))
        .filter(|i| n % i == 0)
        .map(|i| {
            if n == i * i {
                return 0;
            } else {
                return n / i;
            }
        })
        .sum::<u32>()  
    +
    (1..=(n.isqrt()))
        .filter(|i| n % i == 0)
        .sum::<u32>()
}