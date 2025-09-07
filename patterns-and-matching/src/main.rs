//PATTERNS AND MATCHING
/*
patterns are a special syntax in Rust for matching against the structure of types, both complex and simple
using patterns in conjuctions with match expressions and other constructs gives you more control over a program's control flow.
a pattern consistss of some combination of the following

 -literals
 -distructured arrays, enums, structs, tuples
 -variables
 -wildcards
 -placeholders
*/

//ALL THE PLACES PATTERNS CAN BE USED
/*
patterns pop up in a number of places in RUST and you have been using them a lot without realizing it!

MATCH ARMS
for eg :- the patterns in the match expressions are None and Some(i) to the left of each arrow

one requirement for "match expression" is that they need to be exhaustive in the sense that all possibilities
 for the value in the match expression should be accounted for. one way to ensure you've covered every possibility is
 to have a catchall pattern for the last arm.
  match X {
  None => None,
  Some(i) => Some(i + 1)
  }

*/

/*
//CONDITIONAL IF LET EXPRESSIONS
/*
the downside of using the if let expressions is that the compiler doesnot check for exhanstiveness, where as
  the match expressions does. if we omitted the last else block and therefor handling some cases, the compiler would not alert us
  to the possible logic bug

the code determines what color to make your background based on a series of checks for several conditions
 */
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as a background");
    } else if is_tuesday {
        println!("Tuesday is a green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
*/

/*
//////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
//WHILE LET CONDITIONAL LOOPS
/*
similar in construction to if let, the while let conditional loop allows a while loop to run for as
 long as a pattern continues to match.

 in the code below, we show a while let loop that waits on messages sent between threads, but in this case checking a Result
  instead of an Option
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    //we can use this pattern matching b/c the "recv" method returns Ok each time a message arrives, as long as the sender exists
    //   and then produces an Err once the sender side disconnects
    while let Result::Ok(value) = rx.recv() {
        println!("{}", value);
    }
}
 */

/*
/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////
//FOR LOOPS
/*
in a for loop, the value that directly follows the keyword for is a pattern. for eg, in "for x in y", the x is the pattern

the code blow demostrates how to use a pattern in a for loop to destructure, or break apart, a tuple as part of the for loop.
*/

fn main() {
    let v = vec!['a', 'b', 'c'];

    /*
    we adapt an iterator using the enumerate method so it produces a value and the index for that value, placed into a tuple.
    the first value produced is the tuple (0, 'a'). when this value is matched to the pattern (index, value), index will be 0
    and value will be 'a'. GOTCHA😁
     */
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
*/

/*
/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////
//LET STATEMENTS
/*
 everytime you've used a let statement like this you've been using patterns, although you might not have realized it!
 let pattern = expression;
*/

fn main() {
    let x = 5;

    // to see the pattern-matching aspect of let more clearly, consider the following line which uses a pattern matching with let to distructure a tuple
    /*
    here we match a tuple against the pattern. Rust compare the value (1, 2, 3) to the pattern (x, y, z) and sees that the value matches the pattern,
     in that the number of elements is the same in both, so Rust bind 1 to x, 2 to y and 3 to z. you can think this tuple pattern as nesting three individual variable patterns inside it.

    if the number of elements in the pattern doesn't match the number of elements in the tuple, the overall type won't match and we will get a compiler error.

     */
    let (x, y, z) = (1, 2, 3);
}
 */

/*
////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
//FUNCTION PARAMETERS
/*
the "x" part is a pattern
 */
fn foo(x: i32) {
    //code goes here
}

fn print_coordinated(&(x, y): &(i32, i32)) {
    println!("current location: ({} {} )", x, y);
}
fn main() {
    //the value passed matched the pattern of the parameters of the function
    let point = (3, 5);
    print_coordinated(&point);
}
*/
////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
//REFUTABILITY: WHETHER A PATTERN MIGHT FALL TO MATCH
/*
Patterns come in two forms: refutable and irrefutable
 patterns that will match for any possible value passed are irrefutable. eg: let x = 5;
 patterns that fail to match for some possible value are refutable. an example would be Some(x) in the expression
    if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match

let statements and for loops can only accept irrefutable patterns because the program canot do anything meaningful when values don't match.
if let, while let and the let else statement accept statement accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns
    b/c they are intended to handle possible failure: the functionality of a conditional is its ability to perform differently  depending on success or failure
*/

/*
fn main() {
    /*
      let's look at an example of what happens when we try to use a refutable pattern when RUST requires an irrefutable pattern and vice versa
      the bottom line of code will not compile. b/c Some(x) pattern is irrefutable patterns, b/c there is nothing valid code can do with a None value.
      Rust will complain that we've tried to use a refutable pattern where an irrefutable pattern is required
    */
    // let Some(x) = Some(3); //b/c we didnot cover (couldn't cover) every valid value with the pattern Some(x), Rust rightfully produces a compiler error

    /*
       if we have a refutable pattern where irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let.
       then if the pattern doesnot match, the code will just skip the code in the curly brackets, giving it a way to continue validly.
    */
    //so this works now
    let Some(x) = Some(4) else {
        return;
    };

    /*
    what if we give if let an irrefutable pattern (a pattern that will always match) such as the line of code below
      IT will give us a warning that "the else block is useless"
     */
    let x = 5 else {
        return;
    };
}
 */

/*
 for this reason, match arms must use refutable patterns, except for the last arm,
   which should match any remaining values with an irrefutable pattern. Rust allows us
   to use an irrefutable pattern in a match with only one arm, but this syntax is not
   particularly useful and could be replaced with a simpler let statement
*/

/*
////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////
//MATCHING NAMED VARIABLES
fn main() {
    let x = Some(5);
    let y = 10;

    /*
    The pattern in the first match arm doesnot match the defined value of x, so the code continues.
     the pattern in the second match arm introduces a new variable named y that will match the value inside a Some value.
     because we are in the new scope inside the match expression, this is a new y variable, not the y we declared at the beginning
     with the value 10. this new y binding will match any value inside a Some, which is what we have in x. therefore, this new y binds
     to the inner value of the Some in x. that value is 5, so the expression for the arm executes and prints matched y = 5.
     */
    /*
    to create a match expression that compares the values of the outer x and y, rather than introducing a new variable
    that shadows the existing y variable, we would need to use a match guard conditional instead. we will talk about that later
     */
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case,  x={:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
 */

/*
///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
//MULTIPLE PATTERNS
/*
  you can match multiple patterns using the | syntax, which is the pattern or operator.
   for eg :- look at the folowing code
*/
fn main() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
 */

/*
///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
//MATCHING RANGES OF VALUES WITH ..(exclusive) & ..=(inclusive)
fn main() {
    let x = 5;
    match x {
        1..=5 => println!("one through 5"),
        _ => println!("something else"),
    }

    match x {
        1..5 => println!("one through 5"),
        _ => println!("something else"),
    }

    //REMEMBER - RANGE CAN ONLY BE USED FOR "NUMERIC VALUES" AND "CHARACTERS"
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
 */
/////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////
//DESTRUCTURING TO BREAK APART VALUES
/*
 we can also use patterns to destructure structs, enums, and tuples to use different parts of these values.
*/
/*
//DESCTURING  STRUCTS
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p = Point { x: 0, y: 7 };

    //this code creates the variables a and b that match the values of the x and y fields of the p struct.
    let Point { x: a, y: b } = p;

    //if we want to distructure with the same name as the struct fields, which is more common
    let Point { x, y } = p;

    // assert_eq!(dude1, dude2);
    /*
    assert_eq! checks if two values are equal.
    If they are equal, the program continues.
    If they are not equal, Rust panics (crashes) and prints a message showing both values.
         */
    assert_eq!(0, a);
    assert_eq!(7, b);

    /*
    we can also destructure with literal values as part of the struct pattern rather thatn creating variables for all the fields.
    Doing so allows us to test some of the fields for particular values while creating variables to destructure other fields
       in the bottom code we have a match expression that separates Point values in 3 cases : points that lie directly
       on the x axis(which is true when y=0), on the y axis(x=0) or neither axis.
     */
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis : ({} {})", x, y),
    }
}
 */

/*
//DESTRUCTURING ENUMS
/*
    the number of variables in the pattern must match the number of elements in the variant we are matching
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("the quit variant has no data to destruture.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message : {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {}, green {} and blue {}", r, g, b);
        }
    }
}
 */
/*
/////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////
//DESTRUCTURING NESTED STRUCTS AND ENUMS

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            );
        }
        _ => (),
    }
}
 */

/*
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
//DISTRUCTURING STRUCTS AND TUPLES
/*
  we can mix, match, and nest destructuring patterns in even more complex ways. the following
   example shows a complicated destructure where we nest structs and tuples inside a tuple and destructure all the
   primitive values out
*/

struct Point {
    x: i32,
    y: i32,
}
fn main() {
    //the code lets us break complex types into their component parts so we can use the values we're interested separately
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
 */
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
//IGNORING VALUES IN A PATTERN
/*
  you have seen that it is sometimes useful to ignore values in a pattern, such as in the last arm of a match,
  to get a catchall that doesn't actually do anything but does account for the remaining possible values.
  there are a few ways to ignore entire values or parts of values in a pattern: using the _ pattern(which you have seen),
  using the _ pattern within another pattern, using a name that starts with underscore, or using .. to ignore remaining parts of a value.
    let's explore how and why to use each of these patterns
*/

/*
//AN ENTIRE VALUE WITH _
//completely ignore the first value
//  ignoring a function parameter can be especially useful in cases when, for eg, you're implementing a trait when you
//    need a certain type signiture but the function body in your implementation doesnot need one of the parameters.
fn foo(_: i32, y: i32) {
    println!("this code only uses the y parameter : {}", y);
}

fn main() {
    foo(3, 4);

    //PARTS OF VALUE WITH NESTED_
    /*
     when we want to test for only part of a value but have no use for the other parts in the corresponding code we want to run.
    */
    //example case : the business requirements are that the user should not be allowed to overwrite an existing customization of a setting
    //                but can unset the setting and give it a value if it is currently unset.

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => setting_value = new_setting_value,
    }
    println!("setting is {setting_value:?}");

    //we can also use underscores in multiple places within one pattern to ignore particular values.
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbrers:{}, {}, {}", first, third, fifth);
        }
    }

    //UNUSED VARIABLE BY STARTING ITS NAME WITH _
    let _x = 5;
    let y = 5;

    /*
    the difference b/n _ and _variable. -> the variable bind a value.
     */
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s); //so that this won't compile, ownership moved

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s); //but this compiles, b/c we  never bind s to anything. it isnot moved
}
*/
/*
////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////
//REMAINING PARTS OF A VALUE WITH ..
/*
  with values that have many parts, we can use the .. syntax to use specific parts and ignore the rest,
  avoiding the need to list undescores for each value. The .. pattern ignores any parts of a value that we haven't
  explicitly matched in the rest of the pattern
*/
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };

    /*
    we list the x value and just include the .. pattern. this is quicker than having to list y: _ and z : _,
    particularly when we're working with  structs that have lots of fields in situations where only one or two fields are relevant.
     */
    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    //we use use ..(remaining part) for tuples as well
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers : {}, {}", first, last);
        }
    }

    /*
    how ever using .. must be unambigous. if it is unclear which values are intended for matching and which should be ignored
    rust will give us an error
     */
    /*
    //THIS WILL NOT COMPILE
    match numbers {
        (.., second, ..) => {
            println!("some number:{}", second);
        }
    }
     */
}
     */
