//An enum i s a type that represents a set of possible values.
//Each possible value is called a variant
//enum comes from the word enumerate - means mention one by one
//an enum is useful when a type represents a value from a limited collection of possible values.
//enum name should be in pascal case / uppercase first letter
/*
    ALL OWNERSHIP PRINCIPLES WORK FOR TUPLES LIKE ANY OTHER HEAP DATAS.
*/

/*
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit, //we can use the enums any where we use data types.
}

fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Diamonds];
    println!("{:?}", card_suits);

    let card_suits = (CardSuit::Hearts, CardSuit::Spades);
}
 */

//////////////////
/// ENUM WITH ASSOCIATED VALUES I
// enum PaymentMethodType {
//     CreditCard, //these are called VARIANTS
//     DebitCard,
//     PayPal,
// }
/*enum PaymentMethodType {
    CreditCard(String), //THESE ARE CALLED TUPLE VARIANTS
    DebitCard(String),
    PayPal(String),
}

fn main() {
    //many times we will want to combine a variant with some additional daa that is related to it.
    // 1, the first method is using tuple and add any related data with the selected value from the enum.
    //but this is not the ultimate way to do.
    // let visa: (fn(String) -> PaymentMethodType, String) = (
    //     PaymentMethodType::CreditCard,
    //     String::from("2345-6437-8936-2056"),
    // );

    // // 2, the best way to do it is to define the related datas in the enum defination with RUST's nice syntax.
    // let visa = PaymentMethodType::CreditCard(String::from("1234-5678"));
    // let master_card = PaymentMethodType::DebitCard(String::from("1234-5678"));
    // println!("{:?}", visa);
    // println!("{:?}", master_card);

    let visa = PaymentMethodType::CreditCard(String::from("1234-0987"));
}
 */
////////////////
//////
///
//ENUM WITH ASSOCIATED VALUES 2
/*
we can have the fields with difference type and number of associated values
*/

// enum PaymentMethodType {
//     CreditCard(String),
//     DebitCard(String),
//     PayPal(String, String),
// }
/*
fn main() {
    let mut my_payment_method = PaymentMethodType::CreditCard(String::from("0123-9876"));

    my_payment_method =
        PaymentMethodType::PayPal(String::from("yes@gmail.com"), String::from("password"));

    println!("{:?}", my_payment_method);
}
*/

//STRUCT VARIANTS
/*
TILL NOW WE HAVE SEEN 2 VARIANTS
 -NO DATA and TUPLE variants
 -now we see the third type - STRUCT type
 STRUCT TYPE - stores associated data in fields rather than by position.
             - each piece of data has associated name.
*/

/*
#[derive(Debug)]

struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug)]

enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    // Represents a PayPal payment with two associated values:
    // The first String is the user's email, and the second String is the password.
    // ❗️ Cons of this tuple-style variant:
    // - Lack of clarity: It's not obvious what each String represents without a comment or extra documentation.
    // - Error-prone: The order of the values matters, so swapping the email and password by mistake won’t trigger a compile error — but it'll break functionality.
    // - Harder maintenance: If the variant needs more fields later (e.g., a 3rd value for a security token), this tuple style becomes messy and less readable.
    // PayPal(String, String),

    //this is technically a struct type. just using structs for the fields.
    //but it is not the better way to do it. we need more code to do this
    PayPal1(Credentials),

    //the folowing is the struct type - this is the syntax that RUST give us for thr struct variant.
    PayPal2 { username: String, password: String },
    cash,
}

fn main() {
    let paypal_credentials = Credentials {
        username: String::from("Yeabsira"),
        password: String::from("1234-0987"),
    };

    let paypal = PaymentMethodType::PayPal1(paypal_credentials);

    let paypal2 = PaymentMethodType::PayPal2 {
        username: String::from("Yeabsira"),
        password: String::from("12345678"),
    };

    println!("{:?}", paypal);
}
 */
////////////
/// NESTING ENUMS IN ENUMS
///
///
/*
#[derive(Debug)]

enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]

enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]

enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}
fn main() {
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("lunch was {lunch:?} and dinner was {dinner:?}");
}
 */
//////////////////
///
/// THE MATCH KEY WORD
/// //this is a REMINDER for match key word
/* fn main() {
    let number: i32 = 20;

    match number {
        5 => println!("the number is 5"),
        6 => println!("the number is 8"),
        _ => println!("it is something else"),
    }
}
*/

//THE RUST COMPILER WILL VALIDATE THAT THE MATCH EXPRESSION HANDLES EVERY POSSIBLE CASE THAT THE DYNAMIC VALUE COULD BE.
//THIS MAKES THE MATCH KEYWORD A PERFECT KEYWORD TO USE IN COMBINATION WITH AN ENUM, B/C "MATCH" WILL VALIDATES THAT EVERY POSSIBLE VARIANT IS ACCOUNTED FOR.
/*
enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
}

fn main() {
    let my_computer = OperatingSystem::Windows;
    let age = years_since_release(my_computer);
    println!("My computers operating system is {age} years old.")
}

// fn years_since_release(os: OperatingSystem) -> i32 {
//     match os {
//         OperatingSystem::Windows => 39,
//         OperatingSystem::MacOs => 23,
//         OperatingSystem::Linux => 34,
//     }
//     //now RUST compiler is happy b/c we covered every case that is in the ENUM.
// }

fn years_since_release(os: OperatingSystem) -> i32 {
    match os {
        OperatingSystem::Windows => {
            println!("quite an old operating system");
            39
        }
        OperatingSystem::MacOs => 23,
        OperatingSystem::Linux => 34,
    }
}
 */

