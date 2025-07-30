/*
//ITERATORS

to iterate means to perform repeatedly. iteration is repeating the same
operation on a sequence of items, one item at a time
*/

/*
//MANUAL ITERATION - length calculation, stoping condition and different things are calculated imperatively. so prone to error
fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let mut current_index = 0;
    let final_index = numbers.len() - 1;

    // loop {
    //     if current_index > final_index {
    //         break;
    //     }
    //     println!("{}", numbers[current_index]);
    //     current_index += 1;
    // }

    // while current_index <= final_index {
    //     println!("{}", numbers[current_index]);
    //     current_index += 1;
    // }

    for number in numbers {
        println!("{}", number);
    }
}
 */
///////////////////////////////////////
////////////////////////////////
// ITERATOR AND INTOITERATOR TRAITS

// ITERATOR TRAIT:
// - Any type that implements the `Iterator` trait can be iterated over.
// - It has around 76 methods, most with default implementations.
// - The only required method to implement is `next()`.
// - The `next()` method returns `Some(item)` until the iteration ends, then it returns `None`.

// INTOITERATOR TRAIT:
// - The `IntoIterator` trait converts a type into an iterator.
// - It defines the `into_iter()` method, which consumes the type and returns an iterator.
// - It's automatically implemented for types like Vec<T>, arrays, and references to them.
// - For example, `vec.into_iter()` moves ownership, while `&vec` or `vec.iter()` yields references.

/*
use std::collections::HashMap;

fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.into_iter(); // my_iterator is now an iterator over the vector

    let my_vector = vec![false, true, false];
    let my_iterator = my_vector.into_iter();

    let mut my_hashmap = HashMap::new();
    my_hashmap.insert("CBS", 2);
}
 */

/*
///////////////////////////////
/////////////////////////////
//EXHAUSTING THE ITERATOR -
// EXHAUSTING THE ITERATOR -
// - When you call `.next()` repeatedly on an iterator, it returns elements one by one.
// - Once all items have been returned, `.next()` returns `None`.
// - This process is called "exhausting the iterator".
// - After an iterator is exhausted, further calls to `.next()` will keep returning `None`.

fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);

    println!("{:?}", my_iterator.next());
    println!("{:?}", my_iterator);
}
 */
///////////////////////////
////////////////////////////////
/*
// THE FOR LOOP WITH ITERATOR -
// - The `for` loop in Rust automatically calls `.into_iter()` on the value being looped over.
// - In this case, we explicitly create an iterator using `into_iter()`.
// - The loop then takes ownership of the iterator and repeatedly calls `.next()` behind the scenes.
// - Each value yielded by the iterator is assigned to `number` in each iteration.

fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42]; // Create a vector
    let my_iterator = my_vector.into_iter(); // Convert vector into an iterator (moves ownership)

    // Iterate over the items in the iterator and print them
    //but as we know we don't have to have iterator b/c the for loop calls `.into_iter()` method by default. that is why we loop over iterables using for loop
    for number in my_iterator {
        println!("{}", number); // Prints: 4 8 15 16 23 42 (each on a new line)
    }
//similar as the above one - the above one is behind the scene thing
    for number in my_vector{
      println!("{}", number);
    }

    // println!("{:?}", my_iterator); //this is not valid b/c the for loop takes the ownership of "my_iterator" iterator
    // println!("{:?}", my_vector); //this is not valid b/c the "my_iterator" takes ownership  of the vector
}
 */
/*
/////////////////////////////////////
/////////////////////////////////////
// THE ITER METHOD - will create an iterator that yields immutable references to the collection's of elements
//                 -so creates iterator of references (no the original values)
fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter();

    for number in my_iterator {
        println!("{}", number);
    }

    //similar as the above one - so this calls "iter()" method behind the scenes
    for number in &my_vector {
        println!("{}", number);
    }

    // println!("{:?}", my_iterator);this is not valid b/c the for loops takes the ownership of the iterators. not matter it is iterator of values or references
    println!("{:?}", my_vector); //valid b/c iter method creates iterator of references
}*/

