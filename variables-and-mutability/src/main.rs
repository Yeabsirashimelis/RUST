/*fn main() {
    let apples = 50; //i32, i - rust is inferring that the type of the data is integer and 32 - the amount of memory the variable takes in the memory
    let oranges = 14 + 6;
    let _fruits = apples + oranges;

    //we can use both of the following to print dynamic values (interpolation)
    println!(
        "This year, my garden has {} apples and {} oranges.",
        apples, oranges
    );

    //this saves us from reapeating write the argument
    println!(
        "This year, my garden has {0} apples and {1} oranges. I can't believe I have {0} apples.",
        apples, oranges
    );
    println!("This year, my garden has {apples} apples and {oranges} oranges.");
}
*/

//immutability - the value assigned to the variable nae can not chnage during the program execution
// fn main() {
//     // let gym_reps = 10;
//     let mut gym_reps = 10;
//     println!("I plan to do {gym_reps} reps");

//     // gym_reps = 15; -this is error b/c by default variables are immutable in Rust
//     gym_reps = 15;
//     println!("I now plan to do {gym_reps} reps");

//     // gym_reps = "string";  -even if the variable is mutable, we can't change the type of it.
// }

//VARIABLE SHADOWING -variable shadowing means redeclaring a variable. the original variable is "replaced" by the new one.
//we use variable shadowing when we want to perform different transformation on a value and reuse a variable b/ the variable name fundamentlally represents the value in different stages
//variable shadowing is applicable in the same scope. if we declare in different scope they are considered as different variables
// fn main() {
//     let grams_of_protein = "100.345"; //this will be active until the second one is declared
//     let grams_of_protein = 100.345;
//     let mut grams_of_protein = 100; //the last declared will be the active one
//     grams_of_protein = 101
// }

// // BLOCK / SCOPE - block is an area that is bounded by a curly braces.
// //   and variables declared in a scope is valid only in that scope.

// fn main() {
//     let coffee_price = 5.99;

//     {
//         let coffee_price = 4.99;
//         println!("The price is ${coffee_price}"); //give priority to the inner one.
//         let cookie_price = 1.99; //this variable is valid only in this inner scope
//         println!("The cookie price is ${cookie_price}");
//     }

//     // println!("The cookie price is ${cookie_price}"); //this is error
//     println!("The price is ${coffee_price}");
// }

/*
//CONSTANT - is varibale that its value never change during the program execution.
//             NO mut
so what is the difference b/n immutable variables and constants
- B/C CONSTANTS CAN BE DECLARED AT THE FILE LEVEL (OUTSIDE FUNCTIONS) - so they can have access in any functions in that file
-constants are known by the compiler at build time, so cannot use them at run time
   like store user inputs
--therefore they take hardcoded values that will be used in the program
 */

// const TAX_RATE: f64 = 0.05; //for constants we have to write an type explicitly
// fn main() {
//     let income: i32 = 1000;
//     println!("My income is {income} and The tax rate is {TAX_RATE}")
// }

// #![allow(unused_variables)] //this will apply to the entire file. the synthax is a little bit different(!), but it is to tell
//    the compiler not just the next line, it is the entire file.

// // TYPE ALIASES -is an alternative name that we can assign to an exsiting type
// type Meters = i32;
// // COMPILER DIRECTIVES - is an annotation that tells the compiler how to parse the source code. it is like a meta data for the compiler

// #[allow(unused_variables)] //this will apply to the entire function
// fn main() {
//     //now we know that the 2 variables model the same idea, not only the same type(random integer)
//     // #[allow(unused_variables)] // - this will apply directly to the line bottom to it
//     //now the compiler don't give us the warning of unused variables
//     let mile_race_length: Meters = 1600;
//     // #[allow(unused_variables)] // - this will apply directly to the line bottom to it
//     let two_mile_race_length: Meters = 3200;
// }

const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    let season: &str = "winter";
    let mut points_scored: i32 = 28;
    points_scored = 35;
    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!(
        "My favourite season is {season}. 
    the last value of points scored was {points_scored}.
    the eventtime was string but now shadowed to integer and its value is {event_time}.
    the touchdown has a point {TOUCHDOWN_POINTS}.",
    );

    #[allow(unused_variables)]
    let favorite_beverage: &str = "coca-cola";
}
