// FUNCTIONS - is a sequence of steps to be executed in order
//the power of functions lies in their ability to capture a reuseable collection of instructions.
//parameters - is a name for an expected input to a function.
//argument - is a concrete value passed in for a parameter when the function is invoked.
/*
fn main() {
    open_store("Debre-Zeit");
    bake_pizza(10, "pepperoni");
    swim_in_profit();
    swim_in_profit();
    swim_in_profit();
    open_store("Addis Ababa");
    bake_pizza(15, "mushroom");
}

fn open_store(neighborhood: &str) {
    println!("opening my pizza store in {neighborhood}.");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas.");
}

fn swim_in_profit() {
    println!("so much time. so little time.");
}
 */

/*
//////////////////
//explicit return values - the output of a function to the caller.
// no code can run after the return key word. it is the conclusion for the function.
//implicit return -
fn main() {
    let result = square(5);
    println!("the square of 5 is {result}");

    let result = square(12);
    println!("the square of 12 is {result}")
}

// fn square(number: i32) -> i32 {
//     return number.pow(2);
//     // return number * number;
// }

//for implicit return we need to make the statement appear at last without a semicolon
fn square(number: i32) -> i32 {
    number.pow(2)
}
 */

///////////

/*
//A UNIT AS A RETURN TYPE - a unit is an empty tuple. a tuple without values.
// tuple is like this, let tuple = (2, 4.5, "yeabsira");
fn main() {
    // let result = (); //this is a unit
    //unit -  is a default return when there is no explicit or implicit return value is given
    let result = mystery();
    dbg!(result);
}

fn mystery() {
    println!("hello world");
}
 */

/*
//block
// a block create an independent execution environment without the need to create a new function
fn main() {
    let multiplier = 3;

    //this is a block
    let calculation = {
        let value = 5 + 4;
        value * multiplier // no semi-colon as this will be the returned value from this block and stored in the variable
    };

    println!("{calculation}")
}
 */

//////////////////////
/// CODING CHALLENGE
fn main() {
    apply_to_jobs(21, "Rust Developer");
    let result = is_even(8);
    println!("{result}");
    let result = is_even(9);
    println!("{result}");

    println!("{:?}", alphabets("aardvark"));
    println!("{:?}", alphabets("zoology"));
    println!("{:?}", alphabets("zebra"));
}

fn apply_to_jobs(number: i32, title: &str) {
    println!("I am applying to {number} {title} jobs.");
}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}

fn alphabets(text: &str) -> (bool, bool) {
    let contain_a = text.contains('a');
    let contain_z = text.contains('z');

    return (contain_a, contain_z);
}
