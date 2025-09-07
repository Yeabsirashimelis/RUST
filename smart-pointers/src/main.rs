/*
POINTERS
-a variable is a name for a piece of data
-the rust book defines a ointer as a variable that contains an adress in memory
-by storing an address, a pointer variable indirectly points to a piece of data
-pointer prevents the duplication of the original piece of data. instead, we store the data's address

REFERENCE - is a type/ category of pointer that points to valid data
 RUST'S borrow checker guaranties that reference point to valid / allocated data.
 but POINTER is not guaranteed

 RAW POINTER
 a raw pointer is a varaible that stores a memory address without any safety checks
 a raw pointer may point to valid data but it can also point to deallocated memory
  common in C OR C++
  but RUST also support it (when to use them : when we interact with other lanugages in library creation (for commonground b/n the languages))
 */
/*
////////////////////////////////////////
///////////////////////////////////////

//ROW POINERS AND UNSAFE CODE
fn main() {
    let mut sushi = String::from("Yellowtail");
    /*
    //this will be a problem. b/c if mutable reference exists, other reference is not allowed
    //these safety mesures are working. b.c we are using references
    let sushi_ref = &mut sushi;
    let sushi_ref2 = &sushi;
    println!("{} and {}", sushi_ref, sushi_ref2);
     */

    /*
    row pointer equivalent
     */
    let sushi_raw_pointer_1 = &raw const sushi; //const - means immutable
    let sushi_raw_pointer_2: *const String = &sushi;

    let sushi_raw_mutable_pointer_1 = &raw mut sushi; //mutable
    let sushi_raw_mutable_pointer_2 = &raw mut sushi;

    //here the coexsitence of mutable references is allowed
    //DEREFERECING raw pointer is unsafe operatio. so rust need that code block to be in unsafe block
    //           means rust needs acknowledgment form us for the potential problems could result from the code block
    unsafe {
        println!(
            "{} and {}",
            *sushi_raw_mutable_pointer_1, *sushi_raw_mutable_pointer_2
        );
    }

    //let's mimic here that the "sushi" data is cleaned up from the memory
    drop(sushi);
    unsafe {
        //as you can see here, the program compiles will have a problem
        //  as "sushi_raw_mutable_pointer_1" is dangling pointer
        println!("{}", *sushi_raw_mutable_pointer_1);
    }
}
 */

/////////////////////////////////////////////////////
///////////////////////////////////////////////////
/*
SMART POINTERS
 a smart pointer is a type that behaves like a pointer
 a smart pointer can store additional information and perform more actions compared to plain pointer/ reference
 most smart pointer are build with structs. structs grant the capacity to store more data

 Reference like &T borrow data
 reference are not responsible for diallocating the data; the original owner is responsible for deallocation.
 where as, SMART POINTERS often own and manage their own datam typically on the heap

 the advantage is that smart pointers behave like pointers but can be treated like owned types

 A String is a smart pointer
  a String points a pointer to the heap memory where the text data is located
  a string also stores extra metadata like the length and capacity of the text
  The string smart pointer handles the complexity of the pointer/reference behind the scenes.
  so that we trat the String like the regular owned type (owner) -> we don't worry about any reference thing, we take it as an owner of ths stored data
 */
/*
////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////
//THE BOX SMART POINTER
/*
THE BOX SMART POINTER stores a piece of data(any type) on the heap
the BOX is an owned type that is a container around the raw pointer that holds the memory address of the allocated heap data
 *SO THE BOX SMART POINTER ABSTRACTS OR HIDES AWAY THE COMPLEXITY OF DEALING WITH THAT RAW POINTER TO THE HEAP DATA THAT YOU SPECIFY THAT REQUEST TO BE STORED
//the box pointer stored on the stack, b/c the size of the raw pointer never changes. the data pointed is on the heap so is able to change
*/
fn main() {
    //a box wrapper which wraps the raw pointer that is pointing the data, "100" in the heap
    let my_box = Box::new(100); //we can put any type of data (structs, Strings, Enums, Floats, Booleans, ...)
    println!("{}", *my_box);

    let your_box = my_box;

    //this doesnot work
    //this is a proof that smart pointers act as owned type. b/c ownership is moved here
    // println!("{}", my_box);
}
 */
