/*
//TRAIT - is a distinguishing quality or characteristic
consider a trait like flight. flight describes the quality of being able to fly
2 different types can have similar trait like that of a bird and a plane has the ability to fly

A TRAIT - is a contract that descibes functionality that a type should have
we use the word implement to describe when a type honors a trait's requirements
 for eg: we can say that a candle, a computer screen and the sun implement the "illumination" trait


a trait defination declares the method(s) that a type implementing that trait must have.
     -method name
     -parameters with types
     -return value type
-till now we have seen different traits. the display, debug and the clone traits

-once we have defined a trait, we can implement it on structs and enums. the type promises to honor the trait's contract
    a type implementing the flight trait promises it can fly / it can call a method called fly
-multiple types can implement the same trait.
    a bird and a plane type both implement the flight trait
-a type can implement multiple traits
    a plane can implement both the flight and storage traits
*/
////////////////////////////
////////////////
/*
//DEFINATION OF TRAITS and THEIR IMPLEMENTATION
// DEFAULT IMPLEMENTATION - sometimes we want to do the samething for the method in the trait. so we can give it a body at the first place in the defination of the method at the first place
// or it is usefull when the type don't declare the method so that the default will be applied to it
// A type (any) that implements a trait must define all the methods required by the trait
//  and the structure (the params and their types and the return type) should be similar to the one which is defined

use std::{collections::HashMap, vec};
trait Accommodation {
    //this one is a default implementation - a default method to run in case a type chooses not to individually implement the method
    fn get_descirption(&self) -> String {
        String::from("a wonderful place to stay")
    }
    fn book(&mut self, name: &str, nights: u32);
}

#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}

impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }

    //calling trait method from another method
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_descirption())
    }
}

//with this our "HOTEL" struct should have the methods that are defined in the trait
impl Accommodation for Hotel {
    fn get_descirption(&self) -> String {
        format!("{} is the pinnacle of luxury.", self.name)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

// Airbnb is a company and an online platform that allows people to rent out their homes, apartments, or even single rooms to travelers looking for short-term accommodations.
#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnb {
    fn get_descirption(&self) -> String {
        format!("please enjoy {}'s apartment.", self.host)
    }

    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

//TRAITS FOR FUNCTIONS PARAMETER CONSTRAINTS
//here we can pass any type which implements the accomodation trait
//here "entity" is a mutable reference to some type that implements accomodation - so don't take ownership
// fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
//     entity.book(guest, 1);
// }

////A TRIAT BOUND synthax - requires that a generic type implement a specific trait
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

//let's declare a function with those 2 difference ways to see the difference b/n the the "triat bound syntax" and the "impl" in the parameter
// fn mix_and_match(first: &mut impl Accommodation, second: &mut impl Accommodation, guest: &str) {
//     first.book(guest, 1);
//     second.book(guest, 1);
// }
// Using a trait bound syntax - both `first` and `second` must be the same concrete type that implements the `Accommodation` trait
//but we call this function with different struct types in the main.
//so inorder to make this function work we need another generics type
//SO AS ITS NAME INDICATES IT CREATES A BOUND - I.E if we need the param type to be a specific type and implement a certain trait
fn mix_and_match<T: Accommodation, U: Accommodation>(first: &mut T, second: &mut U, guest: &str) {
    first.book(guest, 1);
    second.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    // println!("{}", hotel.get_descirption());
    // println!("{}", hotel.summarize());
    // hotel.book("Piers", 5);
    // hotel.book("dagim", 3);
    // book_for_one_night(&mut hotel, "Yeabsira");

    // println!("{:#?}", hotel);

    let mut airbnb = AirBnb::new("peter");
    // println!("{}", airbnb.get_descirption());
    // airbnb.book("yeabsira", 3);
    // book_for_one_night(&mut airbnb, "Dagim");

    // println!("{:?}", airbnb);

    //as you can see the first and the second type don't have to be the same type as long as they implement the Accomodation trait
    mix_and_match(&mut hotel, &mut airbnb, "Yeabsira");
    println!("{hotel:#?}, {airbnb:#?}");
}
 */

