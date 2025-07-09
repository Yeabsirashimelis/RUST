//CLOSURE - is a function without a name. it is sometimes called an anonymous function or a lambda
//  they are key in functional programming paradignm
//functional programing treats a function like any other value in a program. eg: we can pass a function as an argument, we can return a function or assign a function to a variable

// REGULAR functions - can't access data from the external world
//CLOSURES - can access the data from the outer scope/ external world or containing environment
//             primarly meant to be used for short, one off specific short procedures

/*
//NESTED FUNCTIONS
fn main() {
    let multiplier = 5;

    // //it is only avaliable in the main function scope as any other values in RUST
    // fn multiply_by(value: i32) -> i32 {
    //     value * multiplier //this will be an error. b/c the data is from external world
    // }

    //this is a closure and as we can see its type is a thing which implements the Fn() - so it is callable like normal functions
    // let multiply_by = |value: i32| -> i32 { value * multiplier };

    //in closures, we don't have to manually specify the return type, it knows by itself
    // if there is a single line of code in the block of the closure we can remove the curly braces
    //also if we don't speicify the type of the parameters, they are infered by the first call of the closure so that any other call below must obey that
    let multiply_by = |value| value * multiplier;

    println!("{}", multiply_by(3));

    let product = |a: i32, b: i32| -> i32 {
        return a * b;
    };

    println!("{}", product(3, 10));
    println!("{}", product(5, 8));
}
 */

//////////////////////////////////////////////////
/////////////////////////////////////////
//TRAIT HIERARCHY
/*
FnOnce (parent) - closure captures values by move (transfering ownership)
       -closure will be invoked once
FnMut (child) -captures values by mutable reference
       -closure can be invoked multiple times
Fn (grand-child) -closure captures values by immutable reference (read only) or doesn't capture anything at all
       -closure can be invoked multiple times

 if a function requires the parameter to be a type that implements fnonce trait, then that function can accept an fnone closure.
     but it can also accept an fn mute closure or fn closure (additive)
*/

/*
//CLOSURES THAT CAPTURE IMMUTABLE REFERENCE -"Fn type"
fn main() {
    let multiplier = 5;

    //this one is just Fn - b/c the i32 implements the copy trait. the above concepts are for heap datas
    let multiply_by = |value| value * multiplier;
    println!("{}", multiply_by(3));

    let numbers = vec![4, 8, 15, 16, 23, 42];
    println!("{:?}", numbers);

    //Fn type closure - we can call it as many times as we want
    /*
       Immutable Reference Capture:
    Your closure reads numbers but doesn’t modify it.
    So, Rust will automatically capture numbers by immutable reference (&numbers).
    Multiple Invocation is Safe:
    Since it only reads numbers, it’s safe to call the closure many times.
    No mutation or ownership is involved, so it implements the Fn trait.
        */
    let print_numbers = || {
        println!("{:?}", numbers);
    };
    print_numbers();
    print_numbers();
    print_numbers();

    println!("{:?}", numbers); //it works, b/c there is no transfer of ownership
}
 */
///////////////////////////////
/*
//CLOSURES THA CAPTURES MUTABLE REFERENCE
fn main() {
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    //FnMut type - it caputures the a mutable reference to the value
    //  inorder for the code to work, we have to make the closure mutable, b/c for each call the value it takes can be
    //    will be changed, so the closure which takes it should also be changed
    let mut add_number = || numbers.push(100);
    // println!("{:?}", numbers); //this won't work. b/c we are trying to print the data as it was (immutable reference)
    //                            but on the previous line we have mutable reference to that number. and 2 references are not allowed at the same time
    add_number();
    // println!("{:?}", numbers); //this won't work for the same reason as the above
    add_number();
    println!("{:?}", numbers); //this one work b/c the lifetime of the mutable reference ended here
}
 */