//////////////////////////////////////////////////
//////////////////////////////////////////////////
//INTRO TO LINKED LISTS
//one of the common use cases of a box smart pointer is a recursive data structure
/*
recursive functions use a base case to force the termination of recursion
recursive data structures use a Box to solve the infinity problem
the compiler doesnot have to worry about the nested daa structures occupying infinite memory

A LINKED LIST is a data structure that consists of a linear collection of connected nodes
each node stores a piece of dataa and a reference to the next node in sequence
unlike arrays/vectors, the nodes are not stored contiguosly in memory which makes it easier to insert them in the middle of the list

//this is how it is modeled
 struct MusicPlayListItem {
   name: Stirng,
   artist: String,
   nextTrack: MusicPlayListItem
}
*/
/*

#[derive(Debug)]
enum LinkedList<T> {
    Empty,
    Node { value: T, next: Box<LinkedList<T>> },
}
fn main() {
    let list = LinkedList::Node {
        value: 1,
        next: Box::new(LinkedList::Node {
            value: 2,
            next: Box::new(LinkedList::Node {
                value: 3,
                next: Box::new(LinkedList::Empty),
            }),
        }),
    };

    println!("{:?}", list);

    //let's create a play list
    let im_with_u = LinkedList::Node {
        value: String::from("I'm with you"),
        next: Box::new(LinkedList::Empty),
    };

    let skBer_boi = LinkedList::Node {
        value: String::from("SkBer Boi"),
        next: Box::new(im_with_u),
    };

    //this will be invalid, b/c ownership is moved.
    // println!("{:?}", im_with_u);

    let complicated = LinkedList::Node {
        value: String::from("Complicated"),
        next: Box::new(skBer_boi),
    };

    println!("{:?}", complicated);
}
 */
/////////////////////////////////////////////
///////////////////////////////////////////
//BOX VS REGULAR REFERENCE
/*
#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        //box smart pointer is a reference wrapper and can take ownership. therefore there will no be lifetime problem
        //call it like a reference which acts as the owner of the data
        next: Box<LinkedListUsingBox<T>>,
    },
}

#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        //it is just regular reference. so ownership problem is there
        //the referenced value must live at least as long as the struct that holds the reference.
        // a reference cannot outlive the thing it references
        next: &'a LinkedListUsingReference<'a, T>,
    },
}

fn main() {
    //regular REFERENCES
    let second_node = LinkedListUsingReference::Node {
        value: 2,
        next: &LinkedListUsingReference::Empty,
    };

    let first_node = LinkedListUsingReference::Node {
        value: 1,
        next: &second_node,
    };

    // println!("{:?}", first_node);

    //this is ok, the referenced thing can outlive the referencer
    // drop(first_node);
    // println!("{:?}", second_node);

    //this is invalid b,c if the referenced is outlived, the references will point to a nothing (DANGLING REFERENCE)
    // drop(second_node);
    // println!("{:?}", first_node);

    //BOX
    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node),
    };

    // the "next" field in the first node takes the ownership of the "second_node". so lifetime thing is needed here. the onwer gives the ownership to another thing
    // println!("{:?}", second_node);
}
 */
/*
#[derive(Debug)]
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        //box smart pointer is a reference wrapper and can take ownership. therefore there will no be lifetime problem
        //call it like a reference which acts as the owner of the data
        next: Box<LinkedListUsingBox<T>>,
    },
}

#[derive(Debug)]
enum LinkedListUsingReference<'a, T> {
    Empty,
    Node {
        value: T,
        //it is just regular reference. so ownership problem is there
        //the referenced value must live at least as long as the struct that holds the reference.
        // a reference cannot outlive the thing it references
        next: &'a LinkedListUsingReference<'a, T>,
    },
}
 */