////////////////////////////////////
////////////////////////////////
//THE ITER_MUT METHOD - will create an iterator that yields mutable reference to the collection's element
/*
fn main() {
    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    // let iterator = flavors.iter_mut();

    // for flavor in iterator {
    //     flavor.push_str(" Ice Cream");
    // }

    //similar as the above one - so this calls "iter_mut()" method behind the scenes
    for flavor in &mut flavors {
        flavor.push_str(" Ice Cream"); //as we know, RUST has auto-deref for method calls
    }
    println!("{:?}", flavors); // stil valid b/c the iterator just takes a mutable reference (no move of ownership)

    let mut school_grades = [85, 90, 72, 92];

    for grade in &mut school_grades {
        *grade -= 2; // we have to dereference b/c it is not a method call (just our mathematical computation)
    }
    println!("{:?}", school_grades);
}
 */
/*
so SUMMARY
 OWNERSHIP
 for value in collection
 for value in collection.into_iter()

 IMMUTABLE REFERENCES
 for value in &collection
 for value in collection.iter()

 MUTABLE REFERENCES
 for value in &mut collection
 for value in collection.iter_mut()
*/

/*
///////////////////////////////////////////
/////////////////////////////////////////
//HASHMAP ITERATION

use std::collections::HashMap;
fn main() {
    let mut todos = HashMap::new();
    todos.insert("pick up groceries", false);
    todos.insert("study Rust", true);
    todos.insert("sleep", false);

    // means calling into_iter() method
    // for (todo, completion_status) in todos {
    //     println!("Task: {}. Complete:{}", todo, completion_status);
    // }
    // println!("{:?}", todos); //not valid. b/c ownership is moved to the loop

    // means calling iter() method
    // for (todo, completion_status) in &todos {
    //     println!("Task: {}. Complete:{}", todo, completion_status);
    // }
    // println!("{:?}", todos); //valid. b/c we create immutable reference

    // means calling iter_mut() method
    for (_, completion_status) in &mut todos {
        *completion_status = true;
    }
    println!("{:?}", todos); //valid. b/c we create mutable reference
}
 */

/*
//////////////////////////////////
////////////////////////////////
//STRING ITERATION
//the chars method returns an iterator of the unicode characters
// the bytes method returns an iterator of the raw bytes
fn main() {
    let seafood = "Oyster 🍕";

    // for byte in seafood.bytes() {
    //     print!("{}/", byte);
    // }
    // println!("{}", seafood) //valid

    // for character in seafood.chars() {
    //     print!("{}/", character);
    // }
    // println!();
    // println!("{}", seafood); //valid

    // the iterator from the bytes is a valid type and has different methods
    println!("{}", seafood.bytes().len()); //for this case 10. b/c the emogi has 4 bytes

    //so the iterator from the chars is
    println!("{}", seafood.chars().count());
}
 */

/*
/////////////////////////////////
///////////////////////////////
//solve a problem with iteration

use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        //these methods will add the word with 0 if it wasn't in the hashmap before
        //  and if exists it returns a mutable reference to the value
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

fn count_chars(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    for word in words {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }
    }

    counts
}

fn main() {
    println!(
        "{:#?}",
        count_words("Sally sells sea shells by the sea shore")
    );

    println!(
        "{:#?}",
        count_chars("Sally sells sea shells by the sea shore")
    );
}
 */

/*
////////////////////////////////////////
////////////////////////////////////////
//THE FOREACH METHOD - applies a consistent operation to every iterator element. it accepts a closure

use std::collections::HashMap;

fn count_chars(text: &str) -> HashMap<char, u32> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();

    words.for_each(|word| {
        word.chars().for_each(|character| {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        });
    });

    counts
}

fn main() {
    println!(
        "{:#?}",
        count_chars("Sally sells sea shells by the sea shore")
    );
}
 */
/////////////////////////////////////////
////////////////////////////////////////
//ADAPTER METHODS - IS ONE THAT  TRANSFORMS AN ITERATOR INTO ANOTHER ITERATOR BASED ON SOME LOGIC
// USED WHEN WE WANT TO CHANGE ITERATOR TO ANOTHER DESIRED BRAND NEW ITERATOR.

