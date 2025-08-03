//ASSOCIATED CONSTANT - is a constant declared within the trait.
// a constant is a name for a fixed immutable value
// used to store fixed unchanging immutable data

/*
trait Taxable {
    const TAX_RATE: f64 = 0.25;

    // fn tax_bill(&self) -> f64;
    //tax_bill method is doing more or less similar things. so it is a candidate to be default implementation
    //but rust compiler will not be happy b/c the it does not know about the amount field. / what if another struct without amount fields comes
    //SO ONE SOLUTION IS TO USE A GETTER METHOD - is a method that retrieves a piece of data. it "gets" a piece of data from a type
    // fn tax_bill(&self) -> f64 {
    //     self.amount * Self::TAX_RATE
    // }

    fn amount(&self) -> f64; //getter method - which will give us the float value we need to calculate the tax_bill

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }

    //and now we want a general implementation to double the gain you got from any taxable income
    //but the following implementation will not work for the same reason we talked in the getter function usage
    //SO ONE SOLUTION IS USING SETTER METHOD - is a method that writes a piece of data. it "sets" a piece of state
    // fn double_amount(&mut self) {
    //     self.amount = self.amount() * 2.0;
    // }

    fn set_amount(&mut self, new_amount: f64); //setter method
    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0); //getter and setter combined
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Taxable for Income {
    // fn tax_bill(&self) -> f64 {
    //     //but how to get the constant from the trait?
    //     // answer - we will use "Self"- uppercase S, which is the type
    //     self.amount * Self::TAX_RATE
    // }

    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

struct Bonus {
    value: f64,
}

impl Taxable for Bonus {
    //what is we have a different TAX_RATE for this?
    // answer: we can override it in this impl scope
    const TAX_RATE: f64 = 0.5;
    // fn tax_bill(&self) -> f64 {
    //     self.value * Self::TAX_RATE
    // }

    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

fn main() {
    let mut income: Income = Income { amount: 5000.00 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let mut bonus: Bonus = Bonus { value: 5000.00 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
}
*/

/*
/* SUPERTRAIT - is a trait from which another trait inhertis functionality.
- the parent is called the supertrait and the child is called the subtrait
-the child trait can inherit the behaviours from the parent trait and can also define its own requirements
-ANY TYPE IMPLEMENTS THE CHILD TRAIT MUST ALSO IMPLEMENT THE PARENT TRAIT
 */

//GIVE A FLEXIBILITY TO THE RETURNED THINGS FROM THE VARIABLE. FOR EG: SOME OF THEM MAY WANT TO RETURN A FLOAT FOR THE DOUBLE FUNCTION, BUT SOME MAY WANT TO RETURN INTEGER
trait Investment<T> {
    fn amount(&self) -> T; //getter method - which will give us the float value we need to calculate the tax_bill

    // fn set_amount(&mut self, new_amount: T); //setter method

    //we want to use generics, so let's comment out this function.b/c we use genercis , so the default implementation is not applicable as the multipyling value determines the type like 2, 2.0
    // fn double_amount(&mut self) {
    //     self.set_amount(self.amount() * 2.0); //getter and setter combined
    // }
    fn double_amount(&mut self);
}

// : -> means inherit
//so any type implements the taxable trait must also define methods in the parent trait "Investment"
trait Taxable: Investment<f64> {
    //it is enough for the generic to be f64 for this trait. so no need to use "T" here
    const TAX_RATE: f64 = 0.25;

    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64,
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.amount = new_amount;
    // }

    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64,
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    // fn set_amount(&mut self, new_amount: f64) {
    //     self.value = new_amount;
    // }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5;
    fn tax_bill(&self) -> f64 {
        self.value * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}
//WE CAN ONLY IMPLEMENT PARENT TRAITS
impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }
    // fn set_amount(&mut self, new_amount: f64) {
    //     self.minutes = new_amount;
    // }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let mut income: Income = Income { amount: 5000.00 };
    println!("Total tax owed: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owed: ${:.2}", income.tax_bill());

    let mut bonus: Bonus = Bonus { value: 5000.00 };
    println!("Total tax owed: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Total tax owed: ${:.2}", bonus.tax_bill());

    let mut rust_programming_time = QualityTime { minutes: 120 };
    println!(
        "RUST programming time : {:.2}",
        rust_programming_time.amount()
    );
    rust_programming_time.double_amount();
    println!(
        "RUST programming time : {:.2}",
        rust_programming_time.amount()
    );
}
 */

