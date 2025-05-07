//HASHMAPS - is a colection type that consists of key-value pairs
// they are like "objects in JavaScript"
//they are stored on the "HEAP" as their exact size cannot be predicted at compile time
// they can shrink or grow at the programs execution

use std::collections::HashMap;

/*fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();

    menu.insert(String::from("shiro-wot"), 20.5);
    menu.insert(String::from("Steak"), 29.9);
    menu.insert(String::from("Tuna"), 29.9);

    println!("{:?}", menu);

    let mut country_capitals: HashMap<&str, &str> = HashMap::new();
    country_capitals.insert("Ethiopia", "Addis Ababa");
    country_capitals.insert("France", "Paris");
    country_capitals.insert("Germany", "Berlin");

    println!("{:?}", country_capitals);
}
 */

///////////////////
/*
//THE REMOVE METHOD - deletes a key-value pair from the hasmap using the key
//and it returns an OPTION ENUM.
fn main() {
    //we can initiate using from as well
    //it accepts array of tuple elements. each tuple is going to store 2 elementss within it,
    //      and the first is going to be a "key" and the second is going to be a correxponding "value"
    let data = [("Yeabsira", 4), ("Bobby", 7), ("Ben", 6)];
    let mut years_at_company = HashMap::from(data);

    println!("{:?}", years_at_company);

    let ben = years_at_company.remove("Ben");
    println!("{:?}", ben); //this will give us Some(6)
    println!("{:?}", ben.unwrap()); //but using match key word will be better

    println!("{:?}", years_at_company);
}
*/

/*//HASHMAPS AND OWNERSHIP
// the variable name is the ownerof the hashmap and the the hashmap is the owner of the data that is stored both in keys and values
fn main() {
    // let mut coffee_parings: HashMap<&String, &String> = HashMap::new();
    let mut coffee_parings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    //the above variables are heap data. so they transfer the ownership to the hashmap
    // coffee_parings.insert(drink, milk);
    // println!("{:?}", coffee_parings);
    // println!("{}", coffee_parings.len()); //to find the length of the hashmap / the count of the key-value pairs

    // println!("{}", drink); this will be invalid b/c the drink variable is invalid here as it transfers the ownership to the hashmap

    //inorder not to transfer ownership from the variables to the hashmap, we can clone them or use references
    coffee_parings.insert(&drink, &milk);
    //here we use &String, so rust expects &Strings for all the next key value pairs. but
    //  as we learned in deref coertion &str is a flexible one. so making it &str enable the hashmap to accept
    // both &str and &String
    coffee_parings.insert("Flat White", "Almond Milk");

    println!("{drink}, {milk}"); // this one is valid as we use references instead of move of ownership

    println!("{:#?}", coffee_parings);
}
 */

/*///////////////////
///////////////
//ACCESSING ELEMENTS
fn main() {
    let mut coffee_parings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_parings.insert(&drink, &milk);
    coffee_parings.insert("Flat White", "Almond Milk");

    //this is one approach to access an elements from a hashmap
    let value = coffee_parings["Flat White"];
    println!("{}", value);
    //this method is fine, but if the key doesnot exist it will panic at runtime
    // let value2 = coffee_parings["Cappuccino"]; //this will panic the program at runtime

    //using GET method will return an option enum with immutable reference of the associated value
    // here we got a reference to a reference.
    let value: Option<&&str> = coffee_parings.get("Cappuccino");
    println!("{:?}", value);

    //copied - down the reference to one level
    let value = coffee_parings
        .get("Cappuccino")
        .copied()
        .unwrap_or("Unknown milk");
    println!("{:#?}", value);
}
 */

/*
/////////////////////
//OVERWRITTING A VALUE WITH AN EXISTING VALUE  - if we try to add a key-value pair in which a key is alreasdy existed in the hashmap
//                                                 rust will overwrite the old one
fn main() {
    let mut coffee_parings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_parings.insert(&drink, &milk);
    coffee_parings.insert("Flat White", "Almond Milk");

    println!("{:?}", coffee_parings); //before overwrite

    //this will overwrite the old one
    coffee_parings.insert("Latte", "Pistachio Milk");
    println!("{:?}", coffee_parings);
}
 */