/*
//the map method - the most popular adapter method
//                 appliess a closure to each item in an iterator to arrive at a new iterator of values
fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    // let my_iterator = numbers.iter();
    // let squares = my_iterator.map(|number: &i32| number.pow(2));

    // println!("{:#?}", squares); //this won't see us the vec of the squares. this is just the recipe
    // so inorder to get the square we have to exhaust this iterator (using for loop)

    // for number in squares {
    //     println!("square : {}", number)
    // }
    // println!("{:?}", squares); //this brand new iterator is not valid here, b/c it is exhausted

    for number in numbers.into_iter().map(|number: i32| number.pow(2)) {
        println!("square : {}", number)
    }
}
 */

/*
//THE COLLECT METHOD - exhusta the iterator and gathers the resulting values in a new collection type
//                 you can think it as using a for loop an iterator and push in to a new vector

use std::collections::HashSet; //a collection contains a unique type

fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    //type annotation needed. b/c the we have to specify that to what type we want to collect
    let squares: Vec<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    //or put underscore in place of the generic to let rust do its best. but the collection type should be annotated explicitly
    // let squares: Vec<_> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    //or use a turbofish operator
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<Vec<i32>>();

    println!("{:#?}", squares);

    let squares: HashSet<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    println!("{:#?}", squares);
}
 */
/*
////////////////////////////////
////////////////////////////////
//the map method 2

fn main() {
    let names = [
        String::from("Jimmy"),
        String::from("Cleveland"),
        String::from("Boris"),
    ];

    //this is chaining methods
    let name_lengths: Vec<usize> = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect();

    println!("{:?}", name_lengths);
}
 */

/*
////////////////////////////////////////
////////////////////////////////////////
//THE FILTER AND FIND METHODS
//filter method - extracts a subset of values tht satisfy a condition. pass a closure that returns true for the elemenets to keep and false for the elements to exclude
// find method - find the first element the closure returns true. and returns an option enum
//rfind method - find the last element
fn main() {
    let numbers = [10, 13, 23, 2, 9, 6];

    // let evens: Vec<i32> = numbers
    //     .into_iter()
    //     .filter(|number| number % 2 == 0)
    //     .collect();

    //copied - use to create the copy of the datas. b/c we want the datas not the references in the variable
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .copied()
        .collect();

    println!("{:#?}", evens);
    println!("{:#?}", numbers);

    let first_even = numbers.into_iter().find(|number| number % 2 == 0);
    println!("{:?}", first_even);

    let first_odd = numbers.into_iter().find(|number| number % 2 != 0);
    println!("{:?}", first_odd);

    let nothing = numbers.into_iter().find(|number| *number > 100);
    println!("{:?}", nothing);

    let last_even = numbers.into_iter().rfind(|number| number % 2 == 0);
    println!("{:?}", last_even);

    let last_odd = numbers.into_iter().rfind(|number| number % 2 != 0);
    println!("{:?}", last_odd);
}
 */

/*
/////////////////////////////////////////////////
///////////////////////////////////////////////
//THE FILTER AND FIND METHODS 2 - with structs

#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrmmingTutorials,
}

#[derive(Debug, PartialEq, Eq)]
struct TvChannel {
    name: String,
    channel_type: ChannelType,
}

fn main() {
    let channels = [
        TvChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TvChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrmmingTutorials,
        },
        TvChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TvChannel {
            name: String::from("RustTv"),
            channel_type: ChannelType::ProgrmmingTutorials,
        },
    ];

    let good_channels: Vec<&TvChannel> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials)
        .collect();

    println!("good channels : {:#?}", good_channels);

    let good_channels_names: Vec<String> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials)
        .map(|channel| channel.name.clone())
        .collect();

    println!("good channels string : {:#?}", good_channels_names);

    //find

    let find_good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials);

    println!("the first good channel{:?}", find_good_channel);

    match find_good_channel {
        Option::Some(channel) => println!("the great choice to watch {:?}", channel),
        Option::None => println!("there was no RUST programming on the TV"),
    }
}
 */
