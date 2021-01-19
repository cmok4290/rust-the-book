fn main() {
    println!("if Expressions");
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // BAD: condition must be a bool
    // if number {
    //     println!("number was three");
    // }
   
    if number != 0 {
        println!("number was something other than zero");
    }

    // multiple conditions
    println!("Handling Multiple Conditions with else if");
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("Using if in a let Statement");
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    println!("Repeating Code with loop");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    println!("Conditional Loops with while");
    let mut loop_number = 3;

    while loop_number != 0 {
        println!("{}!", loop_number);

        loop_number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("Looping Through a Collection with for");
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
    println!("LIFTOFF!!!");
}
