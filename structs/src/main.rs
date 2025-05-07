/*
A struct(structure) is a container for relted pieces of data.
//we use structs to represent complext data types that can store multiple pieces of information.
RUST has 3 TYPES of structs
          -NAME FILED STRUCTS
          -TUPLE-LIKE STRUCTS
          -UNIT-LIKE STRUCTS
*/

/*
//STRUCTURE NAMES SHOULD START WITH A CAPITAL LETTER - PASCAL CASE
//an INSTANCE - is the concrete value made from the struct
// ACCESS STRUCT FIELDS - we use "."
fn main() {
    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    //the "mocha" variable is the owner of the structure.
    // /the structure inturn is the owner of the fields
    //  and the fields are the owner of the corresponding value
    /*
    so when "mocha" variable goes out of scope at the end of the main function, "mocha" the owner responsible
        for cleaning up the struct, and that creates almost lke a cascade of cleanups.
     the struct inturn cleans up its fields and the fields inturn cleanup their associated values.
     */
    let mut mocha: Coffee = Coffee {
        name: String::from("Mocha"),
        price: 4.99,
        is_hot: true,
    };

    println!(
        "my {} this morning cost {}. it is {} that it was hot.",
        mocha.name, mocha.price, mocha.is_hot
    );

    //as there will be partial transfer of ownership. we can't use this synthax in array of heap Strings as we saw ealier.
    //but in structs the partial ownership is not a problem. but we canNOT use the field to access the value in the
    //      instance again as it transfers the ownership for tha value to the new assigned variable.
    //Strings in the heap doesnot implement the copy trait
    // let s = [String::from("yes"), String::from("no")];
    // let b = s[0];

    /*
    //ownership transferred.
    let favorite_Coffee = mocha.name;
    println!("{}", mocha.name); // so we canot use the field.
    //so to avoid such problem / unable to use the field. instead of moving ownership, use reference or clone method.
    */

    mocha.price = 5.99;
}
 */

// struct Coffee {
//     price: f64,
//     name: String,
//     is_hot: bool,
// }

/*
fn main() {
    // The coffee variable takes ownership of the returned Coffee instance.

    /*
    Ownership transfer explanation:
    1. The `name` variable owns a String.
    2. When `name` is passed to `make_coffee`, ownership moves to the function parameter.
    3. Inside `make_coffee`, the function parameters transfer ownership to the struct fields.
    4. The function returns the Coffee instance, transferring ownership to `coffee`.
    */
    let name = String::from("Latte");
    let coffee = make_coffee(name, 4.99, false);
    println!(
        "My {} this morning costs {}. It is {} that it is hot.",
        coffee.name, coffee.price, coffee.is_hot
    );

    //the following is also a SHORTHAND initialization is the variable names matches the field names
    let name: String = String::from("Latte");
    let price = 3.99;
    let is_hot = false;
    let latte: Coffee = Coffee {
        name,
        price,
        is_hot,
    };
}

}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    // Coffee {
    //     name: name, // Ownership moves here
    //     price: price,
    //     is_hot: is_hot,
    // }

    //SHOTHAND INITIALIZATION OF INSTANCES
    //if the parameter names match the field names. we can just write the field name to initialize the instance.
    Coffee {
        name,
        price,
        is_hot,
    }
}
 */

/*

fn main() {
    let mut mocha: Coffee = make_coffee(String::from("Mocha"), 4.99, true);
    // drink_coffee(mocha); //here the ownership of the struct moves from the "mocha" variable to the parameter variable "coffee".
    // println!("{}", mocha.name); //this willl be an error, as the mocha variable is no longer valid
    drink_coffee(&mut mocha);
    println!("{}", mocha.price);

    //this is totally valid and safe to do as well, because float and booleans implement the copy trait
    //bt what if we have a lot of fields, like 10. it is alot of manual work to write all of them one by one.
    //     let caramel_macchiato = Coffee {
    //         name: String::from("Caramel Machiato"),
    //         price: mocha.price,
    //         is_hot: mocha.is_hot,
    //     };
    // }
    //So the better way to do it is a STRUCT UPDATE SYNTAX.
    //that is using a spreadinng operator like that of the JS one.

    //spread the instance to copy and overwrite the one we want.
    // let caramel_machiato = Coffee {
    //     // we don't spread the name, b/c string doesnot implement the copy trait.
    //     //     and there will be partial trasfer of ownership from the first struct,
    //     //        which make the field in the first struct unusable.
    //     // name: mocha.name.clone(), //IF WE WANT THE SAME STRING, WE NEED TO CLONE IT.
    //     // name: String::from("Caramel Machiato"),
    //     ..mocha //this should be the last statementin the struct initializaiton
    // };
}
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}

//STRUCTS AS PARAMETERS FOR FUNCTIONS
// fn drink_coffee(mut coffee: Coffee) {
//     println!("Drinking my delicious {}.", coffee.name);
//     coffee.is_hot = false;
// }

//we can pass the reference to the instance as well.
fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}.", coffee.name);
    coffee.is_hot = false;
    coffee.price = 10.99;
}
 */