/*
/////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////
//THE ANY AND ALL METHODS - to check if all the elements satify the conition or if atleast one of them satisfies the condition
//                        -so we get a boolean
#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrmmingTutorials,
}

#[derive(Debug, PartialEq, Eq)]
struct TvChannel {
    name: String,
    channel_type: ChannelType,
}

fn main() {
    let channels = [
        TvChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TvChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrmmingTutorials,
        },
        TvChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TvChannel {
            name: String::from("RustTv"),
            channel_type: ChannelType::ProgrmmingTutorials,
        },
    ];

    // //ALL method
    // //we can do this like this- by filtering the channels we want and compare the length of the
    // //   filtered with the original one. if equal -> all satisfied

    // let good_channels: Vec<&TvChannel> = channels
    //     .iter()
    //     .filter(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials)
    //     .collect();
    // println!("{}", good_channels.len() == channels.len());

    // //ANY
    // //if the find method returns Some variant of the option enum
    // let good_channel = channels
    //     .iter()
    //     .find(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials);
    // println!("{}", good_channel.is_some());

    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials);
    println!("{}", all_are_rust);

    let any_are_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrmmingTutorials);
    println!("{}", any_are_rust);
}
 */

/*
//////////////////////////////////////////
/////////////////////////////////////////
//THE CLONED METHOD
/*
the COPIED method converts an iterator from storing &T elements to T elements
   it makes a copy of each T element
   the T data type must implement the Copy trait

the CLONED method similarly converts an iterator from storing &T elements to T elements.
   it makes a clone of each T element
   the T data type must implement the Clone trait
   so primarly used for heap based data
*/

fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Match"),
    ];

    // //we can get the same result using map - but it is a load of work - it is the one RUST do behind the scenes for the cloned method
    // let more_teas: Vec<String> = teas.iter().map(|tea| tea.clone()).collect();
    // println!("{:?}", more_teas);

    //iter gives us iterator of references, but we get iterator of the data them selves (by cloning the original data) by calling clone method before collecting them in to a vector
    let more_teas: Vec<String> = teas.iter().cloned().collect();
    println!("{:?}", more_teas);

    //let's try to clone some data using filter first
    //only clone the teas that contains "Hot" in their name
    let hot_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();
    println!("{:?}", hot_teas);

    //what abt using the clone method first in the above EG. it will work,
    // but not efficient b/c the clone method is expensive computation. b/c saving to memory occurs . imagine working with a large dataset.
}
 */

/*
///////////////////////////////////////////////
/////////////////////////////////////////////
//THE FILTER_MAP METHOD - both filters and transforms a subset of elements from an iterator
//                 this method returns an "OPTION ENUM"
fn main() {
    let stocks = ["nvda", "", "aapl", "msft", "", "goog"]; //let's mimic this as bad source of data

    //we can use them separately. but it is a lot of work
    let capitalized_stocks: Vec<String> = stocks
        .iter()
        .filter(|stock| !stock.is_empty())
        .map(|stock| stock.to_uppercase())
        .collect();
    println!("{:?}", capitalized_stocks);

    //let's do them with a single method call
    let capitalized_stocks = stocks
        .iter()
        .filter_map(|stock| {
            if stock.is_empty() {
                None
            } else {
                Some(stock.to_uppercase())
            }
        })
        .collect::<Vec<String>>();
    println!("{:?}", capitalized_stocks);
}
 */
/*
////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
//THE FLATTEN METHOD - returns iterator that flattens nested data structures

fn main() {
    let spreadsheet = vec![[100, 200, 300], [132, 456, 789], [987, 654, 321]];

    let values = spreadsheet.into_iter().flatten().collect::<Vec<i32>>();
    println!("{values:?}");
}
 */
/*
//////////////////////////////////////////////////////
///////////////////////////////////////////////////////
//THE FLAT_MAP METHOD - combines the idea of flatten and map methods
fn main() {
    let attendees = [
        "Bob, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    //we can get the same result with those 2 separate methods
    let attendees: Vec<&str> = attendees
        .iter()
        .map(|group| group.split(", "))
        .flatten()
        .collect();
    println!("{:?}", attendees);

    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();
    println!("{:?}", attendees);
}
 */

