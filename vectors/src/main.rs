//VECTOR - is a data structure that is similar to an array.

/*
fn main() {
    //the following is an array
    //type annotation is not neccessary. b/c the compiler can undestand it from the values given.
    //an array has fixed length - this is its greatest limitation.
    //so we can't add or remove an element.
    let foods: [&str; 4] = ["shiro", "doro", "genfo", "chechebsa"];

    //so we use a VECTOR for growing or shrinking size of data.
    //so VEECTORS stored on the HEAP at run time.

    //declared using a constructor function new().
    // We're specifying i32 as the type for the generic in Vec.
    // Vectors in Rust are defined using generics, so we must provide a concrete type for it to work at runtime.
    //both are valid instantiation of vectors.
    let pizza_diameters: Vec<i32> = Vec::new();
    let pizza_diameters = Vec::<i32>::new();

    //to initialize with vlaues, we will use the VEC! MACRO.
    let pizza_diameters: Vec<i32> = vec![1, 2, 3, 4];

    println!("{pizza_diameters:?}");
    let pastas: Vec<&str> = vec!["Rigatoni", "Angel hair", "Fettucine"];
    println!("{:?}", pastas);
}
 */

/*
 //////////////////////////////
///
//ADDING AND REMOVING ELEMENTS
fn main() {
    let mut pizza_diameters: Vec<i32> = vec![1, 2, 3, 4];

    //push(value_to_insert) - to insert element at the end of the vector
    pizza_diameters.push(5);
    pizza_diameters.push(6);

    //insert(index_to_insert, value_to_insert) - to insert element at a specific position in the vector.
    pizza_diameters.insert(0, 10);
    pizza_diameters.insert(3, 20);
    println!("{:?}", pizza_diameters);

    //pop() - remove the last element of the vector
    //        and return it in an option enum. could be SOME VARIANT if the vector had value before popping. and return NONE VARIANT if the vector had no value
    let last_pizza_diameter = pizza_diameters.pop();
    println!("{:?}", last_pizza_diameter);

    //remove(index_to_remove) - remove an element for a certain index position.
    //and it returns the type of the value in the vector.
    //so if we try to get element of invalid index, the program will PANIC.
    let third_diameter_from_start = pizza_diameters.remove(2);
    println!("{}", third_diameter_from_start);

    // pizza_diameters.remove(100); //THIS WILL PANIC THE PROGRAM

    println!("{pizza_diameters:?}");
}
 */

/*
//////////////////////////
/// READING VECTOR ELEMENTS
fn main() {
    let pizza_diameters = vec![8, 10, 12, 14];

    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    //ownership moves from the variables to the vector, b/c the variable contains a STRING type.
    let pizza_toppings = vec![pepperoni, mushroom, sausage];
    /*
    // println!("{}", pepperoni); //so this will be invalid

    //here we are accessing an "i32" which implements the copy trait. so there is no partial move of ownership.
    //so it is not neccessary to use references here as stack datas are cheap to copy.
    let value = pizza_diameters[2];
    println!("{value}");

    //but the compiler is not happy for this. "String" does implement the copy trait
    // no partial transfer of ownership is allowed
    // let value = pizza_toppings[2];

    //so we use reference or cloning
    let value = &pizza_toppings[2];
    println!("{value}");

    //panic will occur at runtime if we try to access an index that doesn't exist in the vector.
    // let value = pizza_diameters[100];

    //we can borrow a slice from vector like arrays
    let pizza_slice = &pizza_toppings[1..3];
    println!("{:?}", pizza_slice);
     */

    //THE GET METHOD - is another way of accesing vector elements.
    //               avoids the  problems with non-existent index positions as it returns the option enum.
    //there is no worry about ownership stuff, b/c the assiciated data is returned to us as a REFERENCE.
    let option = pizza_toppings.get(50);

    match option {
        Some(topping) => println!("the topping is {topping}"),
        None => println!("No value at that index position"),
    }
}
 */

