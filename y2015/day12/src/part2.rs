/// ### Solution for Part 2
/// In part two we need to care a little bit about the structure
/// of the JSON. Thus we use serde, to read the JSON object.
/// Then we filter out all the "red" objects and do the sum again.
/// 
/// #### Rust Implementation Details
/// We iterate over the objects in the JSON recursively, avoiding 
/// "red" containing objects and arrays and sum the numbers found.
use serde_json::Value;

pub fn solve(input: &str) {
    let value: Value = serde_json::from_str(input).unwrap();

    println!("Sum of all numbers minus reds: {}", evaluate(&value));    
}

fn evaluate(object: &Value) -> i64 {
    match object {
        Value::Number(number) => return number.as_i64().unwrap(),
        Value::Array(values) => {
            let mut sum = 0;
            for val in values {
                sum += evaluate(val);
            }
            return sum;
        },
        Value::Object(map) => {
            let mut sum = 0;
            for (_, value) in map {
                if value == "red" {
                    return 0;
                } else {
                    sum += evaluate(value);
                }
            }
            return sum;
        },
        _ => return 0
    }
}