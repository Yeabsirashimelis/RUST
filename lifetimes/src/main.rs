//LIFETIMES - refers to how long it is valid (capable of being used) within the code
//          -a value's lifetime is the time during which it exists at a particular memory address
//LIFETIMES ARE OFTEN CONNECTED TO THE IDEA OF SCOPE BUT THEY DON'T NECCESSARILY HAVE TO BE
//     one scenario it is not related to scope is MOVEMENT OF OWNERSHIP. when ownership is transferred, the lifetime of the varaible ended.

/*
//CONCRETE LIFETIME - a lifetime which is clearly defined
fn main() {
    //
    //
    //
    //a's lifetime starts here. and its lifetime ends when the main function ends
    // RUST can clearly see where "a" variable comes into existence (lifetime begins) and lifetime ends
    let a = 1;

    {
        let b = 2;
    }
    // println!("{b}"); //this is an error b/c the lifetime ended in the inner block

    let c = String::from("winter");
    let d = c; //the lifetime of variable "c" ends when it moves ownership to variable "d"

    drop(d); //the lifetime of d ends here as drop function will remove it from the heap
}
 */

/////////////////////////////////////
////////////////////////
//we mostly discuss on "lifetime of references", b/c it is the one in which many complexities occurs
//CONCRETE LIFETIMES FOR REFERENCES - the lifetime of references is contained in the lifetime of the referenced value. so all the rules applied for references in which we talked on normal variables.
//                                              and the additonal contraint it that the lifetime of the refernece is dependent on the real value (referent) as well
//                                  so by this we are sure that a reference will not exist without the referenced thing
/*
fn main() {
    let dog = String::from("Watson"); // referent - the value being borrowed

    {
        let my_pet: &String = &dog; //reference - the actual borrow
        println!("{my_pet}");
    }

    println!("{dog}");

    {
        let my_pet: &String = &dog; //reference - the actual borrow
        println!("{my_pet}");
    }
}
 */

/*
 /////////////////////////////////////////////
///////////////////////
//NON-LEXICAL LIFETIMES - means NOT lasting until the end of the block
//   the borrow checker treats the end of a reference's lifetime as the last place it is used; a reference has non-lexical scope
fn main() {
    let dog = String::from("Watson");
    let my_pet = &dog;
    println!("{}", my_pet); //the end of lifetime of my_et reference is here. where it is being used last, even if there are 100 lines of code after it in the scope

    //100 lines of code
}

// //NON-LEXICAL LIFETIMES is added to RUST recently. for eg: let's see one example that was
// //  invalid before this concept
// fn bar() {
//     //this was not valid before b/c the end of the lifetime of slice(mutable reference is at the end of the block).
//     //    and the push method is also use mutable reference of the vector.
//     // and 2 mutable references are not allowed at the sametime.

//     //BUT NOW, if we don't use the slice mutable reference lifetime ends non-lexically before the other mutable reference begins. so there is not coexistence of references.
//     //   so it is okay to go now.
//     let mut data = vec!['a', 'b', 'c'];
//     let slice = &mut data[..];
//     capitalize(slice);
//     data.push('d');
//     data.push('e');
//     data.push('f');
// }
*/

/*
//////////////////////////////
/////////////////////////
//INVALID LIFETIMES - THE PROGRAM WILL NOT COMPILE IS THE BORROW CHECKER SEES AN INVALID LIEFETIME WHICH IS CAUSED BY A DANGLING REFERENCE
//   DANGLING REFERENCE  - is a reference to  data that no longer exists / the reference outlived the value.
fn main() {
    /*let cities = vec![
        String::from("Addis Ababa"),
        String::from("Debre-zeit"),
        String::from("Naziret"),
    ];

    let favorite_cities = &cities[0..2];
    drop(cities); //this code will not compile
    println!("{:?}", favorite_cities)
    */

    /*
    // this is also not correct. b/c the life time of somecities end at the end of the block
    //    but we are returning the a reference to the outer scope which is a dangling reference
    let some_cities = {
        vec![
            String::from("Addis Ababa"),
            String::from("Debre-zeit"),
            String::from("Naziret"),
        ];

        &cities[0..2]
    };

     */

    ////////////////////////////////////////////
    //the following is also one way of happening for dangling reference. but rust is great, it never compile
    let cities = vec![
        String::from("Addis Ababa"),
        String::from("Debre-zeit"),
        String::from("Naziret"),
    ];

    /*
        // This creates a dangling reference because:
        // 1. `places` takes ownership of `cities` (moves it)
        // 2. Then we try to create a slice reference to the now-moved `cities`
        // The reference points to invalid memory since `cities` was moved to `places`
        let places = cities;
        let favorite_cities = &cities[0..2];  // ERROR: borrow of moved value `cities`
        println!("{:?}", favorite_cities);
    */

    /*
        // This creates a dangling reference because:
        // 1. We create a reference to part of `cities` first
        // 2. Then we move `cities` to `places`, invalidating the reference
        // The reference `favorite_cities` becomes dangling since what it points to was moved
        let favorite_cities = &cities[0..2];
        let places = cities;  // ERROR: move occurs while `cities` is borrowed
        println!("{:?}", favorite_cities);
    */
}
 */

