fn add(a: i32, b: i32) -> i32 {
    // return a + b;
    a + b // ommit semi-colon for tail to return
}

fn main() {
    // variable
    let x: i32;
    x = 512;
    println!("Hello, world! x = {}", x);

    // touple
    let touple: (i32, i32, i32) = (1, 2, 3);
    let (a, b, _c) = touple;

    println!("a = {}, b = {}", a, b);

    // function
    let result = add(1, 2);
    println!("result = {}", result);

    // block like in ruby
    let block = {
        let a = 1;
        let b = 2;
        a + b // tail
    };

    println!("block = {}", block);
}