/////////////////////////////
//CUSTOM DISPLAY TRAIT FOR STRUCTS
//let's create a custom display trait for structs which doesn't implement th builtin display trait as we know
/*
/*
//the standard Rust display trait
pub trait Display {
//Required method
 fn fmt(&slef, f: &mut Formatter<'_>) -> Result<(), Error>

//  Formatter<'_> - is a struct in fmt sub-module that acts as a buffer or destination where your formatted string gets written when implementing traits like Display or Debug
//f - is a place whose type is Formatter where we write the content of the string
}
//they are the place and the type respectively
*/

<<""
GENERALLY THINGS PRINTED BY PRINTLN!() MACRO ARE PRINTED ONLY B/C THEY IMPLEMENT THE DISPLAY TRAIT BY DEFAULT
    SO WE CAN IMPLEMENT THAT TRAIT FOR THINGS LIKE STRUCTS AND ENUMS (WHICH DON'T IMPLEMENT IT BY DEFAULT) AND
        AND GIVE A DEFAULT IMPLEMENTATION FOR THE OBLIGED METHOD IN THE TRAIT AND MAKE "PRITNTLN" WORK FOR THEM
"">>
use std::fmt::{
    Display,
    Formatter,
    Result, // this is created from Result enum but ()-empty tuple for the associated data
};

struct Apple {
    kind: String,
    price: f64,
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        //formatter: is a place where we write the content of the string
        //write!() - macro is used to write formatted data into buffer that implements the std::fmt::Write trait. it works similarly like println!, but instead of printing to the console, it writes to a destination like a string
        write!(formatter, "{} 🍏for {}", self.kind, self.price)
    }
}
fn main() {
    let lunch_snack = Apple {
        kind: String::from("Granny Smith"),
        price: 1.04,
    };

    //no error b/c we implement the display trait for the struct
    println!("{}", lunch_snack);
}
 */
/*
////////////////////////
//LET'S TAKE THE IMPLEMENTATION OF THE DISPLAY TRAIT TO THE NEXT LEVEL BY MAKING
//WE WILL ALSO DO A CUSTOM IMPLEMENTATION OF DEBUG TRAIT
/*

DROP TRAIT
every heap allocated type owner (the variable) is responsible for deallocating that heap memory when the name or the owner goes out of scope
all heap datatypes implements the DROP TRAIT
 any type which implements the drop trait, Rust will automatically call the "drop" method when the instance is being cleaned up
we can do out DROP mechanism for our custom types
*/
//  THE "KIND" FIELD IN THE APPLE STRUCT AN ENUM
use std::fmt::{
    Debug,
    Display,
    Formatter,
    Result, // this is created from Result enum nad found in the fmt module but ()-empty tuple for the associated data
};
use std::fs;
use std::ops::Drop;

enum AppleType {
    RedDelicious,
    GrannySmith,
}

struct Apple {
    kind: AppleType,
    price: f64,
}

//as you can see we will have 2 "fmt" methods for appletype. will that be a problem?
//  - no, it is not a problem. RUST will differentiate them based on how we want to print the data. like {} or {:?}
impl Display for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "🍎 Delicious 🍎"),
            AppleType::GrannySmith => write!(formatter, "🍏 Granny Smith 🍏"),
        }
    }
}

impl Debug for AppleType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(formatter, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(formatter, "AppleType::GrannySmith"),
        }
    }
}

impl Display for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // formatter: is a place where we write the content of the string
        // write!() - macro is used to write formatted data into buffer that implements the std::fmt::Write trait. it works similarly like println!, but instead of printing to the console, it writes to a destination like a string
        write!(formatter, "{} for {}", self.kind, self.price)
    }
}

impl Debug for Apple {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        // write!(
        //     formatter,
        //     "Apple ::: [kind:{:?}, price: {}]",
        //     self.kind, self.price
        // )

        ////////////////////
        //FORMATTER METHODS - they are an alternate way that can create our desired string for either
        formatter
            .debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Cost", &self.price)
            .finish() //change the chained debug structs to Result
    }
}

