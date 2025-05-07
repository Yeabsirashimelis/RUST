//THE OPTION ENUM - models a scenario where a type could be a valid value or nothing at all.
//FOR EG:- let's say that we are measuring temperatures on a daly basis and we are recording those temperature values as integers.
//what happens if we forget to take temperature on a given day.
// how do we model the idea of a mmissing value of an absent value?
//so other programing languages called it a null value, which is a type that represents nothingness, the absence of value.
//for eg : in other familiar languages, JavaScript has null and undefined.
// this null value, has long been known to cause errors in programs, because sometimes the code thinks
// it has a valid value when it has a null value or nothing at all.
// So the Rust solution for this problem is the Option enum.

/*
//the OPTION enums has 2 variants
// - Option::None - represents an absent value
// - Option::Some(T) - represents the present value - it is a tuple variant by the way in which the associated data type can be any as it is a generics
// having something of some type or nothing at all.
fn main() {
    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);

    //the following 2 are the same thing
    let d: Option<i8> = Option::Some(5);
    let a = Option::<i8>::Some(5); //this is a turbofish operator

    //as we talk in the previous section this will not compile. even there is no assocaited data it can be mutated
    //  to the Option::Some(T) variant as they are the same tuple
    //so we need to give some type
    // let e = Option::None;

    let e: Option<i32> = Option::None;
}
 */

/*
//REAL EXAMPLE OF OPTIONS ENUM / THE GET METHOD ON AN ARRAY
fn main() {
    let musical_instruments = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Bass"),
    ];

    /*

        // Accessing array elements using direct indexes can lead to runtime errors
        // if the index is out of bounds. For example, trying to access an element at index 5
        // when the array only has 3 elements will panic the program.

        // The `get` method returns an `Option` type (`Some(value)` if the index exists,
        // or `None` if it doesn't). This way, the program can handle missing elements safely
        // without crashing.

        let bass: Option<&String> = musical_instruments.get(2); //it will give us a reference  not to take a partial ownership
        println!("{:?}", bass); //it will print Some("Bass")

        // This won't cause a runtime error, even if the index is out of bounds.
        // Instead, `.get()` returns an `Option` enum.
        // Since index 100 doesn't exist, it returns `None` — a safe way to handle invalid indexes.
        let invalid_instrument = musical_instruments.get(100);
        println!("{:?}", invalid_instrument);

        //the get method returns an enum with assoicated value.
        //and now how to get the value then. that is what matters the most
        //the UNWRAP method attempts to extrat the associated data out of the Some variant.
        // but if the variant is NONE we'll run into runtime error, b/c there is nothing to extract
        let valid_instrument = bass.unwrap();
        println!("{valid_instrument}");

        // invalid_instrument.unwrap(); //this will create a runtime error as the invalid_instrumment is an option enum of the NONE variant

        // EXPECT - IS BASICALLY IDENTICAL WITH UNWRAP, BUT IT ALLOWS US TO CUSTOMIZE THE MESSAGE
        // -so once again if you have the "Some" variant, the expect method will extract it.
        //if yoy have the "None" variant, the program will fail at run time but it's going to display the error message the you provide to "expect".
        let valid_instrument = bass.expect("unable to retrive element");
        println!("{valid_instrument}");

        invalid_instrument.expect("unable to retrieve musical instrument"); //THIS WILL BE THE CUSTOM ERROR MESSAGE IN THE TERMINAL
    */
    ////////////////////////////////
    ////////////////////////////////
    ////////////////////////////////
    //SO USING BARE OF THOSE METHODS IN PROBLEMATIC, AS THEY ARE TOO OPTIMISTIC THAT THEY THINK THERE IS DATA EVERYTIME
    //so we use the "MATCH" key word to get an elegant solution.
    //THE MATCH KEYWORD WITH AN OPTION ENUM
    let bass: Option<&String> = musical_instruments.get(2);
    let invalid_instrument: Option<&String> = musical_instruments.get(100);

    // match bass {
    //     Option::Some(instrument) => println!("playing the {instrument}"),
    //     Option::None => println!("singing with my voice"),
    // }

    // match invalid_instrument {
    //     Option::Some(instrument) => println!("playing the {instrument}"),
    //     Option::None => println!("singing with my voice"),
    // }
    play(bass);

    println!("{:?}", bass); // the bass variable is still valid as there is no transfer of ownership
                            //b/c Option enum implements the copy trait. it is an outlier. rust make it so b/c it is lightwieght.

    play(invalid_instrument);
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("playing the {instrument}."),
        Option::None => println!("singing with my voice."),
    }
}
 */
