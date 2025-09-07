/*
-every RUST value has a data type.
- rust is a statically typed language, which means the compiler
  must know the types of all variables at compile time.
- the compiler can infer the types of variables based on their initial assignments/ if we don't give them explicitly.
*/

/*
fn main() {
    let eight_bit: i8 = -112;
    // let eight_bit: i8 = -129; //this will be an error, b/c the amount of range it can contains is -128 - 127

    let eight_bit_unsigned: u8 = 255; // (0- 255) ✔ - out of this range will give us an error
    let sixteen_bit_signed: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 63000;

    let thirtytwo_bit_signed: i32 = -2100002382;
    let thirtytwo_bit_unsigned: u32 = 4_200_002_382; //we can used underscores as visual separators like we use commas in hand writing.

    let some_value = 20i8;
}
 */

/*
//usize and isize - which takes the size depending on the arichtecture the code is running.
fn main() {
    //usize
    //isize
    // 55 will take 64 bits in 64 bit arichtecure PCs and 32 bits in 32 bit arichtecture PCs.
    let days: usize = 55;
    let years: isize = -15_000;
}
 */

/*
STRINGS - there are differet types of strings.
string literals - are known at compile time and they are what we have been using since this part

/*
 */
fn main() {
    println!("Dear Yeabsira,\nhow have you been?"); // \n - is a special character that forces a line break
    println!("\tOnce upon a time"); //\t - is a tab

    //  \ - used to escape special characters like (\, "",...)
    println!("Juliet said \"I love you Romeo\" "); // \" \" - to use quotes

    // let filePath = "c:\Documents\new\videos"; //this creates a problem b/c rust analyzes \D,\n and \v as special characters and they are not valid.
    let filepath = "c:\\Documents\\new\\videos";

    let filepath2 = r"c:\Documents\new\videos"; // r- row,  means treat all text as row text

    println!("{filepath}");
    println!("{filepath2}");
}
     */

/*
//quick intro to methods - are functions that we call on different types of values
//  --they are just built in functions
fn main() {
    let value: i32 = -15;
    println!("{}", value.abs()); //the return value will be 15.

    let empty_space = "    my content    ";
    println!("{}", empty_space.trim()); //remove the spaces at the beginning and the end of the string
    println!("{}", value.pow(2));
}
 */

/*
//FLOATING POINTS
//has 2 types, 32 bit, and 64 bit (float and double in c++ consecutively)
//they are all signed, there is no unsigned float type
fn main() {
    let pi: f64 = 3.1415926535897932384; //f64 is the default and it has 15 to 17 digits of precision.

    println!("The current value of pi is {pi}");
    println!("The current value of pi is {pi:.2}"); //formating floats with format specifier
    println!("The current value of pi is {:.4}", pi); //we can do this as well.

    println!("{}", pi.floor()); //the integer part of the float
    println!("{}", pi.ceil()); //rounds up to the next whole number.
    println!("{}", pi.ceil()); // round to the closest integer according to the rules
}
 */

/*
//TYPE CASTING - is transforming a value from one type to another.
fn main() {
    //accross different integer types.
    let miles_away = 50;
    let miles_away: i8 = miles_away as i8;
    let miles_away_u8: u8 = miles_away as u8;

    //accross different float types
    let miles_away = 100.329032;
    let miles_away: f32 = miles_away as f32;

    //accross different data types.
    let miles_away_int = miles_away as i64;
    print!("{miles_away_int}");
}
 */

/*
//MATH OPERATIONS
fn main() {
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;

    let floor_division = 5 / 3;
    println!("{floor_division}");

    let decimal_division = 5.0 / 3.0;
    println!("{decimal_division}");

    let remainder = 7 % 2;
    println!("{remainder}")
}
 */

/*
//AUGMENTED ASSIGNMENT OPERATOR
fn main() {
    let mut year = 2025;
    // year = year + 1;
    year += 1; //✔
    println!("The new year is {year}");
}
 */

