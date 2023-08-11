use std::num::ParseIntError;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use List::{Cons, End};

// This is your first Rust program!
fn first() {
    // Print "Hello, World!" to the console
    println!("Hello, World!");
}

fn second() {
    // Declare a variable
    let greeting = "Hello, Rust!";
    println!("{}", greeting);

    // Declare a mutable variable
    let mut counter = 5;
    println!("Counter: {}", counter);

    // Modify the mutable variable
    counter = 10;
    println!("Counter: {}", counter);

    // Constants
    const MAX_VAL: u32 = 100_000;
    println!("Max Value: {}", MAX_VAL);

    // Shadowing
    let x = 5;
    let x = x * 2;
    let x = x + 10;
    println!("x: {}", x);
}

fn third() {
    let num = 10;

    // If/else statement
    if num < 10 {
        println!("The number is less than 10.");
    } else {
        println!("The number is 10 or greater.");
    }

    // Match statement
    match num {
        0 => println!("The number is zero."),
        1..=9 => println!("The number is between 1 and 9."),
        _ => println!("The number is 10 or greater."),
    }

    // Loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break;
        }
        println!("Loop count: {}", counter);
    }

    // While loop
    counter = 0;
    while counter < 5 {
        counter += 1;
        println!("While loop count: {}", counter);
    }

    // For loop
    for num in 1..6 {
        println!("For loop count: {}", num);
    }
}

// Define a function
fn greet() {
    println!("Hello, Rust!");
}

// Define a function with parameters
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Define a function with a return value
fn add(a: i32, b: i32) -> i32 {
    a + b  // no semicolon, so this is an expression that gets returned
}

// Define a higher order function
fn apply<F>(f: F, x: i32, y: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(x, y)
}

fn fourth() {
    // Call a function
    greet();

    // Call a function with parameters
    greet_person("Alice");

    // Call a function with a return value
    let sum = add(5, 7);
    println!("5 + 7 = {}", sum);

    // Call a higher order function
    let product = apply(|a, b| a * b, 5, 6);  // pass in a closure that multiplies
    println!("5 * 6 = {}", product);
}

// Define a struct
struct Point {
    x: f64,
    y: f64,
}

// Define a function that takes a Point
fn print_point(point: Point) {
    println!("Point at ({}, {})", point.x, point.y);
}

// Define an enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Define a function that takes a Direction
fn print_direction(direction: Direction) {
    match direction {
        Direction::North => println!("We're heading North!"),
        Direction::South => println!("We're heading South!"),
        Direction::East => println!("We're heading East!"),
        Direction::West => println!("We're heading West!"),
    }
}

fn fifth() {
    // Instantiate a Point
    let p = Point { x: 5.0, y: 7.0 };
    print_point(p);

    // Use an enum
    let dir = Direction::North;
    print_direction(dir);
}

fn sixth() {
    // Ownership and functions
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function and is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's ok to still use x afterward

    // Borrowing
    let s = String::from("hello");
    no_take_ownership(&s);          // s is borrowed, not owned

    // Mutable borrowing
    let mut s = String::from("hello");
    change(&mut s);                 // mutable borrowing
    println!("{}", s);

    // Lifetimes
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        let smallest = smallest_string(&string1, &string2);
        result = smallest;
    }
    println!("Smallest string: {}", result);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn no_take_ownership(some_string: &String) {
    println!("{}", some_string);
} // some_string goes out of scope. Nothing special happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn smallest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
} // Returns a reference to the smallest string

// This function may fail if the string cannot be parsed into an integer
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

fn seventh() {
    match parse_number("10") {
        Ok(num) => println!("It's a number: {}", num),
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("ten") {
        Ok(num) => println!("It's a number: {}", num),
        Err(e) => println!("Error: {}", e),
    }

    // You can also use the `?` operator to propagate the error up
    let number = match "10".parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e.into()),  // convert ParseIntError into a Box<dyn Error>
    };
    println!("Number: {}", number);
}

fn ninths() {
    // Closures
    let add = |x, y| x + y;
    println!("5 + 3 = {}", add(5, 3));

    let numbers = vec![1, 2, 3, 4, 5];

    // Iterators
    // Using `map` to transform each element in the iterator
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers: {:?}", doubled);

    // Using `filter` to select certain items
    let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);
    
    // Using `find` to get the first match
    let first_greater_than_three = numbers.iter().find(|&&x| x > 3);
    match first_greater_than_three {
        Some(val) => println!("First number greater than 3: {}", val),
        None => println!("No number greater than 3 found"),
    }
}

struct Circle {
    radius: f64,
}

impl Circle {
    // Associated function (like static methods in other languages)
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Method (works on an instance)
    fn area(&self) -> f64 {
        3.14159265358979323846 * self.radius * self.radius
    }
}

// A function with lifetimes
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn tenths() {
    let circle = Circle::new(5.0);  // Call associated function
    println!("Circle area: {}", circle.area());  // Call method

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
    }
    println!("Longest string: {}", result);
}

// Define an enum to represent a web event
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

// Function to process a web event
fn process_event(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page unloaded!"),
        WebEvent::KeyPress(c) => println!("Key '{}' pressed!", c),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
    }
}

fn elevenths() {
    let pressed_key = WebEvent::KeyPress('x');
    let click_position = WebEvent::Click { x: 20, y: 80 };

    process_event(pressed_key);
    process_event(click_position);
}

// Define a trait named `Printable`
trait Printable {
    fn format(&self) -> String;
}

// Implement `Printable` for `i32`
impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

// Implement `Printable` for `String`
impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", *self)
    }
}

// A generic function that takes a `Printable` type
fn print_it<T: Printable>(item: T) {
    println!("{}", item.format());
}

