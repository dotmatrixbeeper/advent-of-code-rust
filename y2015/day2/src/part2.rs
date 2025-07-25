use crate::Dimensions;

pub fn solve(data: Vec<Dimensions>) {
    let size: u32 = data.iter()
        .map(| side: &Dimensions | {
            let perimeter: (u32, u32, u32) =  (2 *(side.0 + side.1), 2 * (side.1 + side.2), 2 * (side.2 + side.0));
            let volume = side.0 * side.1 * side.2;
            if perimeter.0 <= perimeter.1 && perimeter.0 <= perimeter.2 {
                return perimeter.0 + volume;
            } else if perimeter.1 <= perimeter.0 && perimeter.1 <= perimeter.2 {
                return perimeter.1 + volume;
            } else {
                return perimeter.2 + volume;
            }
        })
        .sum();
    println!("Total size of wrapping papers requried: {size}")
}