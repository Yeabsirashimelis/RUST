// GENERICS - is a type argument. it is a placeholder for a future type.
//a generic is a very similar concept with variables.
///    but variable is a placeholder for a future value. but generics are for TYPES.

// THE TURBOFISH OPERATOR - it looks like this  "::<I32>"
// it is for if we want to explicitly tell RUST that the type of the value we are passing.
//it is really useful to use types which are not default for RUST. like "i8". b/c if we don't explicitly write the type, always RUST will get it as i32.

/*
#[derive(Debug)]
struct DeliSandwich {}

fn main() {
    println!("{}", identity::<i32>(5));
    println!("{}", identity::<i8>(5));
    println!("{}", identity(13.4));
    println!("{}", identity(false));
    println!("{}", identity("hello"));
    println!("{}", identity::<&str>("hello")); //similar as the above one
    println!("{}", identity(String::from("hello")));
    println!("{{:?}}", identity(DeliSandwich {}));
}

// fn add_5(value: i32) -> i32 {
//     //for eg: here value is a variable and can represent any future i32 value/
//     value + 5
// }

//the weakness of this function is that it can only accepts i32 types.
// what if we want the same function for other types. for eg: boolean
//   with this method, we should write another function
//most of the time, this is no a problem at all, b/c function need a specific type to operate with
//but like these scenarios(function with the same structure for different types), giving a hardcoded type can be disadvantegeous.
//SO MAKE THE TYPE MORE GENERAL/ GENERICS✅
// fn identity(value: i32) -> i32 {
//     value
// }

// fn identity_bool(value: bool) -> bool {
//     value
// }

//these means for rust compiler : IMAGINE THERE IS SOME TYPE CALLED "T" THAT YOU'LL KNOW ABOUT WHEN THE FUNCTION IS INVOKED.
//                                  THE FUNCTION IS NOT BEING INVOKED YET, SO YOU DON'T KNOW WHAT TYPE "T" IS.
fn identity<T>(value: T) -> T {
    value
}

//but at compile time. there is no a concept generics. RUST will create as many functions as our generic function is called.
//the process is called MONOMORPHIZATION - a compile time process where polymorphic functions are replaced by many monomorphic function for ach unique instantiation.
 */
//////////////////////////
///
///

/*

//MULTIPLE GENERICS
fn main() {
    // This works: first argument can be any type, second must be i32
    make_tuple("Hello", 5);

    // This will cause an error! Both arguments must be the same type
    // because make_tuple2 uses a **single generic** type `T` for both parameters.
    // make_tuple2("Hello", 5);

    // This works because both arguments are strings (same type)
    make_tuple2("hello", "world");

    // This works too: make_tuple3 has **multiple generics** (T and U),
    // so the arguments can be of different types.
    make_tuple3("hello", 5);
}

// Function with a single generic type `T` and a fixed i32 for the second argument
fn make_tuple<T>(first: T, second: i32) -> (T, i32) {
    (first, second)
}

// Function with a single generic type `T` for **both** arguments — they must be the same type
fn make_tuple2<T>(first: T, second: T) -> (T, T) {
    (first, second)
}

// Function with **multiple generics** (T and U), allowing different types for each argument
fn make_tuple3<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}
 */
//GENERICS IN STRUCTS - we can ad a generic type to something like struct

// TreasureChest - is something like a box which contains a variety of precious things.

/*
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

//GENERICS AND IMPL BLOCKS
//this means the impl block will define methods only for instances where the treasure field is "String".
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        //now treasure is a String
        self.treasure = self.treasure.trim().to_string();
    }
}

//and this one will only be there for instances where "treasure" field is a length 3 array.
impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize {
        //when we call length method on the array we will get usize

        //now treasure is an array
        self.treasure.len()
    }
}

//INORDER TO MAKE THE IMPL WORK FOR EVERYTYPE OF THE INSTANCE, WE WILL USE A LITTLE BIT DIFFERENT SYNTAX
//if we don't write only the second "T", rust will think it is a concrete type. the first "T" declares a generic type first for rust to use it to represent any type
impl<T> TreasureChest<T> {
    //this method work on any instance
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chest: TreasureChest<&str> = TreasureChest {
        captain: String::from("Fireboard"),
        treasure: "Gold",
    };

    // this will be an error!!, the method is only available for only instances where the "T" variable is a "String". not "&str"
    // gold_chest.clean_treasure();

    println!("{}", gold_chest.capital_captain());

    println!("{:?}", gold_chest);

    let mut silver_chest: TreasureChest<String> = TreasureChest {
        captain: String::from("Bllodsail"),
        treasure: String::from("    Silver"),
    };

    //the type of "T" is string. so the clean_treasure method is available
    silver_chest.clean_treasure();
    println!("{}", silver_chest.capital_captain());

    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Bootyplunder"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{}", special_chest.capital_captain());

    //the type of "T" is array containing 3 elements. so the amount_of_treasure method is available
    println!("{:?}", special_chest.amount_of_treasure());

    println!("{:?}", special_chest);
}
 */
///////////////////
///
/*
// GENERICS IN ENUMS - Generics can be used in enums to allow flexible associated data types.
enum Cheesesteak<T> {
    plain,
    Topping(T),
}

fn main() {
    // Creating variants with different types of toppings.
    let mushroom = Cheesesteak::Topping("mushroom");
    let onions = Cheesesteak::Topping(String::from("onions"));
    let topping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&topping);

    // This will cause an error because `plain` doesn't hold a value,
    // but Rust still requires a concrete type for `T` (e.g., Cheesesteak<()> or Cheesesteak<String>).
    //it should know the type at compile time, (even not used)
    // let plain = Cheesesteak::plain;

    //we should give the type b/c what if the variable is mutable and tried to be changed to the enum type which has the generic type
    //even we start out wuth the "Plain" variant that doen't utilize "T", b/c there is a potential
    //   for this variable change into the Cheesestreak variant that does require "T".
    let mut plain: Cheesesteak<String> = Cheesesteak::plain;
    plain = Cheesesteak::Topping(String::from("plain"));
}
 */
////CODING CHALLENGE
///
///

#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("watching the {:?}", self.content);
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

// .to_string() - will create a heap allocated string like String::from() method.
fn main() {
    let c_m1 = ChatMessage {
        content: "how are you doing?",
        time: "3:10".to_string(),
    };

    let c_m2 = ChatMessage {
        content: String::from("I'm pretty well and U?"),
        time: "3:15".to_string(),
    };

    let c_m3 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: "3:20".to_string(),
    };

    c_m3.consume_entertainment();

    println!("{}", c_m1.retrieve_time());
    println!("{}", c_m2.retrieve_time());
    println!("{}", c_m3.retrieve_time());
}
