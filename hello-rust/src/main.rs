// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Hello fellow Rustaceans!");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(&message, width, &mut writer).unwrap();
// }

struct Person {
    name: String,
    age: u8,
}

fn new(name: String, age: u8) -> Person {
    Person { name, age }
}

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Point(f64, f64, f64);

enum Animal {
    Dog,
    Cat,
    Bird,
}

impl Animal {
    fn speak(&self) {
        match self {
            Animal::Dog => println!("Woof!"),
            Animal::Cat => println!("Meow!"),
            Animal::Bird => println!("Chirp!"),
        }
    }
}

enum Shape {
    Circle,
    Triangle,
    Square,
}

// enum Option<T> {
//     None,
//     Some(T),
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

    // // CONTROL FLOW
    // check_number(10);
    // check_number(11);

    // count_down(10);

    // day_of_week(3);

    // // OWNERSHIP
    // let s = String::from("Hello");
    // take_ownership(s);
    // // println!("{}", s);

    // let a = String::from("Chukwuemeka");
    // println!("The length of the string is {}", calculate_length(&a));

    // let mut b = String::from("Hello");
    // println!("Before appending: {}", b);
    // append_world(&mut b);
    // println!("After appending: {}", b);

    // // SLICING
    // let name = String::from("Chukwuemeka Onwuzurumba");
    // let first_name = first_word(&name);
    // println!("First Name: {}", first_name);

    // let arr = [1, 2, 3, 4, 5];
    // let middle = middle_three(&arr);

    // for num in middle {
    //     println!("{}", num);
    // }

    // let words = String::from("");
    // let first = first_word(&words);
    // println!("{}", first);

    // // STRUCTS
    // let person = new(String::from("Chukwuemeka"), 25);
    // println!("Name: {}, Age: {}", person.name, person.age);

    // let circle = Circle { radius: 5.0 };
    // println!("The area of the circle is {}", circle.area());

    // let point = Point(2.45, 3.58, 10.34);
    // println!("The x-coordinate is {}", point.0);
    // println!("The y-coordinate is {}", point.1);
    // println!("The z-coordinate is {}", point.2);

    // ENUMS
    let mandy = Animal::Dog;
    mandy.speak();
    let kaila = Animal::Cat;
    kaila.speak();

    println!("A circle has {} sides", shape_sides(Shape::Circle));
    println!("A triangle has {} sides", shape_sides(Shape::Triangle));
    println!("A square has {} sides", shape_sides(Shape::Square));

    let result = divide(10.0, 0.0);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Invalid operation"),
    }
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

fn count_down(mut limit: i32) {
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
        _ => println!("Invalid day"),
    }
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn calculate_length(s: &String) -> u64 {
    s.len() as u64
}

fn append_world(s: &mut String) {
    s.push_str(" World");
}

fn first_word(words: &String) -> &str {
    if words.len() == 0 {
        return "No words found!";
    }

    let bytes = words.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &words[..i];
        }
    }

    &words[..]
}

fn middle_three(arr: &[i32; 5]) -> &[i32] {
    &arr[1..4]
}

fn shape_sides(shape: Shape) -> i32 {
    {
        match shape {
            Shape::Circle => 0,
            Shape::Triangle => 3,
            Shape::Square => 4,
        }
    }
}

fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