//////////////
///
/*
//RETURNING AN OPTION ENUM FROM  A FUNCTION
fn main() {
    let availability = is_item_in_stock(true, true);
    let availability = is_item_in_stock(true, false);
    let availability = is_item_in_stock(false, false);
    match availability {
        Option::Some(value) => println!("item is available: {value}."),
        Option::None => println!("your item doesn't exist in our system."),
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Option::Some(true)
    } else if item_is_in_system {
        Option::Some(false)
    } else {
        Option::None
    }
}
 */

/*
///////////////////
///////
/// TOP-LEVEL OPTION VARIANTS
///
/// The Rust Prelude is a collection of commonly used types, functions, and macros that are automatically imported into every program.
/// This allows us to use certain constructs without needing to explicitly import them.
///
/// For example: The prelude includes the `Option` enum, which is why we can write `Option` directly in our code.
/// Because of this, we don't need to prefix `Some` and `None` with `Option::` — they're available at the top level.
fn main() {
    let availability = is_item_in_stock(true, true);
    let availability = is_item_in_stock(true, false);
    let availability = is_item_in_stock(false, false);

    match availability {
        Some(value) => println!("Item is available: {value}."),
        None => println!("Your item doesn't exist in our system."),
    }
}

fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if item_is_in_system && item_is_in_stock {
        Some(true)
    } else if item_is_in_system {
        Some(false)
    } else {
        None
    }
}
 */
/*
//////
///
///
////UNWRAP_OR METHOD - it is another method found on the OPTION enum
///                  - it accept one argument which will be a backup if the variant is NONE.
fn main() {
    let present_value = Some(13);
    let missing_value: Option<i32> = None;

    println!("{}", present_value.unwrap_or(0));
    println!("{}", missing_value.unwrap_or(0)); // this will give us 0.
}
*/

/*
//BUILDING AN OPTION ENUM FROM SCRATCH
//first we will derive not only the Debug trait. but the Copy and the clone trait like that of the built in Option enum
#[derive(Debug, Copy, Clone)]
enum MyOption {
    Some(i32),
    None,
}

impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("uh oh😡😡😡"),
        }
    }

    fn unwrap_or(self, fallback_value: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => fallback_value,
        }
    }
}

fn main() {
    let some_option = MyOption::Some(100);
    println!("{}", some_option.unwrap());

    println!("{some_option:?}"); // the variable is still valid, b/c there is no transfer of ownership. we derived the copy trait first

    let none_option = MyOption::None;
    // println!("{}", none_option.unwrap()); // so there is nothing to unwrap, this will panic the program

    println!("{}", some_option.unwrap_or(0));
    println!("{}", none_option.unwrap_or(0)); //there will no panic, bc we have a backup value
}
 */

/*
//THE RESULT ENUM - the result enum modea=ls the outcome of an evaluation that can produce either SUCCESS or an ERROR.
//so the OPTION enum is for representing presence vs absence, while Result is for modeling success versus error.
//has 2 variants
//the OK variant indicates a success. it stores an associated piece of data of generic type T.
//the ERR variant indicates an error. it stores an associated piece of data of generic type E.
//the idea is called EXCEPTION in other languages
fn main() {
    let ok: Result<i32, &str> = Result::Ok(5);
    println!("{:?}", ok);
    let disaster: Result<i32, &str> = Result::Err("something went wrong");
    println!("{:?}", disaster);
}
*/
///////////////////
///
/*
//REAL EXAMPLE OF RESULT ENUM (the PHRASE method on a string)
fn main() {
    let text = "50";
    //parse - is a method used to extract a number from a string.
    //so it could end up parsing successfully if the string has numberic content
    // or we will fail if there is no numeric data to pull out
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);

    let text = "yup";
    let text_as_number = text.parse::<i32>();
    println!("{:?}", text_as_number);
}
 */

/*
fn main() {
    let result = divide(10.0, 2.0);
    // let result = divide(10.0, 0.0);

    match result {
        Ok(calculation) => println!("Result : {}", calculation),
        Err(error_message) => println!("Error: {}", error_message),
    }
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err(String::from("can not divide by zero"));
    } else {
        return Ok(numerator / denominator);
    }
}
 */

/*
//uisng match keyword for RESULT enum is common for extraction of values, but result enum also has method that helps with extraction
//RESULT METHODS
fn main() {
    let result = divide(10.0, 0.0);

    //used to extract data from the OK variant
    // it will create a panic for the ERR variant
    // println!("{}", result.unwrap());

    //we can pass a custom error message, but there is still a panic
    // println!("{}", result.expect("can't divide by 0."));

    //with this we can give a fallback value
    // println!("{}", result.unwrap_or(0.0)); //if divided by zero (the ERR variant), the result will be 0.0.

    //ISOKAY method - will give us true for the OK variant
    println!("{}", result.is_ok());

    //ISERR method - will give us true for the OK variant
    println!("{}", result.is_err());
}

fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err(String::from("can not divide by zero"));
    } else {
        return Ok(numerator / denominator);
    }
}
 */
