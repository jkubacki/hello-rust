use std::fmt::Error;

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

    // catch all match
    let _: &str = match 3 {
        1 => "one",
        2 => "two",
        _ => "unknown",
    };

    // methods
    impl SomeStruct {
        fn add(&self) -> i32 {
            self.a + self.b + self.c
        }
    }
    let result = some_struct.add();
    println!("some_struct.add() = {}", result);

    // immutability ❤️
    // let number = 1;
    // number = 2; // error

    // mutability
    // let mut number = 1;
    // number = 2;

    // generic functions
    fn generic<T>(value: T) -> T {
        value
    }

    let _ = generic(1);
    let _ = generic("test");

    fn generic_same_type<T>(v1: T, v2: T) -> (T, T) {
        (v1, v2)
    }

    let _ = generic_same_type(1, 2);
    // let _ = generic_same_type(1, "test"); // mismatched types

    fn generic_different_type<T, U>(v1: T, v2: U) -> (T, U) {
        (v1, v2)
    }
    let _ = generic_different_type(1, "test");

    // define simple macro
    macro_rules! add {
        ($a:expr, $b:expr) => {
            $a + $b
        };
    }
    // use macro
    let _ = add!(1, 2);

    // unwrap, panic, option
    let something: Option<i32> = Some(1);
    something.unwrap();

    // let none: Option<i32> = None;
    // none.unwrap(); // panic

    // Result, Ok, Err
    let result: Result<i32, &str> = Ok(1);
    result.unwrap();

    // let result: Result<i32, &str> = Err("error");
    // result.unwrap(); // panic

    // function with Result
    fn fetch_from_api() -> Result<i32, Error> {
        // Err(Error)
        Ok(200)
    }
    let res = fetch_from_api();
    res.unwrap(); // panic on Err

    let _ = fetch_from_api().expect("200"); // panic on Err

    // Destructure
    if let Ok(http_response) = fetch_from_api() {
        println!("http_response = {}", http_response);
    } else {
        println!("error");
    }

    fn calling_function() -> Result<i32, Error> {
        let res = fetch_from_api()?; // bubble up error
        Ok(res)
    }

    let _ = calling_function().expect("200");
}
