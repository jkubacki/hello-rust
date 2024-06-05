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
    fn add(a: i32, b: i32) -> i32 {
        // return a + b;
        a + b // ommit semi-colon for tail to return
    }
    let result = add(1, 2);
    println!("result = {}", result);

    // block like in ruby
    let block = {
        let a = 1;
        let b = 2;
        a + b // tail
    };

    println!("block = {}", block);

    // if
    fn return_1_or_2(return_2: bool) -> i32 {
        if return_2 {
            2
        } else {
            1
        }
    }
    return_1_or_2(true);

    // match
    fn return_1_or_2_revamped(return_2: bool) -> i32 {
        match return_2 {
            true => 2,
            false => 1,
        }
    }
    return_1_or_2_revamped(false);

    // access values
    let touple = (1, 2, 3);
    let _ = touple.0;

    // struct
    struct SomeStruct {
        a: i32,
        b: i32,
        c: i32,
    }
    let some_struct = SomeStruct { a: 1, b: 2, c: 3 };
    let _ = some_struct.a;
    let _ = some_struct.b;
    let _ = some_struct.c;

    // use library
    use std::cmp::min;
    let _ = min(1, 2);

    let _ = "test".len();
    let _ = str::len("test");
}