/*
/////////////////////////////
////////////////////////////
//FUNCTIONS CANNOT RETURN REFERENCES TO OWN VALUES OR PARAMS
//  b/c either one of them creates a dangling reference. ( b/c the variable will go out of scope when the function block ends)

/*
//so this is a compile time error
fn create() -> &i32 {
    let age = 100;
    &age //returning reference to the "age" variable will be a dangling reference, b/c the "age" variable will go out of scope when the function block ends
}

//this is also a compile time error
fn create_slice(items: Vec<i32>) -> &[i32] {
    &items or &items[0..2]
}

     */
//so how to return a reference in functions  the following is the ony way, which is using reference as a param
// fn select_first_two_elements(items: &Vec<String>) { - only works for vectors
fn select_first_two_elements(items: &[String]) -> &[String] {
    //means collection of strings / works for any collections. arrays, vectors,...

    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("Addis Ababa"),
        String::from("Debre-zeit"),
        String::from("Naziret"),
    ];

    //the original data "cities" must live enough to contain the lifetime of "two_cities" - which take the returned reference
    let two_cities = select_first_two_elements(&cities);
    // drop(cities); // if we drop "cities" here the bottom usage of the refernce will outlive the original data. so a danglinf reference.
    println!("{:?}", two_cities);
    {
        let coffees = [String::from("latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);

        println!("{:?}", two_coffees);
    }
}
 */
////////////////////////////////////
///////////////////////////
//INTO INTO GENERIC LIFETIMES
/*
CONCRETE lifetimes - is a region of code that a value exists in the program (the time it lives in its memory address)
GENERIC lifetimes - is more abstract. it is a hypothetical lifetime, a non-specific lifetime , a future lifetime that can vary
                 we can annotate generic lifetimes in our code. this enables functions that are flexible enought to handle varying lifetimes.
  LIFETIME ANNOTATIONS
        -a lifetime annotation is a name or label for a lifetime
        -lifetime annotations don't change the reference's lifetime. they don't affect the logic in any way.
        -a liftime annotation is a piece of metadata that we provide to the borrow checker so that it can hanle that references are valid


*/

/*

//for first rule of lifetime elision
// fn my_awesome_function(value: &i32, second: &str) {}

//for second rule of lifetime elision - so it can compile without the manual annotation
// fn my_awesome_function(value: &i32) -> &i32 {
//     value
// }

//but if we have 2 reference params, we need lifetime annotation. b/c the rust analyser cannot know to which to couple the lifetime of the returned one to.
//     SO THE FOLLOWING IS AN ERROR
// fn my_awesome_function(value: &i32, second: &String) -> &i32 {
//     value
// }

// ' - called "tick" and used to declare generic lifetimes
/*"Hey Rust, I'm giving you a list, and I'm going to return a part of that same list. Make sure both live as long as each other." */
// fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
//     &items[..2]
// }

fn main() {
    let cities = vec![
        String::from("Addis Ababa"),
        String::from("Debre-zeit"),
        String::from("Naziret"),
    ];

    // so we are sure that this reference "two_cities" lives in the lifetime of its referent "cities" variable
    // let two_cities = select_first_two_elements(&cities);

    //here as you can see the reference passed to the functional has this block as a scope. but "two_cities" has the scope of the "cities" - the referent
    // so two_cities is valid in the scope of cities. not only in the scope of "cities_reference" b/c the lifetime of the returned thing from the function is coupled with the original data or the referent
    let two_cities = {
        let cities_reference = &cities;
        select_first_two_elements(cities_reference)
    };

    println!("{:?}", two_cities);
    {
        let coffees = [String::from("latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);

        println!("{:?}", two_coffees);
    }
}

//LIFETIME ELISION
//elision - is the act of omitting something
//so LIFETIME ELISION MEANS OMITTING GENERIC LIFETIME ANNOTATIONS IN SITUATIONS WHERE THE BORROW CHECKER
//   CAN INFER THE LIFTIME RELATIONSHIPS AUTOMATICALLY
/*
FIRST ELISION RULE - THE COMPILER ASSIGNS A LIFETIME TO EACH PARAMETER THAT IS A REFERENCE
SECOND ELISION RULE - IF THERE IS ONE REFERENCE PARAMETER AND THE RETURN VALUE IS A REFERENCE, THE BORROW CHECKER WILL INFER THAT THEIR LIFETIMES ARE RELATED
*/
 */

/*
//SO let's see the MULTIPLE PARAMETERS now
//as we talked earlier, these needs an explicit lifetime annotation
//so we annotate to the "first" parameters
fn choose_favorite<'a>(first: &'a str, second: &str) -> &'a str {
    println!("{second}");
    first
}

fn main() {}
 */

use std::{collections, result, vec};

/*
/////////////////////////////////////////////////
//MULTIPLE PARAMTERS 2

//we are declaring is that for some eneric lifetime called 'a, for some region of code that both "first" and "second" live within, that they share. the return value must be contained within that region of code
//so the returned value will have a lifetime of either the first or the second
// fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
//     if first.len() > second.len() {
//         first
//     } else {
//         second
//     }
// }

// "which is forget the lifetime of the second one ('b). don't worry about it.
//   the function will give you only a value with a lifetime of the first('a)"
fn longest<'a, 'b>(first: &'a str, second: &'b str) -> &'a str {
    println!("The second is {}", second);
    first
}

fn main() {
    //this one is a simple one
    // let addis = String::from("Addis Ababa");
    // let debre = String::from("Debre Zeit");
    // let result = longest(&addis, &debre);
    // println!("{result}");

    //this one is also simple but good example to undestand
    // let addis = String::from("Addis Ababa");
    // {
    //     let debre = String::from("Debre Zeit");
    //     //even the result is "Addis Ababa" the lifetime of the result ends in this block. b.c this is the one where both of the lifetime of the inputs exists
    //     let result = longest(&addis, &debre);
    //     println!("{result}");
    // }


    // //the following is an error example. b/c their common lifetime is inside the inner block. but the returned thing lifetime is longer than the "debre" one
    // //   which means a reference which is derived from it (indirectly) surpass the original values lifetime (so dangling reference will happen - the "result" variable can hold a dangling reference is the inner value reference is returned from the function)
    // let addis = String::from("Addis Ababa");
    // let result = {
    //     let debre = String::from("Debre Zeit");
    //     longest(&addis, &debre)
    // };


    //SO TO MAKE THE ABOVE ONE WORKS. THE VALUE RETURNING FUNCTION SHOULD HAVE DIFFERENT GENERIC LIFETIMES,
    //  SO THAT WE CAN ATTACH THE RESULT OF THE RETURNED THING WILL ONLY BE ONE OF THEM
    let addis = String::from("Addis Ababa");
    let result = {
        let debre = String::from("Debre Zeit");
        longest(&addis, &debre)
    };
    //this is fine b/c the compiler is sure that the result will never be "debre"
    println!("{result}");
}
*/
/*
//////////////////////////////////////////////////
//THIRD ELISION RULE - in a method definition, if there are multiple reference parameters but one of them is self,
//                      the borrow checker will assume the lifetime of the instance is connected to the lifetime of the return value
struct DensistAppointment {
    doctor: String,
}

impl DensistAppointment {
    //the borrow checker will consider the lifetime of the "self" for the lifetime of the returned thing
    //means  BEHIND THE SCENES, if liftimes are "'a for self", "'b for check_in_time" and "'c for check_out_time",
    //    the borrow checker will takt "'a" as a lifetime for the returned thing / reference
    // fn book(&self, check_in_time: &str, check_out_time: &str) -> &str {
    //     println!(
    //         "you are booked from {} to {} with doctor {}",
    //         check_in_time, check_out_time, self.doctor
    //     );

    //     &self.doctor
    // }

    //if we don't want the default, we can use the lifetime of the other arguments for the returned thing
    fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        check_in_time
    }
}

fn main() {
    let appt = DensistAppointment {
        doctor: String::from("Tsebi"),
    };
    let result = appt.book("03:00", "04:00");
    drop(appt);
    println!("{}", result); //this is correct for the "second drop function version", b/c we leave the default and the instance is not the one in which is the liftime of the returned things depends on
}
*/

/*
//LIFETIME WITH STRUCTS
#[derive(Debug)]
//the structs name field has a lifetime of "'a" and the lifetime of the struct should not outlive this lifetime
//  so the struct should first deallocated
struct TrainSystem<'a> {
    name: &'a str,
}

fn main() {
    //simple one
    // let name = String::from("NJ Transit");
    // let nj_transit = TrainSystem { name: &name };

    // println!("{:#?}", nj_transit);

    /*
    //THIS WILL BE AN ERROR, B/C THE STRUCT OUTLIVES THE LIFETIME OF THE REFERENT (WHICH IS THE ONE CONNECTED WITH THE STRUCT'S NAME FIELD)
    let nj_transit = {
        let name = String::from("NJ Transit");
        TrainSystem { name: &name }
    }
    println!("{:#?}", nj_transit);
     */
}
 */

/*
//MULTIPLE LIFETIME
#[derive(Debug)]
// struct TravelPlan<'a> {
//      //this means the lifetime of the struct will be the overlapping/ commong lifetime of the referents
//  the smaller/ the safer one will be the lifetime of the returned thing
//     from: &'a str,
//     to: &'a str,
// }

struct TravelPlan<'a, 'b> {
    //the struct fields are accessed based on the lifetime of respective referents
    from: &'a str,
    to: &'b str,
}
fn main() {
    let from = String::from("Portland");
    let plan = {
        let to = String::from("Bangor");
        let travel_plan = TravelPlan {
            from: &from,
            to: &to,
        };

        travel_plan.from
    };

    println!("{plan}")
}
*/

/*
///////////////////////////////////
// 'static LIFETIME - The borrow checker treats 'static as a special lifetime.
// It indicates that the referenced data lives for the entire duration of the program.
// It's commonly used for data with a truly static lifetime, such as string literals, constants.
// While you *can* use 'static in return types, you must ensure the data truly lives that long.
// You cannot return a reference to data created inside a function as 'static unless the data itself is static (e.g., boxed and leaked or stored globally).

//constants also has a static lifetime
const COUNT: i32 = 400;

//we are saying that we are returning an item with static lifetime
fn say_hello() -> &'static str {
    "Hello" //it is a &str - reference to string literal which live in the executable. so they live in the entire program execution
}

fn value() -> &'static i32 {
    &COUNT //this is referencing to a constant which exists in the entire program
}

fn main() {
    let greeting = say_hello();
    println!("{}", greeting);

    let count = value();
    println!("{}", greeting);
}
 */
////////////////////////////////////////////////////
///////////////////////////////////////////////////////
//CODING CHALLENGE

// No explicit lifetime annotation is needed because the reference's lifetime
// is only used within the function and not returned.
// The Rust compiler can infer everything safely in this case.
fn double_the_length<T>(collection: &Vec<T>) -> usize {
    collection.len() * 2
}

//this function doesnot need lifetime annotation. borrow checker can figure out the returned thing lifetime is related with the referent (the actual data referenced)
fn last_two<T>(collection: &[T]) -> &[T] {
    let two_from_the_end = collection.len() - 2;

    &collection[two_from_the_end..]
}

//this function needs lifetime annotation. we will return a reference with a lifetime connected with the "text's referent". so we have to tell this to the borrow checker
fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("{}", announcement);
    &text[..5]
}

//yes it needs lifetime annotation. b/c we want to tell borrow checker that the lifetime of the returned reference is related with either the first or the second
fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if first.contains(target) {
        return first;
    } else {
        return second;
    }
}

fn main() {
    let double_length = double_the_length(&vec![1, 2, 3]);
    println!("{}", double_length);

    let vector = vec![1, 2, 3];
    let last_two = last_two(&vector);

    // let last_two = last_two(&vec![1, 2, 3]);
    println!("{:?}", last_two);

    let first_five = first_five("refrigerator", "Hello");
    println!("{:?}", first_five);

    let where_substring = find_string_that_has_content("programming", "dining", "gram");
    println!("{:?}", where_substring);
}
