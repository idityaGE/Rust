fn main() {
    let r = get_reference();
    println!("{}", r);
}

fn get_reference() -> &String {
    let s = String::from("Hello, world!");
    return &s;
}