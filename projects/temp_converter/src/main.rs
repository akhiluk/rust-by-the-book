use std::io;

fn main() {
    println!("TEMPERATURE CONVERTER\n=====================");

    println!("Enter the temperature in Fahrenheit.");

    let mut user_input = String::new();

    io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read user input from keyboard.");

    let user_input: f32 = user_input.trim().parse().expect("Invalid number entered!");

    converter(user_input);
}

fn converter(input: f32) {
    let result = (input - 32 as f32) / 1.80;
    println!("{}° F is {:.2}° C.", input, result);
}