/*
//////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////
//EXTRA CONDITIONALS WITH MATCH GUARDS
/*
 a match guard is an additional if condition specified after the pattern in a match arm, that must also match for that arm to be choosen.
 match guards are useful for expressing more complex ideas than a pattern alone allows. Note, however, that they are only available in match expressions,
   not in if let or while let expressions.

   the downside of this additional explissiveness is that the compiler doesnot try to check for exhaustiveness when the match guard expersions are involved
Because guards are runtime checks, not compile-time checks.
The compiler can’t prove exhaustiveness when a guard is present, so it relaxes the rule and assumes you “know what you’re doing.”
*/
fn main() {
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("the number {x} is even"),
        Some(x) => println!("the number {x} is odd"),
        None => (),
    }

    //we mentioned that we could use match guards to solve our pattern-shadowing problem.
    // recall that we created a new variable inisde the pattern in the match expression instead of using the variable outside the match.
    //   that new variable meant we couldnot test against the value of the outer variable.
    //solve this using the match guard expression
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("matched, n = {n}"), //there is no variable n to shadow in the outer scope.
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {x:?}, y = {y}");

    //you can also use the or operator | in a match guard to specify multiple patterns; the match guard condition will apply to all the patterns.
    let x = 4;
    let y = false;

    //the match condition states that the arm ony matches if the value f x is equal to 4, 5, 6 and if y is true.
    // "no" => will be printed
    match x {
        4 | 5 | 6 if y => println!("yes"), //if y will apply for all of them
        _ => println!("no"),
    }
}
 */
/////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////
//@ BINDINGS
/*
that at operator (@) let's us create a variable that holds a value at the same time we're testing that value
 for a pattern match.
*/
enum Message {
    Hello { id: i32 },
}
fn main() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..7,
        } => println!("Found an id in range : {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            //the id variable is not available to use here. b/c it could have been 10, 11, 12
            println!("find an id in another range")
        }
        Message::Hello { id } => println!("found some other id: {}", id),
    }
}
