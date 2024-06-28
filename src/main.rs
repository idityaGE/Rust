fn main() {
    let mut s = String::from("Hello, world!");
    let r1 = &s;  // Immutable reference
    let r2 = &s;  // Immutable reference
    // let r3 = &mut s;  // Error
    println!("r1: {}, r2: {}", r1, r2);
}