/*
//////////////////////////////////////
//CLOSURES WITH OWNERSHIP
fn main() {
    let number = 13;
    //Fn type. b/c i32 implements the copy trait and for every call it copies the value. so can be called multiple times
    let capture_number = || number;

    let a = capture_number();
    let b = capture_number();
    println!("{} {} {}", a, b, number);

    let first_name = String::from("Yeabsira");
    //Fn once type - we can't call the closure more than one time. move will occur to the taker of the returned value
    let capture_string = || first_name;

    // println!("{first_name}"); //this is not valid, b/c there is a move of ownership
    let owner = capture_string(); //the new owner (recieve the returned string)

    let first_name = String::from("Yeabsira");
    //similar here. there is a move of ownership to inside declared variable
    let capture_string2 = || {
        let person = first_name;
        println!("{}", person);
    };
    capture_string2();
}
 */
//////////////////////////////////////////
/////////////////////////////////////
/*
///
// the MOVE keyword- that will force the movement of ownership of values into a closure, even when the closure won't do so by default
fn main() {
    let first_name = String::from("Yeabsira");
    let last_name = String::from("Shimelis");
    //ORIGINALLY, this is Fn type - just takes immutable reference to the value. no ownership moves
    //let's use the move keyword now
    let capture_string = move || println!("{first_name} {last_name}");

    // println!("{} {}", first_name, last_name); //this will be an error b/c "move" keyword creates a ownership transfer

    //we can call it more than once as it is Fn type
    // WAIT - how the hell it is still Fn type?
    //b/c the datas passed to the closure are not consumed (don't move their values) (returned or something)
    capture_string();
    capture_string();
    capture_string();
}
 */

/*
////////////////////////////////
////////////////////////////////
//THE UNWRAP_OR_ELSE METHOD
/*
//inside the compiler
   pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }

*/
fn main() {
    let option = Option::Some("Salami");
    let food = option.unwrap_or_else(|| "Pizza");

    println!("{}", food);

    let option: Option<&str> = Option::None;
    // let food = option.unwrap_or_else(|| "Pizza");
    // println!("{}", food); //we will se the fallback here as the closure is executed for the None type
    //the above thing just gives fallback, so same as unwrap_or, innit?
    //let's do more with it(unwrap_or_else)
    let pizza_fan = false;
    let closure = || if pizza_fan { "Pizza" } else { "Hot Pockets" };
    let food = option.unwrap_or_else(closure);
    println!("{}", food);
}
 */
/*
////////////////////////////////////////////////
////////////////////////////////////////////////
//DEFINES A  METHOD THAT ACCEPTS A CLOSURE i (FnOnce trait)

use std::{io::stdin, process::exit};

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}

impl Vault {
    // fn unlock<F: FnOnce()-> String>(self, procedure: F) -> Option<String>
    // fn unlock(self, procedure: impl FnOnce() -> String) -> Option<String>
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Option::Some(self.treasure)
        } else {
            Option::None
        }
    }
}

fn main() {
    let vault = Vault {
        password: String::from("topsecret"),
        treasure: String::from("Gold"),
    };

    let mut user_input = String::new();
    println!("Please provide a password to crack the vault : ");
    let result = stdin().read_line(&mut user_input);

    match result {
        Result::Ok(_) => {
            println!("input correctly read");
        }
        Result::Err(error) => {
            println!("there was an error: {}", error);
            exit(0);
        }
    }

    user_input = user_input.trim().to_string();

    let hack = || user_input;

    let extraction = vault.unlock(hack);
    println!("{:?}", extraction);
}
 */

/*
///////////////////////////////////////
/////////////////////////////////////////
//THE STRING.RETAIN METHOD - it loops over each character of the string one at a time and excutes the closure once for each character passing in the character as an argument
//                           so used to keep or discard elements of the string
fn main() {
    let mut game_console = String::from("PalyStation");
    let mut deleted_characters = String::new();

    // let closure = |character| character != 'a';

    let closure = |character| {
        let is_not_a = character != 'a';
        if is_not_a {
            true //this is how the above one works behind the scenes. return true for the one to keep
        } else {
            deleted_characters.push(character);
            false
        }
    };
    game_console.retain(closure);

    println!("{}", game_console);
    println!("{}", deleted_characters);
}
 */

