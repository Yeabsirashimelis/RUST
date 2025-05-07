//ERROR HANDLING- is the process of dealing with potential errors from operations that can go wrong
/*
we can categorize the errors in 2 categories
 1, RECOVERABLE - are erros that we can define code to handle
    eg: when the user gives us invalid input, we can identify that circumstance and asking for a valid input again.
 2, NON-RECOVERABLE - are errors that cause the program to be unable to proceed.
    eg: trying to access a non-existent index inside a vector
    these errors are runtime errors that occurs after the program is successfully compiled. they PANIC the program
    -the compiler WILL NOT catch them, so we have to be carefull

    //after a panic occurs in the terminal we will see what is called the BACKTRACE
    BACKTRACE - is the list of files and functions that were running at the point that the error occured

    // after a panic occurs RUST cleans any stack entries and datas in the heap
       and this process is called UNWINDING.
*/

/*
//THE PANIC! MACRO - to force a panic manually
fn main() {
    // None::<&str>.unwrap(); // this will panic the program, b/c the unwrap method will get nothing to unwrap from the NONE variant
    let a = 1;
    panic!("something went wrong");
}
 */

/*
///////////////////////////////////
///////////////////////////////////
//THE PROCESS MODULE AND EXIT FUNCTION - sometimes you'll want to gracefuuly terminate a RUST program without showing the user a complex error message of a PANIC.
//                                            to accomplish this, we can use the "EXIT" function inside the "PROCESS" submodule in the standard library
//most we use it with conditionals

use std::process;
fn main() {
    //expects a parameter called "code" and its type is i32.
    process::exit(0); //is no error occurs, giving 0 is a convention
    process::exit(1); //is an error occurs, giving 1 is a convention. or you can give any number greater that 0.
}
*/
/////////////////////////////
/////////////////////
/* STANDARD ERROR - (EPRINTLN!() MACRO)
-prints messsages to what's called the standard error
e- stands for error

***
println! - for regular outputs
e.println! - for errors
***
 */
/*
fn main() {
    //visually the do the same thing. both print just texts
    /*but let's use println! for both and try to save the outputs to the file using
     cargo run > example.txt -this will create a file in the current dir and add every println contents to the file and nothing willbe printed on the terminal
      and the error message is also added, which is not INTENDED TO BE.

    but if we differentiate them the println content will be written and the error will be printed on the terminal
     */
    println!("some status update");
    // eprintln!("some error message");
    eprintln!("some error message");
}
 */

/*
////////////////////
/////
//OPENING A FILE
fn main() {
    //opening a file will return us a result enum, which has 2 variants. the OK and ERR
    //if it is Success, it will have the "File struct" as an associated data
    //if it is Err, it will have the "Os struct" as an associated data (which contains the error message in it.)
    /* if SUCCESS : Ok(
        File {
            handle: 0x00000000000000a4,
            path: "\\\\?\\C:\\Users\\MODEL\\Desktop\\RUST\\error-handling\\story.txt",
        },
    )

            IF ERROR: Err(
            Os {
                code: 2,
                kind: NotFound,
                message: "The system cannot find the file specified.",
            },
        )
             */
    // let file: Result<File, std::io::Error> = File::open("story.txt");

    //use match is the most elegant way
    let file = match File::open("stor.txt") {
        Result::Ok(file) => file,
        Result::Err(error) => {
            eprintln!("something went wrong reading the file. the error was {error:?}");
            process::exit(1)
        }
    };

    println!("{:#?}", file);
}
 */
/*
///////////////////////////
//////////////////////////
//ASKING THE USER FOR INPUT and READING FILE CONTENTS
fn main() {
    println!("please enter the name of the file you want to read : ");
    let mut input = String::new();
    // let user_requested_file = match stdin().read_line(&mut input) {
    //     Result::Ok(size) => println!("the size of the user entry in bytes is {size}"),
    //     Result::Err(error) => {
    //         println!(
    //             "something went wrong collecting user input. the error was {:#?}",
    //             error
    //         );
    //         process::exit(1)
    //     }
    // };

    //let's do the above code using another approach / the if let construct
    let user_requested_file = stdin().read_line(&mut input);
    //meaning : if the enum returned is error assign the returned enum associated data to "error" variable
    // and execute the block of code.
    if let Result::Err(error) = user_requested_file {
        eprintln!(
            "something went wrong collecting the user input. the error was {:?}",
            error
        );
        process::exit(1)
    }

    let mut file = match File::open(input.trim()) //we have to trim it. b/c the "enter" or "\n char" after giving the input will be considered as the part of the input. and it returns as "&str" which makes us to remove "&" for Strings
    {
        Result::Ok(file) => file,
        Result::Err(error) => {
            eprintln!("something went wrong opening the file. the error was {error:?}");
            process::exit(1);
        }
    };

    //as the read_line method, this also expects a mutable reference to a heap allocated string
    //   which is it will write the file's content.
    let mut file_contents = String::new();
    let read_operation = file.read_to_string(&mut file_contents);

    //why we use if let instead of match ?
    // BECAUSE - the RESULT Enum OK variant contains the number of files as the associated data
    // the content is in the file_contents variable. the read_operation is uses for Rust way of handling errors
    if let Err(error) = read_operation {
        eprintln!(
            "something went wrong reading the file as a string. the error was {}",
            error
        );
        process::exit(1);
    }

    println!("{file:?}");
    println!("{}", file_contents);
}
*/

