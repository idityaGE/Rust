fn main() {
    let x = 10;
    let y = &x;
    let z = add_num(y);  // Error
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn add_num(num: &i32) -> i32 {
    return num + 10;
}