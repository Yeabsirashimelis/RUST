//&str
//String
/*
fn main() {
    //after we assign a string literal this text will be embedded into a binary executable.
    // and when the program runs, we're going to get a reference or a borrow of that complete fragment of text living fragment of text living somewhere else.
    let pirate = "Bloodhook";

    //heap string
    let sailer = String::from(pirate);
    let bad_guy = pirate.to_string();

    //we CAN'T use indexes to access elements of strings. b/c the indexing in Strings works
    //  by counting bytes, not characters. and emogies and other non-eng charas could be more than one byte.
    // let first_initial = &sailer[0];

    //so the only way that we can access individual characters in a string through a byte sequence.
    //here similarly it depends on bytes. but we can correct the number of bytes for that character
    // like [0..4](if the character took 4 BYTES).
    let first_initial = &sailer[0..1];
}
 */

/*
//CONCATENATION
// push_str- it will add a string literal at the end of the heap string.
//push - it will add a character literal at the end of the heap string.
// "+" sign - it transfers ownership. so the string to be added should be a string literal or should be a String reference to allow deref coertion
fn main() {
    let mut full_name = String::from("Yeabsira");
    full_name.push(' ');
    let last_name = "Shimelis";
    full_name.push_str(last_name);
    println!("{full_name}");

    let last_name = String::from("Shimelis");
    //so this is not allowed
    // full_name.push_str(last_name);

    //but giving reference of a String is allowed
    let mut a = String::from("abc");
    let b = String::from("def");
    //this is allowed with  a concept of deref coertion. &String -> str
    let c = a.push_str(&b);

    let first_name = String::from("Abyot");
    // let last_name = "Gizaw";
    let last_name = String::from("Gizaw");

    // let first_name = first_name + last_name;
    //OR
    let first_name = first_name + &last_name;
    println!("{full_name}");
}
 */

/*
//THE FORMAT MACRO - similar to println, but instead of printing a string, it returns a formatted string, including an interpolated contents that we inject within it.
//if we want to easily interpolate one or more values or calculations into a heap based string and then produce
//it or return it instead of printing it, and once we return it, we can of course assign it to a variable or use it as the return value of a function
fn main() {
    let first_name = String::from("Yeabsira");
    let last_name = "Shimelis";

    //it don't move ownership. it just take the references behind the scenes
    let icon = format!("{first_name} {last_name}");
    println!("{icon}");
}
 */

/*
//COMMON STRING METHODS - trim, casing, replace, split
//those methods only take the reference to the strings. so no move of ownership occurs by using this methods
fn main() {
    let mut music_genres = "     Rock, Metal, Country, Rap   ";
    println!("{}", music_genres);
    //the TRIM() method removes the extra spaces at the start and the end of a String
    println!("{}", music_genres.trim()); //the method take the reference of the string to be trimmed

    music_genres = music_genres.trim();
    //their name indicates their job
    // println!("{}", music_genres.trim_start());
    // println!("{}", music_genres.trim_end());

    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());

    //REPLACE - is going to replace all matches of a given character or a pattern.
    println!("{}", music_genres.replace("a", "@"));

    //SPLIT - is helpful method when we want to split or seperate a string based on every occurence of a character, which is often called a delimeter.
    let genres = music_genres.split(", ");
    //if we want to collect the elements into a vector we can use the collect method
    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);
}
 */

/*
/////////////////
///////////////
//COLLECTING USER INPUTS WITH READ_LINE METHOD
use std::io;
// io - input/ output
fn main() {
    let mut name = String::new();
    println!("what is your name : ");

    //the read_line method expects a mutable reference string for a heap allocated String.
    //and it returns the resilt enum
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("failed to collect input from the user");
    // println!("Hello, {name}");

    //but it is better to use the match keyword to ectract the value
    match io::stdin().read_line(&mut name) {
        Result::Ok(_) => println!("Hello, {}", name.trim()),
        Result::Err(error) => println!("There was an error : {}", error),
    }
}
 */

use std::io;

///////////////////
///////////
/// CODING CHALLENGE
fn main() {
    let mut value = String::from("1000");
    make_money(&mut value);

    let capital_name = trim_and_capitalize("   yeabsira  ");
    println!("Capital Name : {}", capital_name);

    elements("Gold!Silver!Platinum");

    let full_name = get_identity();
    println!("Hello, {full_name}");
}

fn make_money(value: &mut String) {
    value.push_str("$$$");
    println!("Hello, {}", value);
}

fn trim_and_capitalize(value: &str) -> String {
    value.trim().to_uppercase()
}

fn elements(value: &str) {
    let x: Vec<&str> = value.split("!").collect();
    println!("{:#?}", x);
}

fn get_identity() -> String {
    let mut first_name = String::new();
    let mut last_name = String::new();
    println!("write your first name : ");
    io::stdin()
        .read_line(&mut first_name)
        .expect("failed to collect your first name");

    println!("write your last name : ");
    io::stdin()
        .read_line(&mut last_name)
        .expect("failed to collect your last name");

    //format method return a heap allocated String
    format!("{} {}", first_name.trim(), last_name.trim())
}

//DEREF COERTION - RUST can transform a &str type into a &str type, NOT in reverse

/*
-the trim method removes the whitespace from the beginning and end of the string. it returns a string slice
-the to_uppercase abd to_lowercase method return Strings with uppercase abnd lowercase characters
-the replace method swaps all occurences of one character with another.
-the split method cuts a string at all occurrences of a delimeter. call the collect method to gather the results in a vector.
*/
