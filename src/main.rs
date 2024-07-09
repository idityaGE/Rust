fn main() {
    let mut arr: [i32;5] = [1,2,3,4,5];
    let _first = arr[0];
    arr[2] = 9;
    let len = arr.len();
    let mut i = 0;
    while i < len {
        println!("{}", arr[i]);
        i += 1;
    }
    for i in 0..len {
        println!("{}", arr[i]);
    }
    for i in arr.iter() {
        println!("{}", i);
    }
    for i in &arr {
        println!("{}", i);
    }
    for i in arr.iter_mut() {
        *i += 1;
    }
    for i in &arr {
        println!("{}", i);
    }
    let arr2 = [1,2,3,4,5];
    let arr3 = [3;5];
    let arr4 = [1,2,3,4,5];
    println!("{:?}", arr2);
    println!("{:?}", arr3);
    println!("{:?}", arr4);
    let arr5 = &arr[1..4];
    println!("{:?}", arr5);
    let arr6 = &arr[1..];
    println!("{:?}", arr6);
    let arr7 = &arr[..4];
    println!("{:?}", arr7);
    let arr8 = &arr[..];
    println!("{:?}", arr8);
    let arr9 = &arr;
    println!("{:?}", arr9);
    let arr10 = &arr as *const i32;
    println!("{:?}", arr10);
    let arr11 = &arr as *const [i32;5];
    println!("{:?}", arr11);


}