/*
/*
//THIS IS THE CASE THAT REGULAR REFERENCES TOTALLY DON'T WORK FOR LINKEDLIST
//  which is using another function to create the linkedlist
fn create_list<'a>() -> LinkedListUsingReference<'a, i32> {
    //the reason is that at the end of the function the "second_node" goes out of scope
    //  so it cannot be "first_node" cannot returned as it contains references to "second_node" -> which is basically nothing at the end of the function
    //we need a thing which takes the ownership and regular reference is not the way to go for this case
    let second_node = LinkedListUsingReference::Node {
        value: 2,
        next: &LinkedListUsingReference::Empty,
    };

    let first_node = LinkedListUsingReference::Node {
        value: 1,
        next: &second_node,
    };

    first_node
}
 */

//USING BOX
fn create_list() -> LinkedListUsingBox<i32> {
    //this will work b/c "second node" moves its ownership to the next field of the first node before it goes out of scope
    let second_node = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let first_node = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node),
    };

    first_node
}

fn main() {
    let list = create_list();
    println!("{:#?}", list);
}
 */
/*
///////////////////////////////////////////////////////
/////////////////////////////////////////////////////
///VECTORS ARE SMART POINTERS

#[derive(Debug)]
enum FileSystemEntity {
    File {
        name: String,
    },
    Folder {
        name: String,
        content: Vec<FileSystemEntity>, //here is there is no problem in this recursive data structure, b/c the vector is a smart pointer
    },
}
fn main() {
    let rust_file = FileSystemEntity::File {
        name: String::from("my rust code.rs"),
    };
    let python_file = FileSystemEntity::File {
        name: String::from("my python_code.py"),
    };
    let code_folder = FileSystemEntity::Folder {
        name: String::from("Code Stuff"),
        content: vec![rust_file, python_file],
    };
    let screenplay = FileSystemEntity::File {
        name: String::from("MY Screenplay.txt"),
    };
    let all_documents = FileSystemEntity::Folder {
        name: String::from("Documents"),
        content: vec![screenplay, code_folder],
    };

    println("{:?}", all_documents);
}
 */
/*
//////////////////////////////////////////
///////////////////////////////////
//BINARY SEARCH TREES

use std::cmp::Ordering;

#[derive(Debug)]
enum BinarySearchTree {
    Empty,
    Node {
        value: i32,
        left: Box<BinarySearchTree>,
        right: Box<BinarySearchTree>,
    },
}

impl BinarySearchTree {
    fn new() -> Self {
        BinarySearchTree::Empty
    }

    fn insert(&mut self, new_value: i32) {
        match self {
            BinarySearchTree::Empty => {
                *self = {
                    BinarySearchTree::Node {
                        value: new_value,
                        left: Box::new(BinarySearchTree::Empty),
                        right: Box::new(BinarySearchTree::Empty),
                    }
                }
            }
            BinarySearchTree::Node { value, left, right } => match new_value.cmp(value) {
                Ordering::Equal => (),
                Ordering::Less => {
                    left.insert(new_value); //recursively call insert method on the left
                }
                Ordering::Greater => {
                    right.insert(new_value); //recursively call insert method on the right
                }
            },
        }
    }

    fn contains(&self, target: i32) -> bool {
        match self {
            BinarySearchTree::Empty => false,
            BinarySearchTree::Node { value, left, right } => match target.cmp(value) {
                Ordering::Equal => true,
                Ordering::Less => left.contains(target),
                Ordering::Greater => right.contains(target),
            },
        }
    }
}
fn main() {
    let mut tree = BinarySearchTree::new();
    tree.insert(5);
    tree.insert(2);
    tree.insert(8);
    tree.insert(1);
    tree.insert(10);
    println!("{:#?}", tree);

    println!("{}", tree.contains(13));
    println!("{}", tree.contains(10));
    println!("{}", tree.contains(2));
}
 */