/*
////////////////////
//THE ENTRY METHOD - accepts a hashmap key and returns an enum "ENTRY" which has 2 variants
//                      "occupied and vacant"
//after entry method if the varaint is vacant (the key doesn't exist) we can use ".or_insert method"
//so really useful to avoid overwriting. b/c only creates a new value with that key, if it is not exist
/*
The or_insert method on Entry is defined to return a mutable reference to the value for the
corresponding Entry key if that key exists, and if not, it inserts the parameter as the new
value for this key and returns a mutable reference to the new value. - "FROM RUST BOOK"

//PROOF OF IT GIVING US MUTABLE REFRENCE TO ITS VALUE
Updating a Value Based on the Old Value
Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, Listing 8-25 shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0.


    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
{"hello": 1, "wonderful": 1, "world": 2}
*/
fn main() {
    let mut coffee_parings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");

    coffee_parings.insert(&drink, &milk);
    coffee_parings.insert("Flat White", "Almond Milk");

    coffee_parings.entry("Latte").or_insert("Pastachio Milk"); //Latte exist so nothing will happen
    println!("{:#?}", coffee_parings);

    coffee_parings
        .entry("Cappuccino")
        .or_insert("Pastachio Milk"); //Capupucino doesn't exist so will get added
    println!("{:#?}", coffee_parings);
}
 */

use std::collections::hash_set;
use std::collections::HashSet;
use std::vec;
/////////////////////////////////////////
///////////////////////////////
/*
//THE HASHSET - it is a distinct type, but it uses a hashmap behind the scenes to implement its functionality
//            -it is often called a set in other languages
//hashset - is a collection type that stores unique values
//        -solves the problem of duplication
fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:#?}", concert_queue);

    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:#?}", concert_queue);
    println!("{}", concert_queue.len());

    //this will NOT panic the program at run time. rust just ignores it as it knows it is a duplicate
    concert_queue.insert("Molly");
    println!("{:#?}", concert_queue);

    //remove()- to remove element from the hashset and returns a boolean
    println!("{}", concert_queue.remove("Megan")); //true - b/c found and removed
    println!("{}", concert_queue.remove("Megan")); // false - not found
    println!("{:#?}", concert_queue);

    //contains() method - to check if the element is found - return boolean
    println!("{}", concert_queue.contains("Molly"));

    //get method - gives us an OPTION enum with immutable reference of the associated value - return reference, otherwise there will be a move of ownership from the collection type to the option enum
    println!("{:?}", concert_queue.get("Molly"));
}

/*
WE CAN USE OTHER TYPES IN THE HASHSET LIKE STRUCTS
use std::collections::HashSet;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    power: usize,
}

let mut vikings = HashSet::new();

vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
vikings.insert(Viking { name: "Harald".to_string(), power: 8 });

// Use derived implementation to print the vikings.
for x in &vikings {
    println!("{x:?}");
} - FROM THE RUST DOCUMENTATION
*/
 */
/*
///////////////////
//////////////
//HASHSET OPERATIONS
/*
 in Rust, set operations on HashSet—such as union, intersection, difference, and symmetric_difference—return lazy ITERATORS.
*/
fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();

    concert_queue.insert("Yeabsira");
    concert_queue.insert("Melissa");

    movie_queue.insert("Yeabsira");
    movie_queue.insert("Phil");
    movie_queue.insert("Melissa");

    //all the methods returns their own special structs

    //UNION - combine them, no duplication  - takes intersections only once
    //        return a special struct called "UNION"
    println!("{:#?}", concert_queue.union(&movie_queue));
    println!("{:#?}", movie_queue.union(&concert_queue));

    //difference - give us only the elements the first hashmap has
    println!("{:#?}", concert_queue.difference(&movie_queue));
    println!("{:#?}", movie_queue.difference(&concert_queue));

    //symmetric difference - coombine elements in either one of them. intersections will not be included = UNION- INTERSECTION
    println!("{:#?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:#?}", movie_queue.symmetric_difference(&concert_queue));

    //is_disjoint() -gives "true" if they no values in common
    println!("{:#?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:#?}", movie_queue.is_disjoint(&concert_queue));

    //subset - return "true" if the first hashmap is the subset of the second / the argument
    println!("{:#?}", concert_queue.is_subset(&movie_queue));
    println!("{:#?}", movie_queue.is_subset(&concert_queue));

    // super set - is the exact opposite of the subset
    println!("{:#?}", concert_queue.is_superset(&movie_queue));
    println!("{:#?}", movie_queue.is_superset(&concert_queue));
}
 */

///////////////////////
////////////////////

//CODING CHALLENGE
fn main() {
    let data = [
        ("ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
    ];
    let mut sauces_to_meals: HashMap<&str, Vec<&str>> = HashMap::from(data);

    sauces_to_meals.insert("Mustard", vec!["Hot dog", "Burgers", "Pretezels"]);

    let mayonnaise = sauces_to_meals.remove("Mayonnaise");
    println!("{:#?}", mayonnaise.expect("mayonnaise not found"));

    let mustard = sauces_to_meals.get("Mustard");

    match mustard {
        Option::Some(_) => println!("{:#?}", mustard.unwrap()),
        Option::None => println!("item with mustard key not found"),
    };

    sauces_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("{:#?}", sauces_to_meals);
}
