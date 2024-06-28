fn main() {
    let s1 = String::from("Hello, world!");
    let (s2, len) = get_length(s1); // ownership gets transferred to the function
    println!("Length of the string '{}' is {}", s2, len); // Error
    // s1 is not available here because ownership is transferred to the function
}

fn get_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s,len );
}