/*
//////////////////////////////////////////////
//////////////////////////////////////////
//THE ENUMERATE METHOD -transforms an iterator so that it yields the index position along with the current element
//                    gives a tuple of the value and the index in a tuple for each.
fn main() {
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Piers", "John", "Dan"];
    //let's say the index position which can be divided with 3 without remainder

    // let winners = applicants
    //     .into_iter()
    //     .enumerate()
    //     .filter(|(index, _applicant)| index % 3 == 0)
    //     .map(|(_index, applicant)| applicant)
    //     .collect::<Vec<&str>>();

    //using filter_map
    let winners = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    println!("{:#?}", winners);
}
 */

/*
////////////////////////////////////////////
//////////////////////////////////////////
//THE PARTITION METHOD - groups and returns the values for which the closure returns true
//                         and for which the closure returns false
//                     -so it returns for both in separate groups.
//                 *and returns a tuple containing both groups(we choose any collection we want for each to contain the groups). not an iterator like the other methods like filter
fn main() {
    let numbers = [4, 8, 15, 16, 23, 42];

    // let groups: (Vec<i32>, Vec<i32>) = numbers.into_iter().partition(|number| number % 2 == 0);
    // println!("{:?}", groups);
    // println!("{:?}", groups.0);
    // println!("{:?}", groups.1);

    let (evens, odds): (Vec<i32>, Vec<i32>) =
        numbers.into_iter().partition(|number| number % 2 == 0);
    println!("{:?}", evens);
    println!("{:?}", odds);
}
 */

/*
////////////////////////////////////////
//////////////////////////////////////////
//THE ZIP METHOD - combines 2 iterators together based on their elements having the same index positions.
//             so we can essentially merge two iterators together into a brand new iterator
//             so a tuple at each shared positions in the iterators
//     if one of the iterator is longer, the overflows will just be ignored
fn main() {
    let first_names = ["Casey", "Robert", "Cargo", "Dan"]; //so "Dan" will be ignored
    let last_name = ["Johnson", "Smith", "Rustman"];

    let full_names = first_names.iter().zip(last_name);

    for (first_name, last_name) in full_names {
        println!("{} {}", first_name, last_name);
    }

    //let's put each persons full name by combining the first and last name into a vector
    let complete_names: Vec<String> = first_names
        .iter()
        .zip(last_name)
        .map(|(first_name, last_name)| format!("{first_name} {last_name}"))
        .collect();

    println!("{:#?}", complete_names)
}
 */

/*
////////////////////////////////////////////////
////////////////////////////////////////////////
//THE FOLD METHOD - exhausts an iterator to build up and produce a single value at the end of iteration
//             it is like reduce method(accumulator) in JavaScript

use std::collections::HashMap;

struct SupportStaff {
    day: String,
    employee: String,
}

fn main() {
    let earnings = [4, 7, 9, 13];
    //  0 -is the starting value for the accmulator
    let sum = earnings
        .into_iter()
        .fold(0, |total, current| total + current);

    println!("{}", sum);

    let week = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Brian"),
        },
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Cam"),
        },
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Walter"),
        },
    ];

    let map = week.into_iter().fold(HashMap::new(), |mut data, entry| {
        data.insert(entry.day, entry.employee);
        data //return updated data(itself) for the next iteration
    });

    println!("{:?}", map);
}
 */

/*
///////////////////////////////////////
///////////////////////////////////////
//THE REDUCE METHOD - the reduce method is similar to fold but it supplies the iterator's
//                           first element as the starter value
//                          returns an option enum - this time the iterator at least needs to have one element.
//                            but in the fold method we return the exact data because we define the initial value selves and it will be returned if the iterator is empty
fn main() {
    let earnings = [4, 7, 9, 13];

    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{:?}", sum);

    let address_portions = [
        String::from("123 Elm Street"),
        String::from("Suburbia"),
        String::from("New Jersy"),
    ];

    let address = address_portions
        .into_iter()
        .reduce(|accumulator, portion| format!("{}, {}", accumulator, portion));
    println!("{:?}", address);
}
*/

