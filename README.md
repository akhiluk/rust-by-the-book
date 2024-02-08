# Rust By The Book

## A Collection of notes and quick projects

Use `> rustup update` to update tools like `rustc` and `rustfmt`.

`func_name!()` calls a macro, whereas `func_name()` calls a function.

### On Cargo

Cargo is not just a **package manager**, it's also a **build system**. As your project grows, calling `rustc` on all your files just won't cut it.

`Cargo.toml`: Pretty self-explanatory

`Cargo.lock`: Stores exact versions of dependencies in your project. Ensures reproducible builds, and makes sure that versions aren't auto-upgraded unless explicitly mentioned.

`> cargo build`: Builds your project and creates a **debug executable**.

`> cargo run`: Builds your project, creates a debug executable and runs it for you.

`> cargo check`: Checks your code to make sure it compiles but doesn't build it.

`> cargo build --release`: Builds your project **with optimizations** and creates a **release executable**, instead of a **debug executable**.

`> cargo update`: Updates dependencies to the latest version. Follows SemVer rules.

### Reading user input

```rust
use std::io;

fn main() {
    let mut user_input = String::new();
    
    io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read user input!");
}
```

`std::io::stdin().read_line(&mut user_input)`
reads user input and *appends* it to `user_input`, but it also returns a `Result<T>` enum.

`Enum`: A type that can be in one of multiple possible states, each state is called a variant.

`.expect()` is actually a mathod defined on `Result<T>`.

### `Result<T>`

* If `Ok(T)` --> `expect()` will take the value that `Ok` holds (number of bytes in the user's input) and return it.
* If `Err` --> program crashes and prints input passed to `expect()`. This only happens if the underlying OS has any issues.

`let secret = rand::thread_rng().gen_range(1..=100);`

`thread_rng()` is an RNG (random number generator) that is local to the current thread of execution and is seeded by the OS.

### Recovering gracefully from invalid user input

`let guess = guess.trim().parse().expect("Please type a number!");`

Converting user input from `String` to `integer` types, but the whole program crashes if someone enters an invalid character. Instead, try recovering gracefully.

```rust
let guess = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue
};
```

Assign `num` to `guess` if the `parse()` operation succeeds, else ignore and continue to next iteration of the loop.

### Notes on shadowing

Making a variable mutable means you can keep changing its value so long as they're the same type as the original variable. But when you shadow a variable, you're essentially creating a new variable with the `let` keyword, so the type can change.

### Tuples

Tuples can be destructured or accessed via the index.

```rust
    let tup = (173, 15.08, 234);
    let (x,y,z) = tup; // destructuring

    let first_int_number = tup.0;
    let float_number = tup.1;
    let second_int_number = tup.2;
```

### Truthiness

Rust does have any "truthiness" unlike other languages. `integer` values are not true or false. Only Boolean expressions can be used in an `if` statement.

To return a value when you break from a loop, add the expression to the `break`.

```rust
    ..
    ..
    ..
    .
    .
    if counter == 10 {
        break counter*2;
    }
```

### Ownership

The problems that ownership solves are:

* keeping track of what parts of code are using what data on the heap.

* minimizing the amount of duplicate data on the heap.

* cleaning up unused data on the heap, so you don't run out of space.

### References

The act of creating a reference is called **borrowing**.

* There can only be **one** mutable reference to a variable in a scope.

* You cannot have a reference to a variable and a mutable reference to the same variable in the same scope.

* However, you can have any number of immutable references to the same variable, since those are all read-only in nature.
