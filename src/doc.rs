// main.rs

// This is the entry point of a Rust program
fn main() {
    // Printing Hello World to the console
    println!("Hello, world!");

    // Data Types Examples
    data_types_example();

    // Ownership and Borrowing Examples
    ownership_example();
    borrowing_example();
    mutable_borrowing_example();
    referencing_and_dereferencing_example();

    // Function Examples
    function_example();

    // Struct Example
    struct_example();

    // Enum Example
    enum_example();

    // Option and Result Examples
    option_example();
    result_example();

    // Loop Examples
    loop_example();

    // Conditional Statements Examples
    conditional_statements_example();
}

// Function to demonstrate various data types in Rust
fn data_types_example() {
    let x: i32 = 10;       // Integer
    let y: f64 = 10.5;     // Float
    let z: bool = true;    // Boolean
    let c: char = 'A';     // Character
    let s: &str = "Hello, world!"; // String slice
    let t: (i32, f64, bool) = (10, 10.5, true); // Tuple
    let a: [i32; 3] = [10, 20, 30]; // Array

    println!("Integer: {}", x);
    println!("Float: {}", y);
    println!("Boolean: {}", z);
    println!("Character: {}", c);
    println!("String slice: {}", s);
    println!("Tuple: {:?}", t);
    println!("Array: {:?}", a);
}

// Function to demonstrate ownership in Rust
fn ownership_example() {
    let s1 = String::from("Hello, world!");  // s1 is the owner of the string
    let s2 = s1;  // s2 is the owner of the string, s1 is no longer valid

    // println!("{}", s1);  // Error: s1 is no longer valid
    println!("{}", s2); // s2 is valid
}

// Function to demonstrate borrowing in Rust
fn borrowing_example() {
    let s1 = String::from("Hello, world!");
    let len = get_length(&s1); // Passing reference
    println!("Length of the string '{}' is {}", s1, len);
}

// Function to get the length of a string slice
fn get_length(s: &String) -> usize {
    s.len()
}

// Function to demonstrate mutable borrowing in Rust
fn mutable_borrowing_example() {
    let mut s1 = String::from("Hello, world!");
    modify_string(&mut s1); // Passing mutable reference
    println!("Modified string: {}", s1);
}

// Function to modify a string
fn modify_string(s: &mut String) {
    s.push_str(" What's Up?");
}

// Function to demonstrate referencing and dereferencing in Rust
fn referencing_and_dereferencing_example() {
    let x = 10;
    let r = &x;  // Creating a reference
    let y = *r;  // Dereferencing the reference
    println!("x: {}, y: {}", x, y);
}

// Function to demonstrate a simple addition function
fn function_example() {
    let n1 = 52;
    let n2 = 29;
    let result = add_num(n1, n2);
    println!("Sum of {} and {} = {}", n1, n2, result);
}

// Function to add two numbers
fn add_num(num1: u16, num2: u16) -> u16 {
    num1 + num2
}

// Struct to represent an Employee
struct Employee {
    name: String,
    age: u8,
    hired: bool,
}

// Function to demonstrate struct usage in Rust
fn struct_example() {
    let emp = Employee {
        name: String::from("Adii"),
        age: 20,
        hired: true,
    };
    println!("Employee Name: {}\nEmployee Age: {}\nEmployee Hired: {}", emp.name, emp.age, emp.hired);
}

// Enum to represent colors
enum Color {
    Red,
    Green,
    Blue,
}

// Function to demonstrate enum usage in Rust
fn enum_example() {
    let color = Color::Red;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

// Function to demonstrate Option usage in Rust
fn option_example() {
    let num: Option<i32> = Some(10);
    match num {
        Some(n) => println!("Number: {}", n),
        None => println!("No value"),
    }
}

// Function to demonstrate Result usage in Rust
fn result_example() {
    let result: Result<i32, &str> = Ok(10);
    match result {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// Function to demonstrate various loops in Rust
fn loop_example() {
    let mut i = 0;
    // Infinite loop
    loop {
        if i == 5 {
            break;
        }
        println!("{}", i);
        i += 1;
    }
    i = 0;
    // Conditional loop
    while i < 5 {
        println!("{}", i);
        i += 1;
    }
    // Iterative loop
    for i in 0..5 {
        println!("{}", i);
    }
}

// Function to demonstrate conditional statements in Rust
fn conditional_statements_example() {
    let num = 10;
    if num > 5 {
        println!("Greater than 5");
    } else if num < 5 {
        println!("Less than 5");
    } else {
        println!("Equal to 5");
    }
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