/*
/////////////////////////////////////////////////////
//we have been seeing ADAPTER - method which create new iterators from existing iterators
//now we will see CONSUMERS - which exhaust an iterator
// THE SUM, PRODUCT, MAX, MIN, COUNT METHODS
fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];

    let total: i32 = numbers.iter().sum();
    println!("{}", total);

    let product: i32 = numbers.iter().product();
    println!("{}", product);

    let max = numbers.iter().max().unwrap(); //returns an option enum(so we can use unwrap. but it causes an error for the None variant)
    println!("{}", max);

    let min = numbers.iter().min().unwrap(); //returns an option enum(so we can use unwrap. but it causes an error for the None variant)
    println!("{}", min);

    let count = numbers.iter().count(); //give us the lenght of the iterator
    println!("{}", count);

    //max and min methods don' work on floats, b/c the float doesnot implement the "ORD(order)" trait.
    // b/c "NAN(not a number) - result of incorrect maths computations" is also a float and how can we find the max or min with comparing to someking of nonsense

    //so to sum floats eg. (needs filtering of NAN)
    let numbers = vec![4.6, 8.8, 0.0 / 0.0, 6.2, f64::NAN];
    let total = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .fold(0.0, |total, current| total + current);
    println!("{}", total);

    //and to find max for floats
    let max = numbers
        .iter()
        .filter(|number| !number.is_nan())
        .copied()
        .reduce(|accumulator, current| accumulator.max(current));
    println!("{:?}", max);
}
*/
/*
///////////////////////////////////////////////////
////////////////////////////////////////////////////
//THE LAST, NTH, NTH_BACK AND POSITION METHOD - to target_index element based on itscurrent position
//                  they return OPTION ENUMS
fn main() {
    let performers = ["Rustful Five", " Rust in Peace", "Rustin Bieber"];
    let last = performers.into_iter().last().unwrap(); //return the option enum. we can call unwrap on it so.
    println!("{}", last);

    let second = performers.into_iter().nth(1).unwrap(); //work based on index
    println!("{}", second);

    let second_to_last = performers.into_iter().nth_back(1).unwrap(); //work based on index
    println!("{}", second_to_last);

    //POSITION = to find the position(index) of an elements if it exists in the iterator
    let target_index = performers
        .iter()
        .position(|element| *element == "Rustin Bieber")
        .unwrap();
    println!("{}", target_index);
}
 */
/*
///////////////////////////////////////////////////////////
/////////////////////////////////////////////////////
//THE TAKE, REV, SKIP AN STEP_BY METHODS - sometimes we need to skip items in iterations
fn main() {
    let fifty_numbers = 1..50; //range type

    //take only the first 15 iterators
    for number in fifty_numbers.clone().take(15) {
        print!("{number}/");
    }
    println!();
    //take only 15 iterators, but starting from the last one
    for number in fifty_numbers.clone().rev().take(15) {
        print!("{number}/");
    }
    println!();

    //skip the first 5 elements
    for number in fifty_numbers.clone().skip(5).take(15) {
        print!("{number}/");
    }
    println!();

    for number in fifty_numbers.step_by(2) {
        print!("{number}/");
    }
}
 */

/*
////////////////////////////////////////////////
//////////////////////////////////////////
//THE SORT AND SORT_BY_KEY METHODS

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

fn main() {
    let mut points = [3, 8, 1, 11, 5];
    println!("{}", points.is_sorted());

    points.sort();
    println!("{:?}", points);

    points.reverse();
    println!("{:?}", points);

    //sort by letters. but capital letters always considered smaller (depending on ASCII code) - Z < a
    let mut exercises = ["squat", "bench", "deadlift"];
    exercises.sort();
    println!("{:?}", exercises);

    let mobil = GasStation {
        snack_count: 100,
        manager: String::from("Meg mobil"),
        employee_count: 3,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 2,
    };

    let mut stops = [mobil, exxon, shell];
    println!("{:#?}", stops);

    stops.sort_by_key(|station| station.snack_count); //we return the field in which we want to compute the comparison from the closure
    println!("{:#?}", stops);

    //if we reverse order
    stops.sort_by_key(|station| -(station.employee_count as i32)); //it type is unsigned32 in the struct,so we have to typecast to something which can be signed
    println!("{:#?}", stops);
}
 */
/*
//////////////////////////////////////////////////////
/////////////////////////////////////////////////////
//THE LINES METHOD - returns an iterator of the string's individual lines
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    /*
        fs::read_to_string("story.txt") tries to read a file.
    This returns a Result type: Result<String, std::io::Error>.
    If it succeeds, it gives you the file contents.
    If it fails (e.g., file not found), it gives an error.
    The ? means:
    “If this fails, return the error immediately from the function.”
    So, instead of writing this:
         */

    let contents = fs::read_to_string("story.txt")?;

    for line in contents.lines() {
        println!("{line}")
    }

    Ok(())
}
 */
