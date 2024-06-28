use std::io;

fn main() {
    println!("Choose any one operations : 1-> Addition, 2->Subtraction, 3-> Multiplication, 4-> Division, 5-> Modulus");
    let mut input = String::new();
    let mut first = String::new();
    let mut second = String::new();

    
    io::stdin().read_line(&mut input).expect("Error");
    io::stdin().read_line(&mut first).expect("Error");
    io::stdin().read_line(&mut second).expect("Error");
    
    let int_input : i8 = input.trim().parse().unwrap();
    let first_input : i64 = first.trim().parse().unwrap();
    let second_input : i64 = second.trim().parse().unwrap();
    operations(int_input,first_input,second_input);

}

fn operations(input:i8,first:i64,second:i64) {
    match input {
        1 => println!("Output of addition is {}",first+second),
        2 => println!("Output of subtraction is {}",first-second),
        3 => println!("Output of multiplication is {}",first*second),
        4 => println!("Output of division is {}",first/second),
        5 => println!("Output of modulus is {}",first%second),
        _ => println!("Wrong input")

    }
    
}