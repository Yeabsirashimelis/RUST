/*
SLICES
- a collection type is one of that can hold multiple values.
- arrays, tuples and strings are collection types.
- a slice is a reference to a portion/ sequence of a collection type. it is a subcategory of reference.
- a STRING SLICE is a reference to a sequence of characters from a string.
- an ARRAY SLICE is a reference to a sequence of elements form an array
- as a reference, a sllice does not take ownership of the collection. it just borrows portion of// chunk of the collection.

* so in real world example :- if you borrow the bedroom, corridor or the whole of my house.
so if it refers the whole collection it will be equivalent with references.
*/

/*
fn main() {
    let action_hero = String::from("Captain America");
    let string_reference = &action_hero;
    println!("{string_reference}");
    ////the ABOVE reference is a reference to the whole part.

    //now let's see slices
    //in the square brackets we will put the range for bytes, not characters
    //it works as you expected/ range of chars / for alphabetic characters as they occupy one byte.
    //but emojies can take 2 or 4 bytes.
    let action_hero = String::from("Captain America");
    let first_name = &action_hero[0..7];
    // &action_hero[0..7] is a reference to just part of the underlying string data, without the metadata. This is why it's a &str.

    println!("{first_name}");

    let last_name = &action_hero[8..15];
    println!("{last_name}");

    //WHEN WE DECALRE STRING LITERALS, THE VARIBALE WILL BE A REFERENCE TO THE STRING SOMEWHERE IN THE EXECUTABLE
    // the variable refers the whole string.
    // let action_hero = "Iron man";

    //action_hero is a reference, but first_name is not a reference to a reference.
    //it just uses the reference to get the string and create reference to the portion of it.
    // let first_name = &action_hero[0..4];
    // &action_hero[0..7] is a reference to just part of the underlying string data, without the metadata. This is why it's a &str.

    // println!("{first_name}");

    //using block
    let first_name = {
        //here you may be curious that the first name will be dangling reference, b/c action_hero reference variable goes out of scope.
        //but that is not true. b/c the string is always there in the executable and first name slice get to refer the portion of the string
        //   using the action_hero reference, before it goes out of scope.
        let action_hero = "Iron man";
        &action_hero[0..4]
    };

    println!("{first_name}");
}
 */

//REMEBERS - STRING LITERALS ARE STRING SLICES

/*//the length of a string slice refers to a count of its bytes, not its characters.
fn main() {
    // let food: &str = "pizza";
    // println!("{}", food.len()); //it will give us the length of bytes, not characters.

    // let pizza_slice = &food[0..3];
    // println!("{}", pizza_slice.len());

    let food = "🍕"; // The emoji appears as one character but is actually 4 bytes in UTF-8.
    println!("{}", food.len()); // Outputs: 4 (length in bytes)

    // Trying to slice from 0 to 3 would cause a runtime error!
    // The reason: Those 3 bytes do not form a valid UTF-8 character, breaking encoding rules.
    // let pizza_slice = &food[0..3]; // ❌ This will panic at runtime!
    // println!("{}", pizza_slice.len()); // Won't reach here due to the panic
}
*/

/*
//SYNTATIC SHORTCUTS
fn main() {
    let action_hero = String::from("black widow");

    //the following 2 means the same thing. if we start from the beggining we can omit 0.
    let _first_name = &action_hero[0..5];
    let first_name = &action_hero[..6];
    println!("{first_name}");

    //also if we want to continue to the last we can only specify the initial character
    let last_name = &action_hero[6..];
    println!("{last_name}");

    let full_name = &action_hero[..]; //this means inlcude the whole thing.
}
 */

/*
//STRING SLICES AS FUNCTION PARAMETERS
fn main() {
    let action_hero = String::from("Wanda Maximof");
    do_hero_stuff(&action_hero);

    let another_action_hero = ("peter parker");
    do_hero_stuff(&another_action_hero);
}

//if the parameters type was &String, it won't support &str parameters in the callers
//but &str is flexible and works for both.
//automatically rust converts it from &String to &str b/c we are sure that there is a full piece of content to be represented and it can be represented by &str
//rust don't convert &str to String, b/c we are not sure that the slice references the full piece of content.
fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}
 */

/*
//ARRAY SLICES - represents chunk or portion of the array.
fn main() {
    let values = [4, 8, 15, 16, 23, 42];
    let slice = &values[0..3];

    println!("{slice:?}");

    //synthatical shortcuts can be used here as well
    let _slice = &values[..3];

    // The first statement creates a slice of the entire array.
    // A slice (`&[T]`) is dynamically sized, meaning it doesn’t store the array's exact length at compile time.
    let my_slice = &values[..];

    // The second statement is a reference to the whole array (`&[T; 5]`).
    // Unlike a slice, this keeps the array's fixed size in the type.
    let my_slice = &values;
}
 */

/*//  DEREF COERTION WITH ARRAY SLICES
fn main() {
    let values = [4, 8, 15, 16, 23, 42];

    let regular_reference = &values;
    print_length(regular_reference);
    print_length2(regular_reference);

    //in slice as we can see there is no concrete size associated with it.
    //that dynamic behaviour makes it more flexible and versatile.
    let slice_of_three = &values[..3];
    // print_length(slice_of_three); // but for this we will get a type mismatch.
    print_length2(slice_of_three);
}

fn print_length(reference: &[i32; 6]) {
    println!("{}", reference.len());
}

//what if we make the param type to be an array slice.
// -as we saw in the string types the string slice can handle both situations
//as the string types, a full array can be represented using a slice. but not the reverse.
// - like wise here the array slice is valid for both
fn print_length2(reference: &[i32]) {
    println!("{}", reference.len());
}
 */

/*
//MUTABLE ARRAY SLICES
//in STRINGS, RUST does NOT permit mutable slices. we can't change anything on the slice which is a portion of the sul string.
//however RUST permits mutable slices of arrays.
fn main() {
    let mut my_array = [10, 15, 20, 25, 30];
    let my_slice = &mut my_array[2..4];

    println!("my slice: {:?}", my_slice);

    //the changes will be reflected in the original array but only for the slice parts.
    my_slice[0] = 100;

    println!("my slice: {:?}", my_slice);
    println!("my slice: {:?}", my_array);
}
 */

///////////////////
///
//CODING CHALLENGE
fn main() {
    let mut cereals: [String; 5] = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    println!("my full array is {:?}", cereals);

    let first_two = &cereals[..2];
    println!("my first slice is {:?}", first_two);

    let mid_three = &cereals[1..4];
    println!("my mid slice is {:?}", mid_three);

    let last_three = &mut cereals[2..];
    println!("my last is {:?}", last_three);

    last_three[2] = String::from("Lucky Charms");
    println!("my full array is {:?}", cereals);

    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[..7];
    println!("{}", cookie);

    let cocoa_puffs = &cereals[3];
    let puffs: &str = &cocoa_puffs[6..];
    println!("{}", puffs)
    ;
}
