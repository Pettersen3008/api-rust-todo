# Rust API

## Overview

Today we have had a skills development day at work and i wanted to use some of the day to learn the basic of Rust.

Learning Rust can be beneficial for several reasons. Here are a few:

Safety: Rust provides strict compile-time checks that help catch bugs and prevent crashes at runtime. It enforces memory safety and eliminates common issues like null pointer dereferences and buffer overflows, making it an excellent choice for systems programming and critical applications.

Performance: Rust offers high-performance execution by allowing fine-grained control over system resources. It provides predictable performance and efficient memory management without the need for a garbage collector, making it suitable for building fast and efficient software.

Concurrency: Rust has built-in support for writing concurrent and parallel code. It ensures thread safety through its ownership and borrowing system, which prevents data races and allows for safe and efficient concurrent programming.

Compatibility: Rust's interoperability with other programming languages enables you to easily integrate Rust code into existing projects or collaborate with developers using different languages. It can be used for system-level programming, web development, game development, and more.

Growing Popularity: Rust is gaining popularity among developers due to its unique features and benefits. Learning Rust can open up opportunities to work on exciting projects and collaborate with a thriving community of developers.

In summary, learning Rust equips you with a powerful programming language that emphasizes safety, performance, and concurrency. Whether you want to build robust systems, optimize performance-critical applications, or explore modern software development, Rust is a valuable language to add to your skill set.

## Installation

The easiest way to get Cargo is to install the current stable release of [Rust](https://www.rust-lang.org/) by using [rustup](https://rustup.rs/). Installing Rust using `rustup` will also install `cargo`.

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Create a new project

```bash
cargo new api-rust-todo
```

## Dependencies

```bash
cargo add actix-web
cargo add serde --features derive
cargo add chrono --features serde
cargo add uuid --features v4
```

## Run the project

```bash
cargo run
```

## References

https://blog.ediri.io/rust-development-creating-a-rest-api-with-actix-web-for-beginners

## What next

My next step will be to add a database like Postgres and run it in a docker container.
I will also try to do something similar in golang to compare what i like the most.
