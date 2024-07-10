fn main() {
    let mut str_arr: [&str; 5] = ["Hello", "World", "from", "Rust", "Programming"];
    print_array(& mut str_arr); 
    println!("\nmain fn arr = {:?}", str_arr);
}

fn print_array(arr: & mut [&str; 5]) {
    arr[2] = "!,";
    for &i in arr.iter() {
        print!("{} ", i);
    }
}
