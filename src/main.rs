fn main() {
    let pi: f32 = 3.14;
    let g = 9.8; // by default f64
    println!("{}", pi);
    println!("{}", g);

    let me: bool = true;
    let you = false; // by default bool
    let us = me && you;
    println!("{}", me);
    println!("{}", you);
    println!("{}", us);

    let char: char = 'A';
    let emo = '😀'; // 4 bytes
    println!("{}", char);
    println!("{}", emo);
}
