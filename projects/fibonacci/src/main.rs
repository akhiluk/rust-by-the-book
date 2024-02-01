use std::io;

fn main() {
    println!("NTH FIBONACCI NUMBER\n====================");

    println!("Enter a number.");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read user input from keyboard.");

    let input: usize = input.trim().parse().expect("Enter a valid number!");

    find_fib(input);
}

fn find_fib(input: usize) {
    let mut vector = vec![0,1];

    for i in 2..=input {
        let result = vector[i-2] + vector[i-1];
        vector.push(result);
    }

    println!("The {input}th element of the Fibonacci sequence is: {}", vector[input-1]);
    println!("{:?}", vector);
}
