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