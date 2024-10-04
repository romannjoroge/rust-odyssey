Rust emphasizes a great developer experience so much! Is this really the same programming language I used. The book kinda frames Rust as this language where the compiler helps you to avoid subtle bugs like concurrency bugs and allows you to focus more on logic. Maybe my issues is that I've never really made anything where I would be thinking of such issues.

Let me give learning Rust a shot then I can act as a judge to see if Rust really allows one to build systems faster

# Chapter 1: Getting Started

rustup is used as a rust version manager - allows one to install different versions of rust and to pick which one to use

## Installing rustup

To install ran the following script
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

This installs rustup and the latest version of rust. To check what version of rust you're using run the following command
```bash
rustc --version
```

To install the latest version of rust do the following
```bash
rustup update
```

## First Program

You can use rustc to run rust files. Pretty cool stumpling into that. Looks like I didn't even need a project setup it just ran the code. Even the guide says that Rust code doesn't have to be in a project. Looks like standard in rust is to seperate words in file names with a _ not -.

It looks like rust also has a main function in a similar way to C. No wonder nothing ran in the initial trial.

rustc does not run the code but it compiles it. I forgot that Rust is compiled.

I wonder how rust macros are different from just normal functions. 

## Cargo

What i know is that cargo is sort of like npm where you can use it to create projects, manage dependencies and build your projects. An interesting thing about creating projects with cargo is that it provides 2 options:

1. cargo new <name> if you also want to create a new folder for the project
2. cargo init when you already have a project folder and want to put your code in it

In the generate Cargo.toml file there is an edition section under [package]. This section is used to denote the Rust edition being used and not your programs edition or anything like that.

Its a must to have a src directory and for that directory to hold your applications code. This is what cargo expects of a project. Cargo can also build and run your project at once using the ~cargo run~ command. This must be why I forgot that Rust is a compiled language.

~cargo check~ is useful for times when you want to check if your project compiles but you don't need to run it. cargo check just *checks* if project compiles. It dose not build so its alot faster

The result of cargo build is not the most optimized build. For something that produces a very optimized build run ~cargo build --release~

# Chapter 2: Making a guessing game

So declaring variables in Rust is not as guessable as I thought it was. What's the syntax for declaring a mutable variable?

Just for formality I should probably state that printing is done using the ~println~ macro. It expects a string arguement and can take format string.
To accept input you use the io crate in the std crate. Pretty similar to C. To import a create use the ~use~ statement. To traverse crates use the ~::~ operator. For example

```rust
use std::io;
```

For one my guess in how variables are declared was pretty close just that it looks like you don't indicate type? Or at least you do it in another way? An example of defining a mutable string is (Good to remember that by default variables are immutable in Rust!):

```rust
let mut stringVariable = String::new();
```

(To define the type of a variable we use the **:** operator. For example):

```rust
let guess: u32 = guess.trim().parse().expect("Something Wrong Happened");
```

Looks like choosing methods of classes also uses the ~::~ operator. These methods are known as associated functions of a type. Correction there are still methods in Rust so associated functions must only be for types and the concept of classes could still be there.

Rust seems to have an interesting way of handling errors where an operation that could lead to an error needs to be **expeceted**. This kinda sounds similar to Python but is done abit differently:

```rust
some_error_prone_function()
    .expect("Some message");
```

Maybe it doesn't handle errors but throws a specific error message?

Even if something is immutable it doesn't mean you can't call its methods. I guess as long as it does not change it.

References are immutable by default so if you want a mutable reference to something you have to do ~&mut~. It also seems that references are used alot to pass variables. Or maybe its coz its a string in this case?

An operation that could result in one or more states i.e failure or success returns an enum. One common enum type is Result which can either be Ok(indicates success) or Err(indicates something wrong). It looks like ~.expect()~ deals with a result if it becomes an Err. To be more precise **expect is a method of the Result enum**. When the Result enum resolves as an Err it crashes with the passed message otherwise it returns the value of the Ok(). 

Looks like in Rust the best practice is to not place brackets for condition of while loop, could be the same as the other control structures.