///////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////
// THE DEREF AND DEREFMUT TRAITS
//DEREF TRAIT - enables a type to behave like a reference. smart pointers extends this trait by default
//                that is why they can be dereferenced to give the values they contain
//  DEREFMUT TRAIT - enable the pointer to mutate the data, like Box pointers (they implement the trait)
//DerefMut is a child trait, so to implement it we have to implement the Deref trait first
/*
//to get the idea, let's build a custom smart pointer which contains only one data
struct CustomBox<T> {
    data: T,
}

impl<T> CustomBox<T> {
    fn new(data: T) -> Self {
        CustomBox { data }
    }
}

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for CustomBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn main() {
    let mut boxy = Box::new(3.14);
    *boxy = 6.28;
    println!("{}", *boxy);

    let mut custom_boxy = CustomBox::new(3.14);
    *custom_boxy = 6.28;
    println!("{}", *custom_boxy); //now it can be dereferenced
}
*/
/*
//and now let's build a custom smart pointer which contains multiple data
struct CustomBox<T, U> {
    data: T,
    more_data: U,
}

impl<T, U> CustomBox<T, U> {
    fn new(data: T, more_data: U) -> Self {
        CustomBox { data, more_data }
    }
}

impl<T, U> Deref for CustomBox<T, U> {
    type Target = T; //the target is the one in which we want to be dereferenced as the real value of the pointer
                     //  other datas included can be just metadata

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, U> DerefMut for CustomBox<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn main() {
    let mut boxy = Box::new(3.14);
    *boxy = 6.28;
    println!("{}", *boxy);

    let mut custom_boxy = CustomBox::new(3.14, String::from("Pi"));
    *custom_boxy = 6.28;
    println!("{}", *custom_boxy); //now it can be dereferenced
}
 */

/*
/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////
//DEREF COERTION
/*
when given a reference to a type that implements the Deref trait,
Rust will convert it into a reference of another type if necessary
*/
use std::ops::{Deref, DerefMut};

fn main() {
    /*
    //when we pass reference for the string, rust will call the "deref mothod" automatically on the String and change to &str
    let text = String::from("Hello");
    output_text(&text); // - &String to &str

    //this is how behind the scene works
    let text = String::from("Hello");
    let the_slice = text.deref();
    output_text(the_slice);
     */

    //this is the magic of Deref Coercions
    let text = String::from("Hello");
    let my_box = Box::new(text);
    output_text(&my_box); // &Box -> &String -> &str

    //how the code look like if deref coertion was not implemented in RUST
    let text = String::from("Hello");
    let my_box = Box::new(text);
    let value = &(*my_box)[..]; //dereference it, then create a String reference and take the full string as a slice, so you created a &str
    output_text(value);
}

fn output_text(text: &str) {
    println!("{}", text);
}
 */
/////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
//TRIAT OBJECTS I
/*
A TRAIT OBJECT is an instance of some type that implements a specific trait
*/

use std::{fmt::format, string};

trait Wearable {
    fn wear(&self) -> String;
}

#[derive(Debug)]
struct Pants {
    fabric: String,
    waist_size: u32,
}

impl Wearable for Pants {
    fn wear(&self) -> String {
        format!("{} size {} pants", self.waist_size, self.fabric)
    }
}

#[derive(Debug)]
struct Tie {
    color: String,
}

impl Wearable for Tie {
    fn wear(&self) -> String {
        format!("{} tie", self.color)
    }
}
fn main() {
    let pants = Pants {
        fabric: "Cotton".to_string(),
        waist_size: 34,
    };

    let tie = Tie {
        color: "Red".to_string(),
    };
    let pants = Pants {
        fabric: "Cotton".to_string(),
        waist_size: 34,
    };

    let tie = Tie {
        color: "Red".to_string(),
    };

    //say that i want to create a collection type, like a vector that is going to package and hold both of these types

    //this won't be possible (is an error), b/c a vector can hold only the same types
    // let outfit = vec![pants, tie];

    //the BOX smart pointer will enable us to solve the problem.
    //if we wrap them in a box, and then we are going to add an additional requirement or an additional mandate

    //OH STILL DON'T WORK - b/c rust thinks that we are storing Box pointers which contain pant types
    // let output = vec![Box::new(pants), Box::new(tie)];

    //ONE MORE THING TO DO
    //we make the type hold things which implement "Wearable" traits - DYNAMIC TYPE POLYMOPHISM / RUNTIME POLYMORPHISM
    let outfit: Vec<Box<dyn Wearable>> = vec![Box::new(pants), Box::new(tie)];

    // for item in outfit {
    //     println!("putting on the {}", item.wear())
    // }

    let items = outfit
        .iter()
        .map(|item| item.wear())
        .collect::<Vec<String>>();

    println!("{:#?}", items);
}