/*
///////////////
////IN THE ABOVE CODE UOR AIN FUNCTION IS DOING MUCH. SO WE HAVE DO DELEGATE DIFFERENT RESPONSIBILITIES TO DIFFERENT FUNCTIONS
// PROPAGATING ERRORS - means to bubble it up or send it up so that it is handled by a higher level piece of code.

use std::fmt;
use std::fs::{self, File};
use std::io::{self, stdin, Read};
use std::{process, result};

//so there are many Error types in different modules
// io::Error;
// fmt::Error;

fn main() {
    let file_result: Result<String, io::Error> = read_file();

    match file_result {
        Result::Ok(contents) => println!("{contents}"),
        Result::Err(error) => eprintln!("there was an error: {error:?}"),
    }
}

// fn read_file() -> Result<String, io::Error> {
//     println!("please enter the name of the file you want to read : ");
//     let mut input = String::new();

//     let user_requested_file = stdin().read_line(&mut input);

//     if let Result::Err(error) = user_requested_file {
//         //by returning to the caller function we are propagating the error to the caller function
//         return Result::Err(error);
//     }

//     let mut file = match File::open(input.trim()) {
//         Result::Ok(file) => file,
//         Result::Err(error) => return Result::Err(error),
//     };

//     let mut file_contents = String::new();
//     let read_operation = file.read_to_string(&mut file_contents);

//     if let Err(error) = read_operation {
//         return Result::Err(error);
//     }

//     return Result::Ok(file_contents);
// }

//////
//the ? operator / the try operator - used for computations which returns the RESULT enums. instead of checking by ourselves for the returning the varaint it abstracted for us by returning
//                                    the Err variant if found and continue executing if the OK variant is returned
// fn read_file() -> Result<String, io::Error> {
//     println!("please enter the name of the file you want to read : ");
//     let mut input = String::new();

//     //if the Err varaint return it. if the OK varaint continue executing
//     stdin().read_line(&mut input)?;

//     //if the ERR variant return it. if the OK variant assign the associated data to the variable and continue
//     let mut file = File::open(input.trim())?;

//     let mut file_contents = String::new();
//     file.read_to_string(&mut file_contents)?;

//     return Result::Ok(file_contents);
// }
///////////////////////////////
//READ_TO_STRING - an associated function
// `read_to_string` -  is an associated function from the `fs` module.
// It opens a file, reads its contents, and returns them as a `String`
// stored on the heap. It returns a `Result`, with `Ok(String)` on success
// and `Err(io::Error)` on failure.
//
// This is different from the `read_to_string` method we've seen on the `File` struct.
// The `fs::read_to_string` function is more convenient for simply reading an entire file.

fn read_file() -> Result<String, io::Error> {
    println!("please enter the name of the file you want to read : ");
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    fs::read_to_string(input.trim())
}
 */

// use std::vec;

/*
//////////////////////////
///////////////
//USING ? USING OPTION ENUM
// likewise the RESULT enum, we can use "?" with OPTION enum and it works similarly
//it will return the None variant if non is returned and if the Some variant returned, the associated data will be pulled out and continue executing
fn main() {
    let mut animals = vec!["Giraffe", "Monkey", "Zebra"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    //as we know the pop method returns an option enum

    //if last_element is not found return the NONE variant early
    let last_element = input.pop()?;
    Option::Some(last_element.len())
}
 */
/////////////////////////////
/////////////
//CODING CHALLENGE

use std::{
    self,
    fs::{self, File},
    io::{self, stdin},
    process,
};

fn main() {
    let write_result = write_to_file();
    match write_result {
        Result::Ok(file_path) => println!(
            "this was the content written to the file path : ``{}``",
            file_path
        ),
        Result::Err(error) => {
            eprintln!("Error occurred :{}", error);
            process::exit(1);
        }
    }
}

/*
fn write_to_file() -> io::Result<String> {
    println!("what file would you like to write to : ");
    let mut input1 = String::new();
    let user_requested_file = stdin().read_line(&mut input1);

    if let Result::Err(error) = user_requested_file {
        return Result::Err(error);
    };

    let file = match File::open(&input1.trim()) {
        Result::Ok(file) => file,
        Result::Err(error) => return Result::Err(error),
    };

    println!("what would you like to write to the file : ");
    let mut content = String::new();
    let user_requested_data = stdin().read_line(&mut content);

    if let Result::Err(error) = user_requested_data {
        return Result::Err(error);
    };

    let result = fs::write(&input1.trim(), &content.trim());

    if let Result::Err(error) = result {
        return Result::Err(error);
    }

    Result::Ok(input1)
}
 */

//  io::Result - is a generic based on the Result enum in which the type for the associated data can by of anytype
//               but has constant type for the error "io::Error". so there is no generic need for the error variant

///// using shorter synthax with "?"
fn write_to_file() -> io::Result<String> {
    let input = io::stdin();

    println!("what file would you like to write to write to : ");
    let mut requested_file = String::new();
    stdin().read_line(&mut requested_file)?;

    println!("what would you like to the file : ");
    let mut content = String::new();
    stdin().read_line(&mut content)?;

    fs::write(requested_file.trim(), content.trim())?;

    Result::Ok(requested_file)
}
