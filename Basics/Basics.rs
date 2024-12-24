// ================================================
// 🦀 Rust Basics: Learn the Foundations of Rust 🦀
// ------------------------------------------------
// This file covers:
// - 📦 Variables (immutable & mutable)
// - 🔢 Integers & Data Types
// - 🔄 Conditional Statements (if-else)
// - 🔁 Loops (loop, while, for)
// - 📜 Functions (declaration & usage)
// ------------------------------------------------
// Follow along to master the essentials of Rust!
// ================================================

fn main() {
    // ===== Variables =====
    // Variables in Rust are immutable by default.
    let x = 10; // Immutable variable
    println!("The value of x is: {}", x);

    // To make a variable mutable, use `mut`.
    let mut y = 5; // Mutable variable
    println!("The initial value of y is: {}", y);
    y = 15; // Now we can change the value of y
    println!("The updated value of y is: {}", y);

    // ===== Data Types =====
    // Rust is a statically-typed language, so every variable must have a type.
    // Types are inferred if not explicitly declared.
    let a: i32 = 42; // 32-bit signed integer
    let b: f64 = 3.14; // 64-bit floating-point number
    println!("Integer a: {}, Float b: {}", a, b);

    // ===== Conditionals =====
    // Rust supports if-else for conditional branching.
    let number = 7;
    if number > 0 {
        println!("The number is positive.");
    } else if number < 0 {
        println!("The number is negative.");
    } else {
        println!("The number is zero.");
    }

    // In Rust, if statements can also be used as expressions.
    let is_even = if number % 2 == 0 { true } else { false };
    println!("Is the number even? {}", is_even);

    // ===== Loops =====
    // Rust has three kinds of loops: loop, while, and for.

    // 1. Infinite loop with `loop`.
    let mut count = 0;
    loop {
        count += 1;
        println!("Loop count: {}", count);
        if count == 3 {
            break; // Exit the loop when count reaches 3
        }
    }

    // 2. Conditional loop with `while`.
    let mut num = 5;
    while num > 0 {
        println!("Counting down: {}", num);
        num -= 1;
    }

    // 3. Iterative loop with `for`.
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("Array element: {}", element);
    }

    // `for` loop with a range.
    for i in 1..5 {
        // Range excludes the upper bound (5).
        println!("Range value: {}", i);
    }

    // ===== Functions =====
    // Functions are declared with `fn`.
    // Arguments must have explicit types.
    println!("Sum of 5 and 3 is: {}", add(5, 3));

    let message = greet("Rustacean");
    println!("{}", message);

    // ===== Ownership and Borrowing =====
    // (Bonus: A quick intro to ownership basics.)
    let s1 = String::from("Hello"); // s1 owns the memory
    let s2 = s1; // Ownership is moved to s2, s1 is invalidated
    // println!("{}", s1); // Uncommenting this line will cause a compile-time error

    let s3 = String::from("World");
    let s4 = &s3; // Borrowing s3
    println!("Borrowed value: {}", s4);
    println!("Original value: {}", s3); // s3 is still valid

    // Mutable borrowing.
    let mut z = 10;
    let z_ref = &mut z; // Borrow z as mutable
    *z_ref += 1; // Dereference and modify
    println!("Updated value of z: {}", z);
}

// A simple function to add two integers.
fn add(x: i32, y: i32) -> i32 {
    x + y // The last expression is returned without a semicolon.
}

// A function that takes a string slice and returns a string.
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust.", name)
}