/*
//BOOLEAN - either true or false
//1 bit is sufficient to store the values. but the RUST uses a full byte for a boolean
//    for performance reasons. b/c accessing individual bits and memory is actually smaller
//   than accessing.
fn main() {
    let is_handsome = true;
    let is_silly: bool = false;

    println!("Handsome: {is_handsome}, Silly: {is_silly}");

    let age: i32 = 21;

    let is_young = age < 35;
    println!("is young : {is_young}");
    println!("{} {}", age.is_positive(), age.is_negative());

    // ! - boolean inverter
    println!("{}", !false);

    let age = 13;
    let can_see_rated_r_movie = age >= 17;
    println!(
        "I am {age} years old. is it forbidden to see this scary movie for me? : {}",
        !can_see_rated_r_movie
    );

    // == - equality operator
    // != - non-equality
    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke");
    println!("{}", 13 == 13.0 as i32);
}
 */

/*//AND LOGIC with &&
fn main() {
    let purchased_ticket = true;
    let plane_on_time = true;
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);
}
 */

/*
//OR logic with || - it needs atleast one true boolean for the entire evaluation to be true
fn main() {
    let user_has_paid_for_subscription = true;
    let user_is_admin = false;
    let user_can_see_premium_experience = user_has_paid_for_subscription || user_is_admin;

    println!("Can this user see my site? {user_can_see_premium_experience}");
}
 */

/*
//CHARACTER TYPE
//UNICODE - is computing industry standsrd for the representation of text for most of the world's writing systems
//          contains more than 140,000 different characters that cover 150 modern and historic languages.
//          acronym is UTF - unicode transformation formats which has several variants, but the most common variant
//            is UTF-8, but there are other options as well like UTF-16 and UTF-32.
//so UNICODE is a standard how peoples agreed upon for modeling how computers are going to store and represent characters.
// CHARACTERS in rust are occupy 4 bytes or 32 bits. technically english letter only need 1 byte, but it is 4 byte to handle all cases like that of emogies.
fn main() {
    let first_initial = 'Y';
    let emoji: char = '🎧';

    println!(
        "{} {}",
        first_initial.is_alphabetic(),
        emoji.is_alphabetic()
    );

    println!("{} {}", first_initial.is_uppercase(), emoji.is_uppercase()); //emogies are neither uppercase nor lowercase
    println!("{} {}", first_initial.is_lowercase(), emoji.is_lowercase()); //emogies are neither uppercase nor lowercase
}
 */

//till now we we learning scalar types.
//now we'll start to learn compound types.

/*
//ARRAY - is a fixed-size collection of homogenous data (data of the same type).
fn main() {
    let _numbers: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("length of apples array : {}", apples.len());

    // let currency_rates = []; //this will be an error as we don't give it a type and it can't annotate b/c it has no elements.
    let _currency_rates: [f64; 0] = [];

    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];
    let first = seasons[0];
    let second = seasons[1];
    println!("The first seasons is {first} and the second season is {second}");
    // println!("{}", seasons[100]); -index out of bound.

    println!("{}", seasons[2]);
    seasons[2] = "Autumn";
    println!("{}", seasons[2]);
}
 */
/////////////////////
//TRAITS
//a trait is a contract that requires a type support one or more methods
//traits establish consistency between types; methods that represent the same behaviour have the same name (across different types)
//so methods that represent the same fundamental behaviour can have the same name across different types.
//we say the type "implements" the trait.
//a type can choose to opting in to implementing a trait
//a type can implement multiple traits. there are hundrads of traits available in Rust.
//a trait is called an "INTERFACE" or protocol in other programming languages.

/*
the DISPLAY trait - requires that a type can be represented as a user friendly, readable string.
the display trait mandates a format method that returns the string.
when we use the {} interpolation syntax, Rust relies on the format method.
integers, floats and booleans all implement the display trait so we are able to interpolate them with curly braces
-not all types implement the Display trait, one example is the array type.
*/