/*
//IF AN ENUMS CONTAINS AN ASSOCIATED DATA, WE CAN ACCESS THAT DATA IN A CORRESPONDING MATCH ARM.
//BUT THE SYNTAX IS DIFFERENT BASED ON THE KIND OF ENUM VARIANT

////////DEFINING METHODS ON ENUMS - like that of structs we can define methods on enums
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => println!("Running a laundry with cold tempretature."),
            LaundryCycle::Hot { temperature } => {
                println!("Running the laundry with a temperature of {temperature}.")
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Runnig the laundry with a delicate cycle for {fabric_type}.");
            }
        }
    }
}

fn main() {
    LaundryCycle::Cold.wash_laundry();
    LaundryCycle::Hot { temperature: 30 }.wash_laundry();
    LaundryCycle::Delicate(String::from("Trousers")).wash_laundry();
}
 */

 /*
//CATCH MULTIPLE VALUES
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
            //     println!("your item is being prepared for shipment")
            // }
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered");
            } // _ => { //this will catch everything. but we can give it a name so we can then use it in the block
            //     println!("your item is not there yet");
            // }
            other_status => {
                println!("your item is {other_status:?}")
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Delivered.check();
    OnlineOrderStatus::Packed.check();
}
 */
/////////THE MATCH KEYWORD V - match with exact value
///
///

/*
enum Milk {
    LowFat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        //with the first 2 will not fullfill everything that should be covered. b/c
        // the we matched the LowFat with a certain value and any other value is valid there
        //so we need to add a more general one for the "LowFat" type.
        //REMEBER : the more generalized format should be after the specific one. otherwise the specific one will be unreachable.
        match self {
            Milk::LowFat(2) => {
                //this will match the lowfat variant but with value 2 only,
                println!("Delicious, 2% milk is my favorite");
            }
            Milk::LowFat(percent) => {
                println!("you've got the lowfat {percent} percent version");
            }
            Milk::Whole => {
                println!("you've got the whole milk");
            }
        }
    }
}

fn main() {
    Milk::LowFat(1).drink();
    Milk::LowFat(2).drink();
    Milk::Whole.drink();
}
 */
/*
////////////////
///
/// THE IF LET CONSTRUCT - combines an if statement with a variable declaration
//like the "MATCH", it is used to compare a hard-coded enums variant with some dynamic/ upredictable one.
//using "MATCH" is sometimes unuseful as it needs foe every scenario to be catched.

enum Milk {
    LowFat(i32),
    whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::whole;

    //this means if my_beverage is a "whole" variant, execute the block
    if let Milk::whole = my_beverage {
        println!("you have whole milk");
    }

    let my_beverage = Milk::LowFat(2);

    //you might wonder, where the variable decalration occurs
    //the decalration occur when you have a variable with associated data.
    if let Milk::LowFat(percent) = my_beverage {
        println!("your beverage is {percent} milk");
    }

    let my_beverage = Milk::NonDairy {
        kind: String::from("Oat"),
    };

    if let Milk::NonDairy { kind } = my_beverage {
        println!("your beverage is type of {kind}.");
    } else {
        println!("you have some other variant");
    }
}
 */
/*
////THE LET ELSE CONSTRUCT - it is the oppsite of the "if let construct"
/// it executes a block if a dynamic value DOES NOT match a hard coded value.
///in addition, it allows us to define a variable that will live after the let else constructs concludes.
enum Milk {
    LowFat(i32),
    Whole,
    NonDairy { kind: String },
}

fn main() {
    let my_beverage = Milk::LowFat(2);

    //the block will be executed if my_beverage is not equal to a LowFat variant.
    //and the "percent" variable is going to be available after the block, basically as long as the main function is running.
    //but the "PERCENT" variable will be declared only if the block is NOT executed.
    //b/c executed means only the right part is done / the else part
    let Milk::LowFat(percent) = my_beverage else {
        println!("you do not have the lowfat milk");
        return; //we need these b/c if the right part is executed, we won't have a variable, and accidentally we may try to use it. so rust will complain if we don't do this.
    };

    println!("{percent}% milk is available here");
}
 */
//////////////////////////////////////////////////////
///
/// CODING CHALLENGE

#[derive(Debug)]

enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premium { tier: Tier },
}

impl Subscription {
    fn summarize(self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site")
            }
            Subscription::Basic(price, months) => {
                println!("you have limited access to he site's premium features for {price} and {months} months,, where {price} amd {months} come from the tuple variant's associated data.")
            }

            Subscription::Premium { tier } => {
                println!(
                    "You have full access to the site's premium features. Your tier is{tier:?}, where {tier:?} comes from the struct variant's associated 'tier' field."
                )
            }
        }
    }
}
fn main() {
    let subscription_free = Subscription::Free;
    let subscription_basic = Subscription::Basic(1000.0, 2);
    let subscription_premium = Subscription::Premium { tier: Tier::Gold };

    subscription_free.summarize();
    subscription_basic.summarize();
    subscription_premium.summarize();
}