//DROP TRAIT - it helps us when we want to do things when a certain instance goes out of scope and cleaned up from the memory
//let's mimic that the Apple trait saves things that it needs for execution in the "apple.txt" file and
//  and when the function goes out of scope we want to remove the file
//the second instance of the struct will have nothing to delete b.c the first one deletes that file. but this is just for example
impl Drop for Apple {
    //this will be executed when the structs goes out of scope
    fn drop(&mut self) {
        let result = fs::remove_file("apple.txt");
        match result {
            Ok(_) => println!("Good bye my sweet apple"),
            Err(error) => eprintln!("Error deleting file {}", error),
        }
    }
}

fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };

    //no error b/c we implement the display trait for the struct
    // println!("{}", lunch_snack);
    println!("{:?}", lunch_snack);

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.35,
    };
    // println!("{}", dinner_snack);
    println!("{:?}", dinner_snack);
}
 */
////////////////////////////
///////////////////
//IMPLEMENTING THE CLONE TRAIT
/*
use std::clone::Clone;
struct Appointment {
    doctor: String,
    start_time: String,
    end_time: String,
}

impl Appointment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: String::from(doctor),
            start_time: String::from(start_time),
            end_time: String::from(end_time),
        }
    }
}

impl Clone for Appointment {
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}

fn main() {
    let morning_apps = Appointment::new("Dr. Yeabsira", "9AM", "10AM");
    let replacement_apps = morning_apps.clone();

    println!(
        "{} is seeing from {} to {}",
        replacement_apps.doctor, replacement_apps.start_time, replacement_apps.end_time
    );
}
 */

/*
/////////////////what about cloning the non-clonable datas by default like struct with just equality sign (remove the move of ownership)
// IMPLEMENTING THE COPY TRAIT, which is a marker trait that implies the type can be duplicated simply by copying bits.
// Types that implement Copy do not require the clone method for duplication.
// Copy is a subtrait of Clone, meaning all Copy types are also Clone, but not all Clone types are Copy.
//    as copy trait is the child of the clone trait, if we want to implement the copy trait, we should implement the clone trait / the parent as well

//but to implement copy trait for structs and the like the things inside the struct should copy the copy trait by default as well

#[derive(Debug, Clone)] //by the way we can implement the clone trait without default implementation like this
struct Duration {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Duration {
    fn new(hours: u32, minutes: u32, seconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl Copy for Duration {} //we don't need implementation, it knows how to copy from its parent "clone trait"

fn main() {
    let one_hour = Duration::new(1, 0, 0);
    // let another_hour = one_hour; //ownership will be moved if we don't implement the copy trait
    //// println!("{:?}", one_hour);  //not valid if the copy trait is not implemented

    //COPY TRAIT IMPLEMENTED
    let another_hour = one_hour;
    println!("{:?}", one_hour); //here as you can see still valid
}
 */

/*
///////////////////////////
//IMPLEMENTING THE PARTIALEQ TRAIT FOR STRUCTS
// "partialeq trait" - established equality b/n 2 values
//if we implement this trait, code can apply like the equality operator and the inequality operator to our values to
//   check whether they are to be considered equal

// #[derive(PartialEq)] //we can implement using the derive keyword, but rust gives us its default implementation, which is for structs every field should be the same, which don't have to in our context

struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}

impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

//let's say for our flights, equality is when the "origin" and the "destination" are the same. the "time" of departure doesn't matter
impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
    //we don't have to declare the method for the inequality case. it is a default method.
    // "you can if you want, but RUST can figure out what is inequality based on your 'equality defination' "
}

/*
pub trait PartialEq<Rhs: ?Sized = Self>  -this is the default in Rust the "Rhs (right hand side)" generic is Self by default, but we can ignore the default inorder to create equality of different types
*/
//so by this we make the other (the right hand side to "BusTrip")
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

// impl PartialEq<Flight> for BusTrip {
//     fn eq(&self, other: &Flight) -> bool {
//         self.time == other.time
//     }
// }

fn main() {
    let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "23:20");
    let c = Flight::new("New York", "Los Angeles", "08:00");

    let d = BusTrip::new("Addis Ababa", "Debre Zeit", "08:00");

    //if we remove the the "PartialEq" implementation block, the equality operator will generate an error. b/c structs don't implement it, so they can't use
    //     equality operation by default
    //the method called when we use "==" is the "eq" method. the one found in the implemented block
    //we have access to the default method for inequality as well. "ne()"
    println!("{}", a == b); //true
    println!("{}", a.eq(&b)); //true
    println!("{}", a.ne(&b)); //false
    println!("{}", a == c); //false
    println!("{}", a != c); //true

    println!("{}", a == d); //different types but equality now works

    //but exchaning the places of types in the comparison will not work
    //b/c the method is called to the "leftside" and it doesn't implement the PartialEq trait (it is only the type of the "other")
    //inorder for this to work, we have to implement for the PartiaEq for the bustrip as well
    // println!("{}", b == a);
}
 */

