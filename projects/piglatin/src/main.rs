fn main() {
    println!("Welcome to the Pig Latin converter!");
    println!("Please enter a word:");

    let mut input = String::new();
    
    std::io::stdin()
    .read_line(&mut input)
    .expect("Could not read input from keyboard!");

    let mut proper_input = String::new();
    for c in input.chars() {
        let b = c.to_lowercase().to_string();
        if c != '\r' || c != '\n' {
            proper_input.push_str(&b);
        }
    }

    proper_input.pop();
    proper_input.pop();

    println!("{} in pig-latin is: {}", proper_input, make_pig_latin(proper_input.clone()));
}

fn make_pig_latin(s: String) -> String {
    let mut first_char = ' ';
    let mut final_string = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for c in s.chars() {
        first_char = c;
        break;
    }

    if vowels.contains(&first_char) {
        let suffix = format!("{}-hay", s);
        final_string.push_str(&suffix);
    } else {
        for c in s.chars() {
            if c == first_char {
                continue;
            } else {
                final_string.push(c);
            }
        }
        let suffix = format!("-{}ay", first_char);
        final_string.push_str(&suffix);
    }

    return final_string;
}