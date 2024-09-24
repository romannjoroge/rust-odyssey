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
