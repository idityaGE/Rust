fn main() {
    let s1 = String::from("Hello, world!");
    let mut s2 = s1.clone();
    {
        println!("s1: {}, s2: {}", s1, s2);
    }
    s2 = "Adii".to_string();
    println!("s1: {}, s2: {}", s1, s2);
}