/*
/////////////////////////////////////////
//////////////////////////////////
//DEFINE A METHOD THAT ACCEPTS A CLOSURE II(fn mut trait)
#[derive(Debug)]
struct Location {
    name: String,
    treasures: u32,
}

//the lifetime of the map struct will end before the lifetime of the location
#[derive(Debug)]
struct Map<'a> {
    locations: &'a [Location], //this is a slice
}

impl<'a> Map<'a> {
    //our explore function is very versatile. b/c it only loops over the locations, we will decide what we want to do for each expore by the closure we passed to it
    fn explore<F>(&self, mut action: F)
    where
        F: FnMut(&Location),
    {
        let final_index = self.locations.len() - 1;
        let mut current_index = 0;
        while current_index <= final_index {
            let current_location = &self.locations[current_index];
            action(current_location);
            current_index += 1;
        }
    }
}

fn main() {
    let locations = [
        Location {
            name: String::from("Enhanted Forest "),
            treasures: 5,
        },
        Location {
            name: String::from("Mystic mountain"),
            treasures: 10,
        },
    ];

    let map = Map {
        locations: &locations,
    };

    //let's say we want our closure wants to calculate the total treasure
    let mut total_treasures = 0;
    let closure = |location: &Location| total_treasures += location.treasures;
    map.explore(closure);

    println!("total treasures collected :{}", total_treasures);

    //let's create another closure which takes the name of the locations and put them in a vector

    let mut location_names: Vec<String> = Vec::new();
    map.explore(|location| {
        location_names.push(location.name.clone());
    });

    println!("{:?}", location_names);
}
*/

/*
/////////////////////////////////////////////////////////
///////////////////////////////////////////////////////
//the Fn trait -closure captures values by immutable reference (read only) or doesn't capture any state at all
//             -closure can be invoked multiple times
//it is the strcites as we talked earlier, so it only accepts an Fn trait

fn execute_thrice<F>(procedure: F)
where
    F: Fn(),
{
    procedure();
    procedure();
    procedure();
}

fn bake_cake() {
    println!("hello chocolate");
}

fn main() {
    let closure = || println!("I am the boss");
    execute_thrice(closure);

    //PASSING IN A FUNCTION TO FN TRAIT PARAMETER
    execute_thrice(bake_cake);
    let option: Option<Vec<String>> = None;
    let collection = option.unwrap_or_else(Vec::new); //Vec:new is a function (a constructor one)
    println!("{:?}", collection);
}
*/
/////////////////////////////////////////////////////////
/////////////////////////////////////////////////
//CODING CHALLENGE
#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let final_index = self.items.len() - 1;
        let mut current_index = 0;

        while current_index <= final_index {
            operation(&mut self.items[current_index]);
            current_index = current_index + 1;
        }
    }

    fn checkout<F>(self, mut operation: F)
    where
        F: FnMut(ShoppingCart),
    {
        operation(self);
    }
}

fn main() {
    let cart_items = vec![
        SupermarketItem {
            name: String::from("APPLE"),
            price: 3.99,
        },
        SupermarketItem {
            name: String::from("BANANA"),
            price: 2.99,
        },
    ];

    let mut cart = ShoppingCart { items: cart_items };

    cart.traverse_items(|item| item.price = item.price * 0.85);

    cart.traverse_items(|item| item.name = item.name.to_lowercase());

    let mut total_price = 0.0;

    cart.checkout(|mut cart| {
        println!("{:#?}", cart);
        cart.traverse_items(|item| {
            total_price += item.price;
        });
    });

    println!("{}", total_price);
}