/*
/////////////////////////////
///////////////////
//MULTIPLE TRAIT BOUNDS
use std::{collections::HashMap, fmt::Display, vec};
trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}

trait Description {
    fn get_descirption(&self) -> String {
        String::from("a wonderful place to stay")
    }
}

#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

//But I want the new constructor for any type T.
impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

//implement for any generic that does implement the "Display" trait, define the methods inside
//means I want the summerize method only on type which implement the Display trait
impl<T: Display> Hotel<T> {
    //calling trait method from another method
    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_descirption())
    }
}

//with this our "HOTEL" struct should have the methods that are defined in the trait
impl<T> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]
struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnb {
    fn get_descirption(&self) -> String {
        format!("please enjoy {}'s apartment.", self.host)
    }
}

//IMPLEMENT BOTH TRATIS USING THE TRAIT BOUND SYNTAX
fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

//what if want to implement both traits for the first variable
// fn mix_and_match(
//     first: &mut (impl Accommodation + Description), //must implement both of them
//     second: &mut impl Accommodation,
//     guest: &str,
// ) {
//     first.book(guest, 1);
//     first.get_descirption();

//     second.book(guest, 1);
//     // second.get_descirption(); //this will create an error, b/c the get_description method is found in the Description trait and the second variable doesn't implement it
// }

//WHERE CLAUSES - IS ANOTHER SYNTAX OPTION THAT WE HAVE AVAILABLE FOR GENERIC TYPES THAT WE WANT TO BIND TO IMPLEMENT ONE OR MORE TRAITS
//let see by coping the mix_and_match function'
fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_descirption();

    second.book(guest, 1);
    // second.get_descirption(); //this will create an error, b/c the get_description method is found in the Description trait and the second variable doesn't implement it
}

fn choose_best_place_to_stay() -> impl Accommodation {
    Hotel::new("The Lux")
}

//and in here we can only return a consistent type for different if cases if we have any
fn choose_best_place_to_stay_v2() -> impl Accommodation + Description {
    Hotel::new("The Lux")

    /*
    //this is not allowed, we have to return a consistent type
    let likes_luxury = true;
    if likes_luxury {
        Hotel::new("The Lux")
    }else{
        AirBnb::new("Peter");
    } */
}

fn main() {
    /*
       // let mut hotel = Hotel::new("The Luxe");
       // let mut hotel = choose_best_place_to_stay(); //the type will not be hotel. it will be sometype that implements the accomodation trait
       let mut hotel = choose_best_place_to_stay_v2(); //the type will not be hotel. it will be sometype that implements the accomodation and description trait
       let mut airbnb = AirBnb::new("peter");

       //if we use the "choose_best_place_to_stay" this will be an error. b/c the "first" variable in the mix_and_match function expects
       //  that the first variable implement both the Accommodation and the Description traits. but here the returned values from the "choose_best_place_to_stay"
       //  is only guaranteed to implement the Accomodation trait
       // and we can correct it by using "choose_best_place_to_stay version2" function
       mix_and_match(&mut hotel, &mut airbnb, "Yeabsira");
    */

    /*
       let hotel1 = Hotel::new(String::from("The Lux"));
       //the sumarize method exists b/c the "String" type implements the Display trait
       println!("{}", hotel1.summarize());

       let hotel2 = Hotel::new("The Golden Standard");
       //the sumarize method exists b/c the "&str" type implements the Display trait
       println!("{}", hotel2.summarize());

       let hotel3 = Hotel::new(vec!["The Sweet Escape", "Hilton Edition"]);
       //the sumarize method DON'T exists b/c the "VECTOR" type DON'T implements the Display trait
       //  println!("{}", hotel3.summarize()); //so this will be a compile time error
    */

    ////////////////////////
    //A TRIAT OBJECT - is an instance of a type that implements a particular trait whose methods will
    //                 be accessed at runtime using a feature called DYNAMIC DISPATCH
    //DYNAMIC DISPATCH - Rust will figure out the exact types of the things and the methods they can invoke at RUNTIME.(opposite of static type)
    //so  dynamic dispatch is slower the static dispatch. b/c the  compiler can not optimize for exactly what it is working with
    let mut hotel = Hotel::new(String::from("The Luxe"));
    let mut airbnb = AirBnb::new("Yeabsira");
    // let stays = vec![hotel, airbnb]; //this will be compile time error, b/c the types in the vector should be similar, but they are different here
    //so to give as that DYAMISIM, WE WILL USE A KEYWORD CALLED "DYN" (similar as the "impl" for function parameters and return types). SO WE DON'T CARE ABOUT A SPECIFIC CONCRETE TYPE,
    //   WE DO CARE ABOUT THE TYPE THAT IMPLEMENTS A GIVEN TRAIT
    let stays: Vec<&dyn Description> = vec![&hotel, &airbnb]; //so the elements in the vector are called trait objects

    println!("{}", stays[0].get_descirption());
    println!("{}", stays[1].get_descirption());

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut hotel, &mut airbnb]; //so the elements in the vector are called trait objects
    stays[0].book("Piers", 2);
    stays[1].book("Amanda", 3);

    println!("{:#?}", hotel);
    println!("{:#?}", airbnb);
}
 */
////////////////////
//////
/*
use std::ops::Add;
use std::str::FromStr;
fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    // The `add` method comes from the `Add` trait, which is implemented for `i32`.
    // By importing the `std::ops::Add` trait, we ensure that `i32` values can use the `.add()` method explicitly.
    let sum = a.add(b);

    let numeric_count = u64::from_str("5");
    println!("{}", numeric_count.unwrap());
}
 */
