use std::cmp::*;
use std::fmt::Display;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{Error, Read};
use std::iter::Filter;

struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn is_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
    fn invert(x: i32, y: i32) -> Self {
        Self { x: y, y: x }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}

fn main() {
    let add = |x, y| x + y; // closure
    let result = add(3, 4);
    println!("The result is {}", result);

    let _z: i32 = 42;
    let mut x = 42;
    let test = min(1, 2);
    x = 24;

    println!("Hello, world!"); // ! = macro
    println!("{x}");
    println!("{}", x);
    println!("{test}");

    dbg!(test);
    dbg!(min(1, 2));

    match str::len("Hello, world!") {
        0 => println!("Empty"),
        1..=20 => println!("Short"),
        _ => println!("Long"),
    }

    let vec = vec![1, 2, 3];
    let vec2: Vec<i32> = vec![1, 2, 3];
    let vec3: Vec<i32> = Vec::new();
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .fold(0, |x, y| x + y);
    println!("{:?}", vec3);

    let p = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, ..p };
    let pp = Point::invert(1, 2);
    println!("{}", pp);

    let str = "Hello, world!";
    let str2 = String::from("Hello, world!");
    greet(&str2);
    greet(str);

    let contents = match read_to_string("test.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Error: {}", error),
    };
    let contents2 = match read_file("test.txt") {
        Ok(contents) => contents,
        Err(error) => panic!("Error: {}", error),
    };
    println!("{}", contents);
    println!("{}", contents2);

    let y = 1;
    let p2 = Point { x: 1, y };

    let v = vec![1, 2, 3, 4, 5];
    let v2 = &v[2..4];
    println!("v2 = {:?}", v2); // {:?} = debug

    println!("{:?}", (0..).contains(&100)); // true >
    println!("{:?}", (..20).contains(&20)); // false <
    println!("{:?}", (..=20).contains(&20)); // true <=
    println!("{:?}", (3..6).contains(&4)); // true

    // Result
    // .unwrap() = panic if failure
    let s = std::str::from_utf8(&[0, 159, 146, 150]).unwrap();
    let s2 = std::str::from_utf8(&[0, 159, 146, 150]).expect("Invalid UTF-8");
    match std::str::from_utf8(&[0, 159, 146, 150]) {
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("Error: {}", e),
    }
    if let Ok(s) = std::str::from_utf8(&[0, 159, 146, 150]) {
        println!("s = {}", s);
    }

    let x = 42;
    let x_ref = &x;

    println!("Value of x: {}", x);
    println!("Value of x_ref: {}", x_ref);
    println!("Value pointed to by x_ref: {}", *x_ref);
}

fn bubble_up_error() -> Result<(), std::str::Utf8Error> {
    // ? = bubble up error
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    Ok(())

    // ==
    // match read_file("test.txt") {
    //     Ok(contents) => println!("{}", contents),
    //     Err(error) => println!("Error: {}", error),
    // }
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn implicit(x: i32, y: i32) -> i32 {
    x + y
}

fn match_string(s: &str) {
    match s {
        "Hello" => println!("Hello"),
        "World" => println!("World"),
        _ => println!("Other"),
    }
}

fn match_point(p: Point) {
    match p {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x, y } => println!("({x}, {y})"),
    }
}

fn generic_func<L, R>(one: L, two: R) {
    // ...
}
fn generic_with_constraint<T: Display>(value: T) {
    println!("value = {}", value);
}

fn option_divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn read_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