//////////////////
///
/*
//NUANCES OF UNWRAP METHOD ON RESULT
fn main() {
    let my_result = operation(true);

    // let content = match my_result {
    //     Ok(message) => message,
    //     Err(err_message) => err_message,
    // };

    // This causes an error because `my_result`'s ownership is moved during the match.
    // `Result<String, String>` holds `String` (a non-Copy type), so ownership transfers
    // when pattern matching extracts the value.
    // println!("{}", my_result.unwrap()); // ❌ Error: value borrowed after move

    //so the solution is using reference for the "result ENUM".
    // let content = match &my_result {
    //     Ok(message) => message,
    //     Err(err_message) => err_message,
    // };

    // println!("{}", my_result.unwrap()); //this will be valid, b/c there is no transfer of ownership

    /////////////////////////////////////////////////////////////////////////////////////////////////////////
    // let content = match my_result {
    //     Ok(message) => message,
    //     Err(err_message) => err_message.to_string(),
    // };

    // This still errors out — even though the Err variant holds a &str (which is Copy),
    // the Ok variant holds a String (which isn't Copy). Rust treats the whole Result
    // as non-Copy, since not all variants have copyable data.
    // So there's a "50% chance" feel: one path moves ownership, the other doesn't.

    // println!("{}", my_result.unwrap());
    /////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let content = match my_result {
        Ok(message) => message,
        Err(err_message) => err_message,
    };
    println!("{content}");
    //there is no error now. b/c both the associated datas implement the copy trait
    // so there is no transfer of ownership
    println!("{}", my_result.unwrap());
}

// fn operation(great_success: bool) -> Result<String, String> {
//     if great_success {
//         Ok("Success".to_string())
//     } else {
//         Err("Error".to_string())
//     }
// }

//let's make one of the type to implement the copy trait
// fn operation(great_success: bool) -> Result<String, &'static str> {
//     if great_success {
//         Ok("Success".to_string())
//     } else {
//         Err("Error")
//     }
// }

fn operation(great_success: bool) -> Result<&'static str, &'static str> {
    if great_success {
        Ok("Success")
    } else {
        Err("Error")
    }
}
 */

/*
//////////////////////////////////
////////////
//THE WHILE LET CONSTRUCT - which is used to compare a dynamic value with a hard-coded enum variant
//                         and create a variable for the associated data incase there is a match

fn main() {
    let mut sauces = vec!["Mayonaise", "Ketchup", "Ranch"];

    // if let Some(sauce) = sauces.pop() {
    //     println!("the next sauce is {sauce}");
    // }

    // if let Some(sauce) = sauces.pop() {
    //     println!("the next sauce is {sauce}");
    // }

    // if let Some(sauce) = sauces.pop() {
    //     println!("the next sauce is {sauce}");
    // }

    // //at the fourth pop call the vector will be empty, so the pop returns a non variant.
    // //  so this final if statement will not be executed
    // if let Some(sauce) = sauces.pop() {
    //     println!("the next sauce is {sauce}");
    // }

    //but instead of doing this, let's use the "WHILE LET" construct
    while let Some(sauce) = sauces.pop() {
        println!("the next sauce is {sauce}");
    }
}
 */
///////////////////////////////////////
////////////////////////////
//CODING CHALLENGE
#[derive(Debug)]
struct Food {
    name: String,
}

#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        // match self {
        //     // Destructuring the fields and checking conditions
        //     Restaurant {
        //         has_mice_infestation: true,
        //         ..
        //     } => Option::None,
        //     Restaurant { reservations, .. } if *reservations < 12 => Option::Some(Food {
        //         name: String::from("Uni Sashimi"),
        //     }),
        //     _ => Option::Some(Food {
        //         name: String::from("Strip Steak"),
        //     }),
        // };

        //we can use an if/ else statement as well
        if self.has_mice_infestation {
            return Option::None;
        }

        if self.reservations < 12 {
            return Option::Some(Food {
                name: String::from("Uni Sashimi"),
            });
        } else {
            return Option::Some(Food {
                name: String::from("Strip Steak"),
            });
        }
    }
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Result::Err("Sorry, we have a mice problem".to_string());
        }

        if address.is_empty() {
            return Result::Err(String::from("No delivery address specified"));
        } else {
            return Result::Ok(Food {
                name: String::from("Burger"),
            });
        }
    }
}

fn main() {
    let restaurant = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };

    println!("{:?}", restaurant.chef_special());

    println!("{:?}", restaurant.deliver_burger("123 Elm Street"));

    let restaurant = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };

    println!("//////////////////////////////////////////////////////////////////////////");

    println!("{:?}", restaurant.chef_special());

    println!("{:?}", restaurant.deliver_burger("123 Elm Street"));
    println!("{:?}", restaurant.deliver_burger(""));
}