/*
///////////////////////
///
//OWNERSHIP WITH VECTORS - as a heap allocated data type, a vector follows the same rules of ownership that we've previously explored throughout the course
//similarly, when we borrow a reference to a vector, the borrow checker will enforce the standard borrowing rules.
//          RECALL - for eg: if wwe have a mutable reference to a value, we cannot have any other reference
//                  at the same time, you may also recall that we can have any number of immutable reference at the same time

fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    //ownership moves from the variables to the vector, b/c the variable contains a STRING type.
    let pizza_toppings = vec![pepperoni, mushroom, sausage];

    //there will be a transfer of ownership
    let mut delicious_toppings = pizza_toppings;
    // println!("{}", pizza_toppings); //so this will be invalid

    //whenever we insert or remove an element anywherein the vector, behind the scenes taht may require
    // new memory allocation on the heap. sometimes RUST will move or copy the elements to a new area in the heap
    //PERHAPS your vector will reach its full capacty and RUST will move it to a new area  of the heap where it can store.
    //  this is not neccessarily going to happedn on every mutation, but it will happen from time to time on some operations.
    //SO in that scenario, a reference to an element could find itself accidentally pointing to deallocated memory
    /*
    Rust has to be extra careful to ensure that there are no invalid references throughout the program,
    that there are no references pointing to a memory address where the value is no longer present.
         */

    let topping_reference = &delicious_toppings[1];

    //this will not valid for the above scenario we talk. b/c we pushed sa data.
    //b/c the "PUSH()" method takes a mutable reference to push an element.
    // and we are taking an immutable reference at the same time which is just forbidden
    //   to have to references at the same time
    // delicious_toppings.push(String::from("Olives"));
    // println!("{}", topping_reference); //we are using the immutable reference after mutable reference is declared. so we couldnot get the intended data. that is why all the above rules are there in RUST
    //                                     SO we can only use that before the mutable reference (push()) method is used
    //ALL THE ABOVE SHIT IS AN ADVANCED CONCEPT OF RUST CALLED "LIFE TIME". we'll se in it a greater depth later in the course
}
 */

/*
/////////////////////////
////////////////////////////
///
///
//WRITTING VECTOR ELEMENTS
fn main() {
    let pepperoni = String::from("Pepperoni");
    let mushroom = String::from("Mushroom");
    let sausage = String::from("Sausage");

    //ownership moves from the variables to the vector, b/c the variable contains a STRING type.
    let mut pizza_toppings = vec![pepperoni, mushroom, sausage];
    pizza_toppings[1] = String::from("Olives");
    println!("{:#?}", pizza_toppings);

    let target_topping = &mut pizza_toppings[2];
    target_topping.push_str(" and Meatballs"); //by taking a mutable reference to vector item, we can concatenate a string on it.

    println!("{pizza_toppings:#?}");
}
 */

/*
////////////////
///
//VECTOR CAPACITY BEHIND THE SCENES
/*
The vector capacity is the maximum number of elements that the vector can contain

// Rust forbids multiple mutable references because one of them could modify the value's size,
//    potentially causing a reallocation. If the memory location changes, the old reference
//    would become invalid, leading to undefined behavior.

*/

fn main() {
    let mut seasons: Vec<&str> = Vec::with_capacity(4);
    println!(
        "Length : {}, capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    seasons.push("Summer");
    seasons.push("Fall");
    seasons.push("Winter");
    seasons.push("Spring");

    println!(
        "Length : {}, capacity: {}",
        seasons.len(),
        seasons.capacity()
    );

    //the capacity is 4. and now it is full. so if we want to add an element. rust will the data to another location

    seasons.push("Summer");
}
 */

//////////////////////
//////////////
///
//CODING CHALLENGE

#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name: name,
            contents: vec![],
        }
    }

    fn create_file(&mut self, name: String) {
        let new_file = File { name: name };
        self.contents.push(new_file);
    }

    fn delete_file(&mut self, index: usize) -> File {
        let file = self.contents.remove(index);
        file
    }

    fn get_file(&self, index: usize) -> Option<&File> {
        let file = self.contents.get(index);
        file
    }
}

fn main() {
    let mut documents = Folder::new(String::from("Documents"));
    documents.create_file(String::from("programming_assigment.dox"));
    documents.create_file(String::from("page_cover.dox"));
    println!("{:#?}", documents);

    println!("*****************************************************************");

    documents.delete_file(1);
    println!("{:#?}", documents);

    println!("*****************************************************************");

    // let file1 = documents.get_file(0);
    let file1 = documents.get_file(100);

    match file1 {
        Option::Some(file) => println!("the file you request is : {:#?}", file),
        Option::None => println!("there is no file at this location"),
    }
}