/*
THE DEBUG TRAIT - to format a given type into a porgrammer-facing string for the purposes of debugging.
display trait is more for USERS. the debug trait is intended more for DEVELOPERS.
array types do implement the DEBUG trait and that will allow us to output a string version of the array to the screen.
*/
/*
fn main() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    println!("{}", 5);
    println!("{}", 3.14);
    println!("{}", true);
    // println!("{}", seasons); //array type don't implement the format trait by  Display
    println!("{:?}", seasons); //now we are using the format trait by debug.
    println!("{seasons:?}");
    println!("{seasons:#?}"); //another format specifier that uses the debug trait is #? - pretty print.
}
 */

/*
//the DBG macro - debug macro
// we use just to print different types. for DEVELOPERS, not users
//it is like the debug trait but it is a macro. the debug trait is just a promise, but this is an independent developed macro which uses the format method by the debug trait
fn main() {
    let seasons = ["Spring", "Summer", "Fall", "Winter"];
    dbg!(2 + 2); //it prints not only the result. it print the file location, the line where the code is found, the code inside the debug macro and finally the result.
    dbg!(seasons);
}
*/

/*
//the TUPLE type - its difference with array type is that it allows different datatypes (hetrogenity), array can contain only homogenous type.
//                 so it is a collection type that can contain multiple elements of different types.
fn main() {
    let employee = ("Molly", 32, "Marketing"); //a tuple can contain different types.
                                               // let name = employee.0; //they have index exactly like arrays but the accessing synthax is a little bit different
                                               // let age = employee.1;
                                               // let department = employee.2;
    let (name, age, department) = employee;

    println!("NAME: {name}, age :{age}, department: {department}");
    println!("{employee:?}");
    println!("{employee:#?}");
    dbg!(employee);
}
 */

/*
//RANGES AND RANGE ITERATION - the RANGE type represents a sequence or interval of consecutive values.
// we can use array or tuples for this but we have to declare each element one by one.
fn main() {
    let month_days = 1..31; //the upper value is exclusive. this will contain elements from 1 to 30
    println!("{month_days:?}");

    let month_days = 1..=31; // the upper value is enclusive in this type.
    println!("{month_days:?}");

    //iteration through the range and do some operation.
    for number in month_days {
        print!("{number}, ");
    }

    println!("");

    let letters = 'b'..'f';
    for letter in letters {
        print!("{letter} ");
    }

    println!("");

    let colors = ["green", "yellow", "red"];
    //we can use for loop for arrays as well
    for color in colors {
        print!("{color} ");
    }
}
 */

/*
//intro into GENERICS
// / a generic is a type argument, not a concrete value
//which is found b/n <>.
/*
Range type is like a box. it can hold different things inside it.
but what type of data we want to store in it? we should use type arguments.
-Generics exist so that we can provide types like this one as their own arguments.
- it is like a placeholder for the coming concrete values
*/
fn main() {
    // <> - are called angle brackets
    // let month_days = 1..31;
    //the range type is found in std module, the operation folder / we can think this structure like a folder structure in our PC.
    let month_days: std::ops::Range<i32> = 1..31;
    // let letters = 'b'..'f';
    let letters: std::ops::Range<char> = 'b'..'f';
}
 */

/////////////////////////////
///
///
///
//CODING CHALLENGE

fn main() {
    let number: i32 = 1_337;
    let _number: i16 = number as i16;

    let f_number: f64 = 4.2842384762;
    println!("the float number is: {:.3}", f_number);

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let _is_my_type_of_coffee = with_milk && with_sugar;

    let _is_acceptable_coffee = with_milk || with_sugar;

    let numbers_arr: [i8; 4] = [1, 2, 3, 4];
    println!("the array in debug mode print is {numbers_arr:?}");
    // println!("the array in debug mode print is {numbers_arr:#?}"); //for pretty print
    dbg!(numbers_arr); //using debug macro

    let tuple = (2, 4.56, false, numbers_arr);
    println!("the tuple: {:?}", tuple);
    println!("the tuple: {:#?}", tuple); //for pretty print
}