Looks like division between integers in Rust always produces an integer!

~cargo build~ also installs dependencies in the Cargo.toml file! Cargo.lock file is responsiple for keeping track of the speciific version of the dependencies installed. This must have been what gave me trouble last time! Be careful when you use the ~cargo update~ command to update dependencies!

It's pretty cool that ~cargo doc~ generate a local site that has documentation of the crates installed as dependencies. So I never needed to go searching online for docs!

**match** a enum is a good way to define various ways to deal with the possible different values of the enum

```rust
use std::cmp::Ordering;

match guess.cmp(...) {
    Ordering::Less => ...
    Ordering::More => ...
    Ordering::Equal => ..
}
```

Using match to handle an enum result feels alot better that simply using if else if else. At least there is alot of certainty that you have handled all possible scenarios.

For an infinite loop instead of doing while true you use **loop**. For me loop feels more readable than while true so makes sense. It communicates intent more!

```rust
loop {
    // Do stuff
    break;
}
```

You can match a Result type. A shorthand for doing this is:

```rust
io::stdin()
    .read_line() {
        Ok(text) => text,
        Err(_) => ...
    }
```

In the Err you define the type of error you are handling. Doing Err(_) means you are handling all errors.

# Chapter 3: Common Programming Concepts

So far I can say that Rust seems to have its own unique way of handling things that are common in other languages e.g loop, matching e.t.c. Let's see more:

## Variables and Mutability

By default variables in Rust are immutable. I think this helps alot in concurrency to avoid things like race conditions, mutex errors etc if something is immutable. We use the **mut** keyword to define something as mutable.

**Constants** in Rust typically start with capital letters (should all be capital letters), when you define them you have to indicate the type of the variable. What's the difference between a constant and an immutable variable? Rust constants can be defined in the global scope.

We have *variable shadowing* where you can define a new variable with the same name as a previously defined variable. Maybe its good for cases where mutliple different things can be explained in the same way? Or their type changes somehow in the future?

```rust
let guess = std::io::stdin()
                        .read_line()
                        .expect("Failed to get user input");
let guess: u32 = guess.trim().parse().expect("Unable to parse number");
```

## Data Types In Rust

Rust numbers go from size of 8 to 128 i.e. u8 all the way to u128. There is also usize (maybe variable size int?).

Booleans in rust are annoteted using the **bool** keyword.

Tuples in Rust are a collection of values that can be of different types. Once declared their size cannot change. They are mutable.

```rust
let tup: (u32, String, bool) = (21, "Roman Njoroge", false)
```

Unlike in other languages you need to destructure a tuple in order to get values from it. *It is not necessary to annotate types of values in a tuple*.

```rust
let patient_data = ("Patient 1", 20, "Flu");
let (name, age, disease) = patient_data;

// To indicate if destructured value is mutable add mut
let (mut name, age, mut disease) = patient_data;
```

You can also index with position of item but unlike in other languages you use a dot to index i.e:

```rust
// Could be useful when you only want specific items from tuple
let patient_data = ("Patient 1", 20, "Flu");
println!("{} has come to the hospital with a {}", patient_data.0, patient_data.2);
```

## Functions

Functions are defined with the following format:

```rust
fn function_name(args..) -> return_type {
    // Body
}

// args takes the form identifier: type
```

Using the arrow to indicate return type is very reminiscent of typing in Python so not bad. Passing args as identifier: type is similar to typing again and I think typescript.

The *&str* type is the type of a string literal. Looks like *str* and *String* are very interroperable.

Rust functions don't have to be defined before they are called! Very nice.

I was able to guess the difference between a statement and expression! But for clarity sake an expression is something that returns a value while a statement doesn't return a value. Function bodies in Rust are made up of statements that can optionally have an ending statement.

Expressions do not end with a semicolon! If you add a semicolon it becomes a statement! This makes this possible

```rust
fn return_string(args) -> &str {
    // If I end it with a semicolon it throws an error because it becomes a statement and you can't return a statement
    "Some String"
}
```
