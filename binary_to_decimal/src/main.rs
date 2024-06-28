fn main() {
    let input = "110110111";
    let result = operations(input);

    println!("Binary to Decimal");
    println!("{} = {}", input, result);
}

fn operations(num: &str) -> i64 {
    let mut decimal = 0;
    let mut base = 1;

    for digit in num.chars().rev() {
        if digit == '1' {
            decimal += base;
        }
        base *= 2;
    }

    decimal
}