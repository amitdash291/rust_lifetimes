use std::io;

pub fn test_add() {
    let mut input = String::new();

    println!("Input the first number: ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read first number");
    let a = parse_string(&input);
    println!("Input: {}, a: {}", input.trim(), a);

    println!("Input the second number: ");
    input.clear();
    io::stdin().read_line(&mut input)
        .expect("Failed to read second number");
    let b = parse_string(&input);
    println!("Input: {}, b: {}", input.trim(), b);

    println!("The sum of {} and {} is {}", a, b, add(a, b));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn parse_string(input:&str) -> i32 {
    let failure = -1;
    input.trim().parse::<i32>().unwrap_or(failure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_string_to_i32() {
        let test = "4";
        assert_eq!(parse_string("  45 \n"), 45);
    }
}