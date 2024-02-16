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


### `thread_rng()`

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

### Package Organization

* **Packages**: A `cargo` feature to build, test and share **crates**.

* **Crates**: A tree of **modules** that produces a library or executable.

* **Modules**: Let you control the organization, scope, and privacy of **paths**.

* **Paths**: A way of naming an item, such as a `struct`, `function` or `module`.

**Binary crates**: Can be compiled to an executable, and each binary crate has a `main()` function.

**Library crates**: Don't have a `main()` function, and cannot be compiled to an executable. A project may only have **one** library crate.

### Points to Remember

* Making a module public does not make its contents public --- those have to be made public explicitly.

* Making a `struct` public does not make its contents public --- those have to be made public explicitly.

* Making an `enum` public makes all of its variants public.

* To remove the `\n` that appears after a string that was input by the user, use `let title = title.trim_end();`.

### Getting the value of a key in a `HashMap`

Consider the following code snippet

```rust
let team_name = String::from("Yellow");
let score = score.get(&team_name).copied().unwrap_or(0);
```

`get()` returns an `Option<&V>`. `copied()` creates a copy of it, so you have `Option<V>` instead of `Option<&V>`. `unwrap_or()` gets the value inside the `Some(V)` or the default value we pass in (0), in case of a `None`.

### Recovering from errors with `Result<T, E>`

Instead of using the `panic!` macro whenever you encounter an erro, use `Result<T, E>` to be able to recover from it.

If you ever encounter errors, propagate the success/failure back to the calling function, and let the calling function decide what to do with it. Rust provides a shortcut for propagating errors. A `?`  can be usedafter any statement that returns a `Result<T, E>`.

```rust
File::open("hello.txt")?.read_to_string(&mut username)?;
```

Returns the `Err` part of `Result` (which can occur after `open()` and `read_to_string()`) to the calling function, execution continues as usual otherwise.

An even faster way of doing it,

```rust
let x = fs::read_to_string("hello.txt");
```

Opens the file (along with error handling), creates a string, loads file contents to the string (along with error handling), and returns the string.

### Generics

Abstract types used in place of concrete types.

```rust
fn largest<T>(list: &[T]) -> &T {

}
```

`largest` is generic over `T`, takes `list` as a parameter, which is a slice of type `T`, and returns a reference to a value of type `T`.

This also works with structs.

```rust
struct Point<T> {
    x: T,
    y: T
}

let integer = Point{x: 1, y: 2};
let float = Point{x: 3.14, y: 1.414};

struct Point<T, U>{
    x: T,
    y: U
}

let both_ints = Point{x: 1, y: 2};
let both_floats = Point{x: 3.14, y: 1.414};
let mixed = Point{x: 1, y: 3.14};
```

Generics don't slow down execution because Rust performs *monomorphization* when compiling.

### Traits

Traits define functionality that a particular type has and can share with others.

Consider a trait `Summary` with a summarize function

```rust
pub trait Summary {
    fn summary(&self) -> String;
}
```

Now, all types that implement the `Summary` trait **must** have a method `summarize()` defined on them; in other words, each type implementing this trait must provide its onw custom behaviour for the body of `summarize()`.

Assume a user-defined struct called `CustomType`.

```rust
impl Summary for CustomType {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.title, self.content);
    }
}
```

`CustomType` has implemented `Summary`, so users can call the `summarize()` method on instances of `CustomType`.

Just remember to bring `CustomType` **and** its trait (`Summary`) into scope.

#### Coherence

You can implement a trait on your own types, but either the type or the trait should be local to your crate.

* You can implement the `Display` trait (from the standard library) on `CustomType` (local)

OR

* You can implement the `Summary` trait (local) on `Vec<T>` (from the standard library).

But you cannot implement an external trait on an external type, like trying to implement `Display` on `Vec<T>`.

This restriction is called *coherence*.

#### Setting default behaviour for methods

You can have default behaviour for some or all of your methods.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("(Read more...)");
    }
}
```
Now, when you implement `Summary` on a type, you can decide whether to keep the default behaviour or to override it.

```rust
impl Summary for CustomType{} // default behaviour implemented, 0 methods overridden.
```
#### Using traits to specify function parameters

If you create a function like so,

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}
```
it will **only** accept items of a type that implements `Summary`. Any type that implements `Summary` can be passed to `notify()`.

A longer, verbose way of creating this function is as follows

```rust
pub fn notify<T: Summary>(item: &T) {
    pritnln!("Breaking news: {}", item.summarize());
}
```

If you have multiple parameters, you can write 

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {
    ...
}
```

instead of

```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    ...
}
```

Further, the first method makes it clear that `item1` and `item2` should both be of type `T` that implements `Summary`, not any two types that implement `Summary`.

#### Multiple trait bounds

If you want your parameter to implement `Summary` and `Display`,

```rust
pub fn notify(item1: &(impl Summary + Display)) {
    ...
}
```

OR

```rust
pub fn notify<T: Summary + Display>(item1: &T) {
    ...
}
```

#### Using `where` Clauses

Useful when dealing with multiple parameters that each need to have multiple traits implemented on them. So, instead of doing
```rust
pub fn some_fn<T: Display + Clone, U: Clone + Debug + Summary>(t: &T, u: &U) -> i32 {
    ...
}
```
you do
```rust
pub fn some_fn<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug + Summary,
{
    ...
}
```

This way of doing things is more readable, because the function signature looks like a normal function.

#### Returning traits

If your function returns a type that implements a trait, the function signature becomes
```rust
fn returns_summarizable() -> impl Summary {
    ...
}
```

Now this function can return a variable of type `CustomType`, `Tweet` (the example from the book), or any other type that implements `Summary`, without caring about which one it is specifically. But what you return **has** to be of one type.

You cannot return a `CustomType` instance or a `Tweet` instance based on a condition.

#### Conditionally implementing methods based on trait bounds

Certain methods can be made available to certain types that implement specific traits.

```rust
struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        return Self{x, y};
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The larger member is: {}", self.x);
        } else {
            println!("The larger member is: {}", self.y);
        }
    }
}
```

Every instance of Pair (of *any* type) will get access to the `new()` method, but only those instances which are of a type that implements `Display` and `PartialOrd` will get access to the `cmp_display()` method.
