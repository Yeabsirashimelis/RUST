/*
//if statement
fn main() {
    let some_condition = true;
    // the condition should only be boolean. in other languages we can use integers other than 0 as true, but that is not applicable in Rust

    if some_condition {
        println!("this line will be output");
    }

    if false {
        println!("this line will not be output");
    }
}
 */

/*

//ELSE IF STATEMENT - when we have multiple related conditional statements
// we can use the else if only after an initial if statement
//it helps a lot in avoiding redundant check if the above condition is true
//else - a code that will be executed when every condition fails.

fn main() {
    let season = "winter";

    if season == "summer" {
        println!("school is out!");
    } else if season == "winter" {
        println!("oww, so cold!");
    } else if season == "fall" {
        println!("leaves falling!");
    } else if season == "spring" {
        println!("lots of rain");
    } else {
        println!("the season provided is not correct.");
    }
}
 */
/////////////////
///
/*
//ASSIGNING THE RESULT OF IF STATEMENT TO A VARIABLE - it is like a ternary operator in other programming languages
fn main() {
    even_or_odd(15);
    even_or_odd(100);
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("the number is {result}");
}
*/
////////////////////////////
///
//THE MATCH STATEMENT - it is like a SWITCH case in other programming languages.
//can react to all possible variants of a value
/*
fn main() {
    let evaluation = false;

    // match evaluation {
    //     true => {
    //         println!("the value is true");
    //     }
    //     false => {
    //         println!("the value is false");
    //     }
    // }

    //assigning a value to a variable in match statement
    //  from both arms we can only return the same type of data
    let value = match evaluation {
        true => 1,
        false => 0,
    };
}
 */

/*
fn main() {
    let season = "fall";

    // if season == "summer" {
    //     println!("school is out!");
    // } else if season == "winter" {
    //     println!("oww, so cold!");
    // } else {
    //     println!("lots of rain!");
    // }

    //refactor the if else to match
    // _ - is like an else statement / default in other prog languages switch statement
    match season {
        "summer" => println!("school is out"),
        "winter" => println!("oww. so cold"),
        _ => println!("lots of rain"), //this should be the last statement
    }
}
 */

 /*
///////////////
//THE MATCH STATEMENT WITH MULTIPLE VALUES AND CONDITIONALS
fn main() {
    let number = 8;

    // match number {
    //     2 | 4 | 6 | 8 => println!("number is even"),
    //     1 | 3 | 5 => println!("number is odd"),
    //     _ => println!("unknown for now"),
    // }

    // we can use  any name. we use "value" here.
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        //we handle every integer, but Rust is not smart enough to know that, so we need to use the underscore
        _ => unreachable!(), //this is a macro for unreachable arm
    }
}
 */
/////////////////////
///

/*
//ITERATION
// iterate - means to repeat, to do something over and over again
//the LOOP and BREAK key word
//the CONTINUE key word - forces the loop to move to the next iteration.
fn main() {
    let mut seconds = 21;
    loop {
        if seconds <= 0 {
            println!("blastoff!!!");
            break;
        }

        if seconds % 2 == 0 {
            println!("{seconds} seonds (even number) seconds...");
            seconds -= 3;
            continue;
        }
        println!("{seconds} seconds to blastoff... ");
        seconds -= 1;
    }
}
 */
/////////////////
///
/*
//the WHILE loop - continues iterating as long as a condition is met.
fn main() {
    let mut seconds = 21;
    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seonds (even number) seconds...");
            seconds -= 3;
            continue;
        }
        println!("{seconds} seconds to blastoff... ");
        seconds -= 1;
    }

    println!("blastoff!!!");
}
*/

/*
///////////////
//RECURSION - is when a function calls itself.
// base case - is the condition that stops the recursion.
fn count_down(seconds: i32) {
    if seconds == 0 {
        println!("blastoff");
    } else {
        println!("{seconds} seconds to blastoff");
        count_down(seconds - 1);
    }
}

fn main() {
    count_down(5);

    count_down(5);

    count_down(5);
}

// DEBUGGING - is the process of finding and fixing errors in your code.
// BREAKPOINT  - is a designated stopping point in the code. execution will pause before the line is run.
 */
//////////
/// CODING CHALLENGE
fn main() {
    println!(
        "the corresponding number related to the color is {}",
        color_to_number("blue")
    );

    println!("the factorial of the 5 is {}", factorial(5));
}

// fn color_to_number(color: &str) -> i32 {
//     if color == "red" {
//         return 1;
//     } else if color == "green" {
//         return 2;
//     } else if color == "blue" {
//         return 3;
//     } else {
//         return 0;
//     }
// }

fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

// fn factorial(mut number: i32) -> i32 {
//     let mut factorial_value: i32 = 1;
//     while number > 0 {
//         factorial_value *= number;
//         number -= 1;
//     }
//     return factorial_value;
// }

fn factorial(number: i32) -> i32 {
    if number > 0 {
        return number * factorial(number - 1);
    } else {
        return 1; // Base case: factorial(0) = 1
    }
}
