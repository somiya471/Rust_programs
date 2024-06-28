use std::io;

fn main() {
    println!("Perform Temperature conversion");

    println!("Choose any one conversion : 1-> Fahrenheit to Celsius, 2-> Celsius to Fahrenheit");
    let mut input = String::new();
    let mut degree = String::new();

    
    io::stdin().read_line(&mut input).expect("Error");
    io::stdin().read_line(&mut degree).expect("Error");

    let int_input : i8 = input.trim().parse().unwrap();
    let first_input : i64 = degree.trim().parse().unwrap();
    operations(int_input,first_input);

}

fn operations(input:i8,first:i64) {
    match input {
        1 => println!("Fahrenheit to Celsius is {}",(first - 32) / 2),
        2 => println!("Celsius to Fahrenheit is {}",(first * 2) + 32),
        _ => println!("Wrong input")

    }
    
}