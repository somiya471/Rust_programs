fn main() {
    let input = 78;
    let result = operations(input);

    println!("Decimal to Octal");
    println!("{} = {}", input, result);
}

fn operations(mut num: i64) -> i64 {
    let mut octal = 0;
    let mut i = 1;

    while num !=0{
        octal = octal + (num % 8) * i;
        num = num / 8;
        i = i * 10;

    }

    octal
}