//////////////
/// /////////
///
/*
/// DERIVING DEBUG TRAIT FOR STRUCTS
///remember - a TRAIT is a contract that mandates that a type will implement some methods.
// a type that implements either the DISPLAY or DEBUG trait promises that it can be represented as a string.
#[derive(Debug)] //this is how we tell the compiler to have a debug trait for the structs below this attribute

struct Coffee {
    price: f64,
    name: String,
    is_hot: bool,
}

fn main() {
    let mocha = make_coffee(String::from("Mocha"), 4.99, true);
    println!("{:?}", mocha);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name,
        price,
        is_hot,
    }
}
*/

/*
////////////////////////////////
///
//DEFINING METHODS FOR STRUCTS
// A METHOD - is a function that belongs to an instance
#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration: u32,
}

//impl - shothand for"IMPLEMENTATION" - and like OOP, all the methods we define here will automatically be associated with the struct
//so any TaylorSwiftSong struct we will create in our program will have access to the methods we will declare here.
impl TaylorSwiftSong {
    // fn display_song_info(self: Self) {
    //     //self - the instance, Self - the type / struct
    //     //immutable struct value (self parameter takes ownership)
    //     //mutable struct value (self paramter takes ownership, has the permission to mutate).
    //     //immutable reference to the struct instance (no ownershp moved)
    //     //mutable references to the struct instance (no ownership moved, have permission to mutate).
    //     println!("Title: {}", self.title);
    //     println!("Realease Year: {}", self.release_year);
    //     println!("duration: {}", self.duration);

    //     //SO NOW WE SEE THE DESADVANTAGE OF OWNERSHIP - after we call one method which takes the ownership, we can't call another method.
    //     // the only way to keep the struct alive, if we transfer ownership is returning self.
    //     //using references in better
    // self // but it is not the best way to do things.
    // }
    // fn display_song_info(self: &Self) {}
    //           OR
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Years since release: {}", self.years_since_release());
        println!("duration: {}", self.duration);
    }

    // fn double_duration(mut self: Self) {
    //     self.duration *= 2;
    // }

    // fn double_duration(self: &mut Self) {
    fn double_duration(&mut self) {
        self.duration *= 2;
    }

    // METHODS WITH MULTIPLE PARAMETERS

    fn is_longer_than(self: &Self, other: &Self) -> bool {
        //2 parameters - both of them are instances of "taylorswiftsong" structs
        self.duration > other.duration
    }

    ////////////////////
    //ASSOCIATED FUNCTIONS - are functions associated with a type
    // for eg String::from("sth"); // form function- isnot a method. we are not calling it on actual String instance. rather it live directly on the String type.
    //so we often use associated functions for CONSTRUCTORS.
    // a CONSTRUCTOR describes a function that return new instance of the give type
    //it is called CONSTRUCTOR b/c the funcction constructs or builds a new value of that type.
    //String::new(); new function here is an example of a constructor function.

    //the following is an associated function.
    // but we define it in the same place as the methods. how the hell RUST knows it is an associated function.
    //ANSWER - is there is no self parameter, RUST will understand the function is an associated function.

    //so the function will exist not on the instances, on the TaylorSwiftSong type.
    fn new(title: String, release_year: u32, duration: u32) -> Self {
        //we can use the REAL STRUCT name as well
        Self {
            title,
            release_year,
            duration,
        }
    }
}

//MULTIPLE "IMPL" BLOCKS - behind the scenes rust will combine them both.
//so, it seems redundant, but it is actually useful for RUST concepts, we will see in the future
impl TaylorSwiftSong {
    //CALLING METHODS IN OTHER METHODS
    //it is better for methods to have a specific functionality. so if there is unrelated stuff, we extract it to other method and call inside other methods which need its functionality.
    fn years_since_release(&self) -> u32 {
        2025 - self.release_year
    }
}

fn main() {
    let mut blank_space = TaylorSwiftSong {
        title: String::from("Blank space"),
        release_year: 2014,
        duration: 231,
    };

    //"song" parameter will transfer ownership to the "self" parameter
    blank_space.display_song_info();
    // println!("{}", song.title) //so this will be invalid as there is a move of ownership to the self variable
    // song.double_duration(); //we can't cal another method as well. b/c "song" transfers the ownership in the first method call
    blank_space.double_duration();
    blank_space.display_song_info();

    let mut all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration: 327,
    };

    all_too_well.double_duration();

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        );
    } else {
        println!(
            "{} is shorter than {}",
            blank_space.title, all_too_well.title
        );
    }

    //NOW LET'S SEE HOW TO USE THE ASSCOIATED FUNCTION CONSTRUCTOR
    let shake_it_out = TaylorSwiftSong::new(String::from("Shake It Out"), 2010, 400);
    println!("{:#?}", shake_it_out);
}
 */

