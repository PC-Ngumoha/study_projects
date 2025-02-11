// use ferris_says::say;
// use std::io::{stdout, BufWriter};


// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(&message, width, &mut writer).unwrap();
// }

fn main() {

    // IMMUTABLE VARIABLES
    // let a = 10;
    // let mut b = 20;

    // println!("a is {} and b is {}", a, b);

    // // a = 100;
    // b = 200;

    // println!("a is {} and b is {}", a, b);

    // // CONSTANTS
    // const PI: f32 = 3.14159;

    // println!("The value of PI is {}", PI);

    // // SHADOWING
    // let num = 5;
    // let num = 45.09;
    // let num = "Hello";

    // println!("The value of num is {}", num);

    // // INTEGER OPS
    // let a = 5;
    // let b = 7;

    // println!("a + b = {}", a + b);
    // println!("a - b = {}", a - b);
    // println!("a * b = {}", a * b);
    // println!("a / b = {}", a / b);

    // // TUPLE MANIPULATION
    // let tup: (i8, f64, char, bool) = (23, 1.72, 'C', true);
    // let (age, height, first_character_in_name, likes_rust) = tup;
    // println!("Age: {},\nHeight: {},\nFirst Character in Name: {},\nLikes Rust: {}", age, height, first_character_in_name, likes_rust);

    // // ARRAY INDEXING
    // let arr = [1, 2, 3, 4, 5];
    // println!("The first element in the array is {}", arr[0]);
    // println!("The last element in the array is {}", arr[arr.len() - 1]);
    // // println!("Accessing out of bounds element in the array: {}", arr[10]);


    // // FUNCTIONS
    // say_hello();
    // println!("The product of 5 and 10 is {}", multiply(5, 10));
    // println!("Is 10 even? {}", is_even(10));
    // println!("Is 11 even? {}", is_even(11));

    // CONTROL FLOW
    check_number(10);
    check_number(11);

    count_down(10);

    day_of_week(3);
}

// FUNCTION: say_hello - prints a greeting message
fn say_hello() {
    println!("Hello from Rusty pikes!");
}

// FUNCTION: multiply - multiplies two numbers
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// FUNCTION: is_even - checks if a number is even
fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn check_number(num: i32) {
    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

fn count_down (mut limit: i32) {
    while limit > 0 {
        println!("{}", limit);
        limit -= 1;
    }
}

fn day_of_week(day: i32) {
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day")
    }
}