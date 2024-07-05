fn main() {
    let st = String::from("Hello, world!");
    let len = get_length(&st);  // Passing reference
    println!("Length of the string '{}' is {}", st, len);
}

fn get_length(s: &String) -> usize {
    // return (*s).len(); // Dereferencing the reference
    return s.len(); // Auto dereferencing
}