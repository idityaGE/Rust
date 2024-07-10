fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let mut _vec2 = Vec::<i32>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);

    let mut vec3 = vec![1, 2, 3];
    println!("{:?}", vec3);
    vec3.push(4);
    vec3.push(5);
    vec3.pop();


}
