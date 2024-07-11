fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(item) = stack.pop() {
        println!("Popped item: {}", item);
    }
}