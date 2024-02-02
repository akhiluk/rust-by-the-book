#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };

    area(&rect1);

    println!("{:#?}", rect1);
}

fn area(input: &Rectangle) {
    // Passing the struct instance as a reference here
    // because we don't want to change ownership.
    println!("The area of the rectangle is: {}", input.height * input.width);
}
