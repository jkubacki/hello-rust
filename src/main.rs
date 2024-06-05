fn main() {
    // variable
    let x: i32;
    x = 512;
    println!("Hello, world! x = {}", x);

    // touple
    let touple: (i32, i32, i32) = (1, 2, 3);
    let (a, b, _c) = touple;

    println!("a = {}, b = {}", a, b);
}
