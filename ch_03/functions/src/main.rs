// Functions
fn main() {
    println!("Hello, world!");

    another_function();

    one_parameter(5);

    two_parameters(5, 6);

    // Function Bodies Contain Statements and Expressions
    let x = 5;

    let y = {
        let x = 3;
        x + 1;
    };

    let z = five();

    println!("The value of z is: {}", z);

    let a = plus_one(5);

    println!("The value of a is: {}", a);
}

fn another_function() {
    println!("Another function.");
}

// Function Parameters
fn one_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn two_parameters(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Functions with Return Values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