/*
///////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////
//COLLECTING COMMAND LINE ARGUMENTS
/*
command line arguments are values passed into the program from the terminal/ command prompt
*/

use std::env; // Provides functions for interacting with the program's environment

fn main() {
    /*
    `env::args()` returns an iterator over the command-line arguments.
    - The first argument is always the name or path of the executable.
    - The following arguments are any additional values passed by the user.
    */
    let args = env::args();

    for arg in args {
        println!("{}", arg);
    }
}
 */
/*
/*
VIDEO PLAYER APPLICATION - in which you open videos using commandline
COMMAND-LINE ARGUMENTS

-VIDEO FILE NAME
-SUBTITLES - boolean input (if they want to enable subtitle )
-HIGH DEFINATION - boolean input (if they want to enable high defination)
*/
use std::env;
use std::process;

#[derive(Debug)]
struct Settings {
    video_file: String,
    subtitles: bool,
    high_defination: bool,
}

fn collect_settings() -> Settings {
    //skip the first item in the iterator - which is the program name currently executing. we don't need that
    //target/debug/iterators rust.mp4 true false - it will be something like this
    let mut args = env::args().skip(1).take(3); //skip the executing file name

    // args.next() gets the first item from that iterator, which is like "rust.mp4".
    let video_file = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified");
        process::exit(0);
    });

    //parse strings to booleans
    let mut settings = args.map(|setting| setting.parse::<bool>().unwrap_or(false));

    //get the parsed boolean or give defaults for which are not given from the user
    let subtitles = settings.next().unwrap_or(false);
    let high_defination = settings.next().unwrap_or(false);

    Settings {
        video_file,
        subtitles,
        high_defination,
    }
}
fn main() {
    let settings = collect_settings();

    println!("{:#?}", settings);
}
*/

/*
////////////////////////////////////////////
////////////////////////////////////////////
//READING DIRECTORY

use std::{fs, io::Result};
fn main() -> Result<()> {
    //? - propagate error if the read_dir is unsuccessfull (returns the error variant)
    for entry_result in fs::read_dir("./")? {
        //we can use if let
        // if let Ok(entry) = entry_result {
        //     println!("{:?}", entry.path())
        // }

        //we can use ? (if exist(Ok variant) execute the next lines, if not propagate the error )
        let entry = entry_result?;

        //distiguish the files and folders and read the contents of the files
        let entry_name = entry.path();
        let metadata = fs::metadata(&entry_name)?;
        if metadata.is_file() {
            println!("{:?}\n======================", entry_name);
            let contents = fs::read_to_string(&entry_name)?;
            println!("{}", contents);
        }
    }
    Ok(())
}
 */

/*
///////////////////////////////////////////////////////
//////////////////////////////////////////////////////
/*
the FromIterator trait
   The FromIterator trait allows you to create a collection from an iterator.

   It defines how to turn something like an Iterator<Item = T> into a concrete collection (like Vec<T>, HashSet<T>, String, etc.).

   "FromIterator lets types define how to build themselves from an iterator — enabling .collect() to work."
*/

use std::collections::HashSet;

#[derive(Debug)]
struct Playlist {
    songs: Vec<String>,
    users: HashSet<String>,
}

impl FromIterator<(String, String)> for Playlist {
    fn from_iter<T: IntoIterator<Item = (String, String)>>(iter: T) -> Self {
        let mut songs = Vec::new();
        let mut users = HashSet::new();
        for (song, user) in iter {
            songs.push(song);
            users.insert(user);
        }
        Self { songs, users }
    }
}

