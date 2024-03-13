/* To enable this program to read the values of command line arguments
that we pass to it, we'll need the `std::env::args` function.

This function returns an iterator of the command line arguments passed to
the program.

*/

use std::env;
use std::process;

use minigrep::Config;

/*
/$$    /$$                              /$$                             /$$  
| $$   | $$                             |__/                           /$$$$  
| $$   | $$ /$$$$$$   /$$$$$$   /$$$$$$$ /$$  /$$$$$$  /$$$$$$$       |_  $$  
|  $$ / $$//$$__  $$ /$$__  $$ /$$_____/| $$ /$$__  $$| $$__  $$        | $$  
 \  $$ $$/| $$$$$$$$| $$  \__/|  $$$$$$ | $$| $$  \ $$| $$  \ $$        | $$  
  \  $$$/ | $$_____/| $$       \____  $$| $$| $$  | $$| $$  | $$        | $$  
   \  $/  |  $$$$$$$| $$       /$$$$$$$/| $$|  $$$$$$/| $$  | $$       /$$$$$$
    \_/    \_______/|__/      |_______/ |__/ \______/ |__/  |__/      |______/
                                                                              
                                                                              
                                                                              


fn main() {
    let args: Vec<String> = env::args().collect();
    // Collect all the command-line arguments, and convert
    //them into a vector of Strings.

    let search_term = &args[1];
    let file_path = &args[2];
    // The first argument is (obviously) the program name
    // We need the second and third arguments in variables.

    println!("Searching for '{}'", search_term);
    println!("In file \"{}\"", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");

    println!("With text:\n{}", contents);
}
*/

/*
/$$    /$$                              /$$                            /$$$$$$ 
| $$   | $$                             |__/                           /$$__  $$
| $$   | $$ /$$$$$$   /$$$$$$   /$$$$$$$ /$$  /$$$$$$  /$$$$$$$       |__/  \ $$
|  $$ / $$//$$__  $$ /$$__  $$ /$$_____/| $$ /$$__  $$| $$__  $$        /$$$$$$/
 \  $$ $$/| $$$$$$$$| $$  \__/|  $$$$$$ | $$| $$  \ $$| $$  \ $$       /$$____/ 
  \  $$$/ | $$_____/| $$       \____  $$| $$| $$  | $$| $$  | $$      | $$      
   \  $/  |  $$$$$$$| $$       /$$$$$$$/| $$|  $$$$$$/| $$  | $$      | $$$$$$$$
    \_/    \_______/|__/      |_______/ |__/ \______/ |__/  |__/      |________/
                                                                                
                                                                                
                                                                                


fn main() {
    let args: Vec<String> = env::args().collect();

    let (search_term, file_path) = parse_config(&args);

    println!("Searching for '{}'", search_term);
    println!("In file \"{}\"", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file.");

    println!("With text: \n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let search_term = &args[1];
    let file_path = &args[2];

    return (search_term, file_path);
}
*/

/*
/$$    /$$                              /$$                            /$$$$$$ 
| $$   | $$                             |__/                           /$$__  $$
| $$   | $$ /$$$$$$   /$$$$$$   /$$$$$$$ /$$  /$$$$$$  /$$$$$$$       |__/  \ $$
|  $$ / $$//$$__  $$ /$$__  $$ /$$_____/| $$ /$$__  $$| $$__  $$         /$$$$$/
 \  $$ $$/| $$$$$$$$| $$  \__/|  $$$$$$ | $$| $$  \ $$| $$  \ $$        |___  $$
  \  $$$/ | $$_____/| $$       \____  $$| $$| $$  | $$| $$  | $$       /$$  \ $$
   \  $/  |  $$$$$$$| $$       /$$$$$$$/| $$|  $$$$$$/| $$  | $$      |  $$$$$$/
    \_/    \_______/|__/      |_______/ |__/ \______/ |__/  |__/       \______/ 
                                                                                
                                                                                
                                                                                


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    
    println!("Searching for '{}'", config.search_term);
    println!("In file \"{}\"", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");
    
    println!("With text: \n{}", contents);
}

struct Config {
    search_term: String,
    file_path: String
}

fn parse_config(args: &[String]) -> Config {
    let search_term = args[1].clone();
    let file_path = args[2].clone();

    return Config {
        search_term, file_path
    };
}
*/

/*
/$$    /$$                              /$$                           /$$   /$$
| $$   | $$                             |__/                          | $$  | $$
| $$   | $$ /$$$$$$   /$$$$$$   /$$$$$$$ /$$  /$$$$$$  /$$$$$$$       | $$  | $$
|  $$ / $$//$$__  $$ /$$__  $$ /$$_____/| $$ /$$__  $$| $$__  $$      | $$$$$$$$
 \  $$ $$/| $$$$$$$$| $$  \__/|  $$$$$$ | $$| $$  \ $$| $$  \ $$      |_____  $$
  \  $$$/ | $$_____/| $$       \____  $$| $$| $$  | $$| $$  | $$            | $$
   \  $/  |  $$$$$$$| $$       /$$$$$$$/| $$|  $$$$$$/| $$  | $$            | $$
    \_/    \_______/|__/      |_______/ |__/ \______/ |__/  |__/            |__/
                                                                                
                                                                                
                                                                                


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}'", config.search_term);
    println!("In file \"{}\"", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    println!("With text: \n{}", contents);
}

struct Config {
    search_term: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments provided.");
        }
        let search_term = args[1].clone();
        let file_path = args[2].clone();

        return Config {
            search_term, file_path
        };
    }
}
*/

/*
/$$    /$$                              /$$                           /$$$$$$$ 
| $$   | $$                             |__/                          | $$____/ 
| $$   | $$ /$$$$$$   /$$$$$$   /$$$$$$$ /$$  /$$$$$$  /$$$$$$$       | $$      
|  $$ / $$//$$__  $$ /$$__  $$ /$$_____/| $$ /$$__  $$| $$__  $$      | $$$$$$$ 
 \  $$ $$/| $$$$$$$$| $$  \__/|  $$$$$$ | $$| $$  \ $$| $$  \ $$      |_____  $$
  \  $$$/ | $$_____/| $$       \____  $$| $$| $$  | $$| $$  | $$       /$$  \ $$
   \  $/  |  $$$$$$$| $$       /$$$$$$$/| $$|  $$$$$$/| $$  | $$      |  $$$$$$/
    \_/    \_______/|__/      |_______/ |__/ \______/ |__/  |__/       \______/ 
                                                                                
                                                                                
                                                                                


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", config.search_term);
    println!("In file: \"{}\"", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file.");

    println!("With text: \n{}", contents);
}

struct Config {
    search_term: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let search_term = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { search_term, file_path });
    }
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