/*
//BUILDER PATTERN - A design pattern is recommended way to write  or structure code to solve specific problems
//                -this is how RUST devs usually do.
#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_drive_capacity: u32,
}

impl Computer {
    fn new(cpu: String, memory: u32, hard_drive_capacity: u32) -> Self {
        Self {
            cpu,
            memory,
            hard_drive_capacity,
        }
    }

    // fn upgrade_cpu(&mut self, new_cpu: String) {
    //     self.cpu = new_cpu;
    // }

    // fn upgrade_memory(&mut self, new_memory: u32) {
    //     self.memory = new_memory;
    // }

    // fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) {
    //     self.hard_drive_capacity = new_capacity;
    // }

    fn upgrade_cpu(&mut self, new_cpu: String) -> &mut Self {
        self.cpu = new_cpu;
        self
    }

    fn upgrade_memory(&mut self, new_memory: u32) -> &mut Self {
        self.memory = new_memory;
        self
    }

    fn upgrade_hard_drive_capacity(&mut self, new_capacity: u32) -> &mut Self {
        self.hard_drive_capacity = new_capacity;
        self
    }
}
fn main() {
    let mut computer = Computer::new(String::from("m3 max"), 64, 2);
    // computer.upgrade_cpu(String::from("m4 max"));
    // computer.upgrade_memory(128);

    //BUT THE BUILDER PATTERN / DEV'S DO / - return references from the methods
    //then e can do a chain call with that. previously, we can call only on the original variable
    computer
        .upgrade_cpu(String::from("M4 Max"))
        .upgrade_memory(128)
        .upgrade_hard_drive_capacity(4);

    println!("Stats : {computer:#?}");
}
 */

/*
///////////////////////////////
///
/// NOW LET'S SEE THE SECOND TYPE OF STRUCTS IN RUST - TUPLE LIKE
//TUPLE STRUCTS - is a struct that assigns each piece of data an ORDER IN LINE rather than a name
//so we use INDEXES here to identify the values.
//                  HOURS, MINUTES
struct LongDuration(u32, u32);
struct ShortDuration(u32, u32);

fn main() {
    let work_shift = ShortDuration(8, 15);
    println!("{} hours and {} minutes", work_shift.0, work_shift.1);

    let era = LongDuration(5, 3);
    println!("{} years and {} months", era.0, era.1);

    // Why use a tuple struct instead of a tuple?
    // ------------------------------------------
    // If we use a plain tuple (u32, u32), it allows unintended mixing of different concepts.
    // For example, the function `go_to_workk` accepts any (u32, u32) tuple.
    // This means we could accidentally pass `era`, which represents years and months,
    // instead of `work_shift`, which represents hours and minutes.

    // However, using a tuple struct (e.g., ShortDuration) enforces type safety.
    // The function `go_to_work` only accepts ShortDuration, preventing concept mismatches.
}

// The type must always be ShortDuration, ensuring correctness.
fn go_to_work(length: ShortDuration) {
    println!("Passing time: {} hours {} minutes.", length.0, length.1);
}

// This accepts any (u32, u32) tuple, even if it represents something unrelated.
fn go_to_workk(length: (u32, u32)) {
    println!("Passing time: {} hours {} minutes.", length.0, length.1);
}
 */

/*
//UNIT LIKE STRUCTS a struct that holds no fields/ with out any data.
//a UNIT - is an empty tuple, a tuple without values.

struct Empty;

fn main() {
    let my_empty_struct = Empty;
}
 */
///////////////
///
///
//CODING CHALLENGE
#[derive(Debug)]

struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: u32,
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: u32) -> Self {
        Flight {
            origin,
            destination,
            price,
            passengers,
        }
    }

    fn change_destination(&mut self, destination: String) -> &mut Self {
        self.destination = destination;
        self
    }

    fn increase_price(&mut self) -> &mut Self {
        self.price = 1.2 * self.price;
        self
    }

    fn itinerary(&self) {
        println!("{} -> {}", self.origin, self.destination);
    }
}
fn main() {
    let mut flight = Flight::new(String::from("Ethiopia"), String::from("USA"), 2000.0, 35);
    flight.change_destination(String::from("Eritrea"));
    flight.increase_price();
    flight.itinerary();

    println!("todays flight :  {flight:#?}");

    let new_flight = Flight {
        origin: String::from("paris"),
        destination: String::from("England"),
        ..flight
    };

    println!("another todays flight :  {new_flight:#?}");
}