fn main() {
    let fifty_numbers = 1..50; //this creates an iterator

    let results = Vec::from_iter(fifty_numbers.clone());
    println!("{:?}", results);

    // // the collect method calls the "from_iter" method behind the scenes
    // let results = fifty_numbers.clone().collect::<Vec<i32>>();
    // println!("{:?}", results);

    let unique_set: HashSet<_> = HashSet::from_iter(fifty_numbers.clone());
    println!("{:?}", unique_set);

    // //using collect method;
    // let unique_set = fifty_numbers.clone().collect::<HashSet<_>>();
    // println!("{:?}", unique_set);

    let chars = ['H', 'E', 'L', 'L', 'O'];
    // let greeting = String::from_iter(chars.iter());
    //not neccessaty to call iter() on the "chars" - rust creates the iterator by itself automatically
    let greeting = String::from_iter(chars);
    println!("{}", greeting);

    let songs = [
        (String::from("I Rust go on"), String::from("Bob")),
        (String::from("A Rust of wind"), String::from("Bob")),
        (String::from("A Rustworthy man"), String::from("Sheila")),
    ];

    // let playlist = Playlist::from_iter(songs);
    // println!("{:?}", playlist);

    //we implement the from_iter trait, so that we can use the collect method
    let playlist = songs.into_iter().collect::<Playlist>();
    println!("{:?}", playlist);
}
 */
/*
////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
//PROJECT: WORDLE
use colored::Colorize;
use std::io::{self, Write};
fn main() {
    let word = "trait";
    let input = io::stdin();

    for _chance in 1..=6 {
        println!("enter a 5 character word");
        let mut user_input = String::new();

        input
            .read_line(&mut user_input)
            .expect("Failed to provide input");

        for (word_character, user_character) in word.chars().zip(user_input.trim().chars().take(5))
        {
            if word_character == user_character {
                //exact match

                print!("{}|", format!("{}", user_character).on_green());
            } else if word.contains(user_character) {
                //if the word just exists
                print!("{}|", format!("{}", user_character).on_yellow());
            } else {
                print!("{}|", format!("{}", user_character).on_black());
            }

            io::stdout().flush().unwrap(); //this is for refreshing the terminal(the color library need that)
        }

        println!();

        if word == user_input.trim() {
            println!("You got it! the word is {word}");
            break;
        }
    }
}
 */
////////////////////////////////////////////////////
////////////////////////////////////////////////////
//PROJECT
use std::collections::HashMap;
use std::env;

// #![allow(unused, dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug, Clone)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug, Clone)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let blender_orders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|order| order.product == Product::Blender)
        .collect();
    println!("******************************************************");
    println!("{:#?}", blender_orders);

    let micro_waves_ordered_quantity = orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .fold(0, |accumulator, current| accumulator + current);
    println!("******************************************************");
    println!("{}", micro_waves_ordered_quantity);

    let args = env::args();
    let mut qunatity_from_user = args.skip(1).take(1);
    let quantity = qunatity_from_user.next().unwrap_or_else(|| 2.to_string());
    let parsed_quantity = quantity.parse::<i32>().unwrap_or_default();
    let orders_dependon_user_input = orders
        .iter()
        .filter(|order| order.quantity as i32 >= parsed_quantity)
        .collect::<Vec<&CustomerOrder>>();

    println!("******************************************************");
    println!("{:#?}", orders_dependon_user_input);

    let unshipped_orders_products_quantity = orders
        .iter()
        .filter(|order| !order.shipped)
        .map(|order: &CustomerOrder| (order.product.clone(), order.quantity))
        .fold(HashMap::new(), |mut acc, (product, quantity)| {
            *acc.entry(product).or_insert(0) += quantity;
            acc
        });
    println!("******************************************************");
    println!("{:#?}", unshipped_orders_products_quantity);

    orders.iter_mut().find(|order| !order.shipped).map(|order| {
        order.shipped = true;
    });
    println!("******************************************************");
    println!("{:#?}", orders);

    let customers_and_their_orders: Vec<Customer> = orders
        .iter()
        .zip(customer_ids_by_order)
        .fold(HashMap::new(), |mut accumulator, (order, user_id)| {
            accumulator
                .entry(user_id)
                .or_insert_with(Vec::new)
                .push(order);
            accumulator
        })
        .into_iter()
        .map(|(user_id, order_refs)| Customer {
            id: user_id,
            orders: order_refs.into_iter().cloned().collect(),
        })
        .collect();
    println!("******************************************************");
    println!("{:#?}", customers_and_their_orders);
}