/*
////////////////////////////
//////////////////
//IMPLEMENTING THE PARTIALEQ TRAITS FOR ENUMS

// #[derive(PartialEq)] //- we can implement the RUST'S default implementation
enum Muscician {
    SingerSongwritter(String), //a single song writter
    Band(u32),                 //the number of musicians in the band
}

impl PartialEq for Muscician {
    fn eq(&self, other: &Self) -> bool {
        match self {
            SingerSongwritter(name) => match other {
                SingerSongwritter(other_name) => name == other_name,
                Band(_) => false,
            },
            Band(members) => match other {
                SingerSongwritter(_) => false,
                Band(other_members) => members == other_members,
            },
        }
    }
}

use Muscician::{Band, SingerSongwritter}; //just to use the enum variants directly

fn main() {
    let rustin_bieber = SingerSongwritter("Rustin".to_string());
    let rustin_timberlake = SingerSongwritter("Rustin".to_string());
    let holly = SingerSongwritter("Holley".to_string());

    let rust_no_one = Band(5);
    let unrustworthy = Band(4);
    let rust_for_vengeance = Band(5);

    println!("{}", rustin_bieber == holly);
    println!("{}", rustin_bieber == rustin_timberlake);
    println!("{}", rustin_bieber == rust_no_one);

    println!("{}", rust_no_one == unrustworthy);
    println!("{}", rust_no_one == rust_for_vengeance);
}
 */

/*
/////////////////////////////////////////
/////////////////////////////////////////
//IMPLEMENTING THE "EQ" TRAITS - is a sub trait of the "PartialEq" super trait. so if the type choose to implement the "Eq" triat, it must implement the "PartialEq" super trait
// "EQ" trait doesnot need implementation. it is just a flag. it declares 3 addtional things to apply.
/*
     1, reflexive, a == a;
     2,symmetric, a == b implies b == a
     3,transitive, a == b and b == c implies a == c
*/
#[derive(PartialEq, Eq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}

impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

fn main() {
    /* let a = Flight::new("New York", "London", "08:00");
    let b = Flight::new("New York", "London", "08:00");
    let c = Flight::new("New York", "London", "08:00");

    println!("{}", a == a);
    println!("{}", a == b);
    println!("{}", b == c);
    println!("{}", a == c); */

    let division = 0.0 / 0.0; //gives as a f64
    println!("{}", division); // NaN

    let value = 3.4;
    println!("{}", value == value); // true

    // NaN is not equal to itself, so this prints false.
    // According to the IEEE 754 standard, NaN != NaN.
    // That's why floating-point types only implement PartialEq (not Eq).
    println!("{}", division == division); // false
}
 */