fn twelvth() {
    let my_string = String::from("Hello");
    let my_int = 5;

    print_it(my_string);
    print_it(my_int);
}

// Function that might fail, returning an Option
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Function that might fail, returning a Result
fn sqrt(number: f64) -> Result<f64, String> {
    if number < 0.0 {
        Err("Cannot take the square root of a negative number.".to_string())
    } else {
        Ok(number.sqrt())
    }
}

fn thirteenth() {
    match divide(5.0, 0.0) {
        Some(result) => println!("Division result is {}", result),
        None => println!("Cannot divide by zero"),
    }

    match sqrt(-9.0) {
        Ok(result) => println!("Square root is {}", result),
        Err(error) => println!("Error: {}", error),
    }
}

// Define a module named "math"
mod math {
    // Private function (not accessible outside the module)
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Public function (accessible outside the module)
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // Public nested module
    pub mod advanced {
        pub fn power(base: i32, exponent: i32) -> i32 {
            (0..exponent).fold(1, |acc, _| acc * base)
        }
    }
}

fn fourteenth() {
    // Accessing the public function from the math module
    println!("5 * 3 = {}", math::multiply(5, 3));

    // Accessing the public function from the nested module
    println!("2 power 3 = {}", math::advanced::power(2, 3));
    
    // This line would cause a compile error, because the `add` function is private
    // println!("5 + 3 = {}", math::add(5, 3));
}

// A function that returns a closure
fn multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    |number| number * factor
}

// A higher-order function that applies a function to a value
fn apply<F>(func: F, value: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    func(value)
}

fn fifteenth() {
    // Simple closure that captures the `factor` variable
    let factor = 3;
    let triple = |x| x * factor;

    println!("Triple of 5 is: {}", triple(5));

    // Using a function that returns a closure
    let double = multiplier(2);
    println!("Double of 5 is: {}", double(5));

    // Using a higher-order function
    println!("Triple of 7 using apply: {}", apply(triple, 7));
}

// A simple function that simulates a heavy computation
fn heavy_calculation(number: i32) -> i32 {
    println!("Computing for number {}", number);
    thread::sleep(Duration::from_secs(2));
    number * number
}

fn sixteenth() {
    // Spawn a new thread to run the heavy_calculation function
    let handle = thread::spawn(|| {
        let result = heavy_calculation(5);
        println!("Result from thread: {}", result);
    });

    // Do some work in the main thread as well
    for i in 1..5 {
        println!("Main thread working on: {}", i);
        thread::sleep(Duration::from_millis(300));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}

fn seventeenth() {
    // Create a simple channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread
    thread::spawn(move || {
        let values = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread!"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_millis(600));
        }
    });

    // Receive messages in the main thread
    for received in rx {
        println!("Received: {}", received);
    }
}

// Define a simple recursive data structure using Box
enum List {
    Cons(i32, Box<List>),
    End,
}

fn eighteenth() {
    // Using the Box to create a recursive data structure
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(End))))));

    // Function to compute the sum of the list
    fn sum_list(l: &List) -> i32 {
        match l {
            Cons(value, next) => value + sum_list(next),
            End => 0,
        }
    }

    println!("Sum of list is: {}", sum_list(&list));
}

#[derive(Debug)]
struct Data {
    value: i32,
}

fn nineteenth() {
    let data = Rc::new(Data { value: 42 });

    println!("Reference count after creating data: {}", Rc::strong_count(&data));

    {
        let reference1 = Rc::clone(&data);
        let reference2 = Rc::clone(&data);
        
        println!("Reference count after creating two references: {}", Rc::strong_count(&data));

        // Accessing data through one of the references
        println!("Data value from reference1: {:?}", reference1);
    }

    println!("Reference count after inner scope ends: {}", Rc::strong_count(&data));
}

#[derive(Debug)]
struct Data {
    value: RefCell<i32>,
    siblings: RefCell<Vec<Rc<Data>>>,
}

fn twenties() {
    let data = Rc::new(Data {
        value: RefCell::new(42),
        siblings: RefCell::new(Vec::new()),
    });

    let sibling1 = Rc::new(Data {
        value: RefCell::new(10),
        siblings: RefCell::new(Vec::new()),
    });

    let sibling2 = Rc::new(Data {
        value: RefCell::new(20),
        siblings: RefCell::new(Vec::new()),
    });

    data.siblings.borrow_mut().push(Rc::clone(&sibling1));
    data.siblings.borrow_mut().push(Rc::clone(&sibling2));

    // Modifying value inside RefCell
    *data.value.borrow_mut() += 10;

    println!("Updated data value: {:?}", data.value);
    println!("Data siblings: {:?}", data.siblings.borrow().len());
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Click { x: i64, y: i64 },
}

fn web_event_handler(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page unloaded!"),
        WebEvent::KeyPress(c) => println!("Key '{}' pressed!", c),
        WebEvent::Click { x, y } => println!("Clicked at x={}, y={}", x, y),
    }
}

fn twenty_first() {
    let pressed = WebEvent::KeyPress('x');
    let clicked = WebEvent::Click { x: 20, y: 80 };

    web_event_handler(pressed);
    web_event_handler(clicked);
}

fn twenty_second() {
    // Simple closure with no parameters
    let greet = || {
        println!("Hello, Rust!");
    };

    greet();

    // Closure with parameters
    let add = |x, y| {
        x + y
    };

    let result = add(5, 7);
    println!("5 + 7 = {}", result);

    // Closure that captures environment variables
    let factor = 3;
    let multiply = |x| x * factor;

    println!("10 times factor = {}", multiply(10));
}