/*
//////////////////////////////////////
//////////////////////////////////////
//IMPLEMENTING THE PARTIALOrd TRAIT - trait indicates that a type can be ordered / sorted
//so with these we can make our types use ">, <, <=, >="
// it is a sub-trait of the partialEq super trait

use std::cmp::Ordering;

struct Job {
    salary: u32,
    commute_time: u32,
}
impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.salary == other.salary
    }
}

impl Eq for Job {}

impl PartialOrd for Job {
    // fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {}
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if self.salary == other.salary {
        //     Option::Some(Ordering::Equal)
        // } else if self.salary < other.salary {
        //     Option::Some(Ordering::Less)
        // } else if self.salary > other.salary {
        //     Option::Some(Ordering::Greater)
        // } else {
        //     Option::None
        // }

        //but what we can do is that the "salary which is u32" implements the PartialOrd trait
        //   so we can call a partial_cmp function on it
        // match self.salary.partial_cmp(&other.salary) {
        //     Some(Ordering::Equal) => Some(Ordering::Equal),
        //     Some(Ordering::Greater) => Some(Ordering::Greater),
        //     Some(Ordering::Less) => Some(Ordering::Less),
        //     None => None,
        // }

        //as you can see in the above, we are returing what we got from the computation. so what abt directly returning it
        self.salary.partial_cmp(&other.salary)
    }
}

fn main() {
    let long_commute_job = Job {
        salary: 100000,
        commute_time: 2,
    };

    let short_commute_job = Job {
        salary: 75000,
        commute_time: 1,
    };

    //we can use methods as well ls(), gt(), le(), ge()
    println!("{}", long_commute_job > short_commute_job);
    println!("{}", long_commute_job < short_commute_job);
    println!("{}", long_commute_job == short_commute_job);
}
*/
/////////////////////////////////////////////////
////////////////////////////
//ASSOCIATED TYPES  - an associated type is a placeholder for a type that is required within a trait
//                  -it is like generics (a placehoder for a future type), but only works for generics
/*
use std::ops::Add;

#[derive(Debug)]
struct Lunch {
    cost: f64,
}

impl Add for Lunch {
    //LET'S saying addition of 2 Lunch structs is a float.
    // type Output = f64; //this is the associated type. (you can see it in the defination)
    // fn add(self, rhs: Self) -> Self::Output {
    //     self.cost + rhs.cost
    // }
  
    type Output = Lunch; //let's make the addition to return a "Lunch" type
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

fn main() {
    let one = Lunch { cost: 19.99 };
    let two = Lunch { cost: 29.99 };

    println!("{:?}", one + two);
}
 */

/*
//this code will not compile b/c rust can't gurantee that the type "T" supports addition with "+" operator
fn add_two_numbers<T>(a: T, b: T) -> T {
    a + b
}
 */
/*
use std::ops::Add;

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let integer_sum = add_two_numbers(1, 2);
    let float_sum = add_two_numbers(1.5, 2.4);

    println!("{} and {}", integer_sum, float_sum);
}
 */

///////////////////////////
/////////////////
//CODING CHALLENGE
use std::fmt::{Debug, Display, Formatter};

trait Drinkable {
    fn consume(&mut self);
    fn get_data(&self) -> String; // a getter
    fn stats(&self) {
        println!("{}", self.get_data());
    }
}

#[derive(Debug)]
enum Milk {
    Whole,
    Oat,
    Almond,
}

struct Coffee<T> {
    kind: T,
    milk: Milk,
    ounces: u32,
}

impl<T> Coffee<T> {
    fn new(kind: T, milk: Milk, ounces: u32) -> Self {
        Self { kind, milk, ounces }
    }
}

impl<T: Debug> Debug for Coffee<T> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("COFFEE")
            .field("kind", &self.kind)
            .field("MILK", &self.milk)
            .field("OUNCES", &self.ounces)
            .finish()
    }
}

impl<T: Display> Drinkable for Coffee<T> {
    fn consume(&mut self) {
        self.ounces = 0;
    }

    fn get_data(&self) -> String {
        format!("A delicious {} ounce {}", self.ounces, self.kind)
    }
}

#[derive(Debug)]
struct Soda {
    calories: u32,
    price: f64,
    flavor: String,
    percentage: u32,
}

impl Soda {
    fn new(calories: u32, price: f64, flavor: String) -> Self {
        Self {
            calories,
            price,
            flavor,
            percentage: 100,
        }
    }
}

impl Drinkable for Soda {
    fn consume(&mut self) {
        self.percentage = 0;
    }

    fn get_data(&self) -> String {
        format!("Flavor: {}, Calories, :{}", self.flavor, self.calories)
    }
}

impl Display for Soda {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "** {} Soda **", self.flavor)
    }
}

impl Clone for Soda {
    fn clone(&self) -> Self {
        Self {
            calories: self.calories,
            price: self.price,
            flavor: self.flavor.clone(),
            percentage: self.percentage,
        }
    }
}

impl PartialEq for Soda {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Soda {}
fn main() {
    let mut latte = Coffee::new("Latte", Milk::Oat, 32);
    println!("{:?}", latte);
    latte.consume();
    println!("{:?}", latte);

    let cappuccino = Coffee::new(String::from("Cappuccino"), Milk::Almond, 20);
    println!("{}", cappuccino.get_data());

    let pepsi = Soda::new(300, 2.99, String::from("cherry"));
    println!("{}", pepsi);

    let mut coke = pepsi.clone();
    println!("{}", pepsi == coke);
    coke.consume();
    println!("{:?}", coke);
}
