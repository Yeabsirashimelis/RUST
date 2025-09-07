/*
-one of the most unique features is ownership.
ownership is a compiler feature, it has no impact on the actual program when it runs.
it exists for the benefit of us, DEVELOPERS.
// OWNERSHIP - is a set of rules that the compiler checks for to ensure the program will be free of memory errors
-memory refers to the area of your computer that is responsible for storing the information your program use.
   memory exists on temporary dynamic datas, as the program is running.
-it is ideal to free memory whe it is no longer in use.

in languages like C/C++ the memory management is manual. means you have to deallocate unused memorys, but
  unfortunatelly humans made mistakes like forget to deallocate memory or may try to deallocate a memory which is already deallocated.
  so these create bugs in the programs.

like languages jaa ,python, ruby and  Go implement a tool called the garbage collector.
the garbage collector looks for data that is no longer in use and deallocates it. it automates the clean up process.
the garbage collector itself occupies memory and can run at disadvantageous times.

RUST offers us the best of all solutions. it introduces a new paradigm: OWNERSHIP.
ownership is a set of rules on how RUST manages your computer memory.
the RUST compiler does not compile the program if an ownership rule is violated.
Best of all worlds: the speed of a language like C with less room for error.
                WHAT IS OWNERSHIP?
    -the owner is who/what is responsible for cleaning up a piece of data when it is no longer in use.
    -who will clean up the memory when the data in it is no longer in use.
    -every value in a RUST program has one owner. one owner at a time.
    -the owner can change over the course of the program, but there is only 1 owner for a value at a time.
    -the owner is usually a name.
     * a variable can be an owner
     * a parameter can be an owner
    -ownership also extends to composite types that own their elements.
     * a tuple and array own their values.

*/

/*
                   THE STACK AND THE HEAP
       -the stack and the heap are 2 different parts or regions of the computer's memory that are avaailable for RUST at run time
       -when the program needs to store a value into a memory, it will choose b/n the stack and the heap.
stack - is generally faster (b/c there is only one place to push or pop a data - the top of the stack), but it only supports data of a fixed, predictable constant size and the size must be known at compile time.
        data type like integers, floating-points, booleans, character and array have a fixed size, RUST stores them on the stack at runtime.
        the pieces of data on the stack will not grow or shrink in size as the program runs.
heap - the heap is generally slower, but it supports dynamic data that can change in size.
       the heap is a large area of storge space. think of it like a warehouse.
       the heap is for a data whose size is not known at compile time (user input, a file's contents, etc).
       when the rust program needs dynamic space, it requests it from the heap. a program called the "memory allocator" finds an empty spot that is large enough to store the data.
       the memory allocator returns a reference, which is an address.
       the reference points to the memory address of the data. think of a parking lot giving you a reference / pointer when they park your car.
       we can store a reference in a variable in a RUST program. references have a fixed size, so RUST stores them on the stack.

the purpose of ownership is to assign responsibility for deallocating memory (primarly heap memory).
ownership is a compiler feature for reducing duplicate heap data and cleaning up heap data that us no longer needed.
*/
/*
fn main() {
    let age = 33; //age is the owner of the value 33. the owner is who is responsible to clean up the data. the owner knows to cleanup the data when the variable goes out of scope.
    {
        let is_handsome = true;
    } //is_hansome goes out of scope here
      //age variable exists here
} //age variable goes out if scope here.
  //so the memories will be cleared in the LIFO order - last in first out
 */

/*
a trait - is like a contract a type promises to uphold

//THE COPY TRAIT - mandates that a type can be copied. a full duplicate can be created
                   primitive data types, or fixed sized ones that we store on the stack like integers, floats, booleans, characters and more. they all implemenet the copy trait.

*/
/*
fn main() {
    //these 2 are duplicate, independent and separate. so they both takes different palces in the stack
    let time = 2025;
    let year = time;

    println!("the time is {time}. it is the year {year}."); //this ensures that both are valid and takes a memory in the stack.
} //after the variables are out of scope the "year" variable is the first one to be pooped out / cleaned
 */

//THE STRING TYPE
/*
RUST has 2 string types
 str string - whenever we use a double quotes, which is the string literal.
   its value is neither stored in the stack nor in the heap. it is embedded into the binary executable that the rust compiler produces.
   the reason that is done is that the value is known at compile time.

  String - is still collection of UTF-8 encoded characters. it is used for dynamic strings.
          strings that are going to change in the course of the program. mutable and stored in the heap.

   */
/*
fn main() {
    let food = "pasta";
    let text: String = String::new(); //the "text" variable is the owner of the string and it is responsible to clean it up at the end of the scope / the main funtion.
    let candy: String = String::from("kitkat"); // to initialize it with a value in the heap.
}
 */

/*
//PUSH() - is a method to add a text at the end of a string / concatenation
fn main() {
    //what will happen behind the scene is that the string's text content will live on the heap, but this creation of a string will
    // also creatte a stack entry. so there is a data both on the heap and the stack
    // the stack entry will hold three pieces of data.
    // 1, the first piece of data will be a reference to the string. and remember, a reference is an address  to the heap location where the text is stored.
    // 2, the second piece of information on the stack will be the length of the string, which is the current number of bytes that the text occupies.
    // 3, the third and the final piece of information on the stack is the capacitty and what the capacity is, the amount of bytes that is available in the heap location.
    /*
      ✔reference       ✔Yeabsira
      ✔length            HEAP
      ✔capacity
       STACK
    */
    let mut name = String::from("Yeabsira");
    println!("My name is {name}");
    //                    FIRST POSSIBILITY
    //when we try to push and the capacity is enough, the concatenated string will be written on the heap, and
    //  the length will be changed on the stack where as the reference and the capacity remains the same.

    //                    SECOND POSSIBILITY
    //if the capacity can not fit the new characters, the text will be moved to a new place on the heap that has the capacity to store.
    // and the old space in the heap is deallocated.
    //so the all info in the stack changes, the reference, length and the capacity infos in the stack.
    println!("My fullname is {name}");
}
 */

/*
////////////////
/// //
///
/*
MOVE - is the transfer of ownership from one owner to another
*/
fn main() {
    //here we are dealing with heap data and String - which is not a primitive data type and doesnot implement the COPY TRAIT
    //in the heap the string will not be copied to the genius like that of the stack datas.
    //it just creates a a stack entry with foe "genius" and copy the reference, length and capacity from the "person"
    //so ultimately one piece of heap data, but 2 references to it the stack.
    //WHO IS THE OWNER NOW?
    // -as you might guess, "genius" - as we assign person to genius, RUST moves the ownership of the value to "genius".
    // so person goes out of scope / INVALID
    let person: String = String::from("Yeabsira");
    let genius = person;
    // println!("My name is : {person}"); -- invalid as "person" goes out of scope.

    /* in other languages there is not an ownership concept. they allow both variables to be valid
      then, when the function ends and they go out of scope, the first variable clears the heap. and the second one will
      will also try to clear that heap area with is already cleared by the first variable, so this will create an ERROR -
      and called "DOUBLE FREE ERROR" in othere programming languages and it can lead to memory corruption and security
      vlnerabilities. but it is no possible in RUST. we can't have 2 variables, 2 owners, attempting to clean up the same heap data
      b/c we can't have 2 owners.
    */
}
*/
////////
////////

/*
/*
THE DROP FUNCTION - used to deallocate memory in the heap.
it is automatically called by RUST at the end of a scope.
but we can do it manually by invoking this function to invalidate a variable and deallocate the memory in the heap.
THE CLONE METHOD - if we want to copy the heap data, we use this method explicitly.
*/
fn main() {
    let person: String = String::from("Yeabsira");
    drop(person); //after this we can't use the person variable
                  // println!(person); //this will be an error

    //now we have different heap memory locations, techinically took the same texts if we read.
    // there is no tranfer of ownership is here.
    //DON'T use clone method if it is not really neccessary, b/c it create a copy in the heap, which is relatively slower.
    let person: String = String::from("Yeabsira");
    let genius = person.clone();
}
 */

//////
/*
REFERENCES AND BORROWING
the problem of cloning is when multiple parts of the program needs the exact same value found in the heap.
exact copy of primitive data types has no performance issue / a light weight / at all.
but cloning datas in the heap is costy/ expensive as it is going to occupy some chunk of memory.
we can use instead a REFERENCE - allows the program to use a value with out moving ownership. and it is called the action of borrowing
BORROWING - means using something without taking ownership of it. so, REFERENCE is an address of the value in the memory.

so reference is a NOUN, it is the entity we are creating in our program
and borrowing is the VERB - to create a reference to some value without taking the ownership of it.

we use a special character which is -  ampersand (&) -a borrow operator
* the other problem rust solve is that references (called pointers in other languages) can point to a memory location that is already deallocated and which can lead to problems.
   but in RUST it is guranteed that a reference is pointing to existing value.
*/

/*
Dereference operator - represented by asterisk(*) operator.
 the dereference means to access the data at the memory address that the reference points to.
*/
/*
fn main() {
    //it is not neccessary in stack data as copying is cheap. but just to see...
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value; //which contains the address, &i32 - is an adress leading to i32.
    println!("{}", *my_integer_reference);
    println!("{}", my_integer_reference); // this will print the value as well because RUST's prinln macro calls the displat trait.
                                          // the implementation of displat and debug for references automatically dereferences them when neccessary.

    let my_heap_value: String = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("{}", *my_heap_reference);
}
 */

/*
//String, &String, str, &str
fn main() {
    // String - a dynamic piece of text stored on the heap at runtime.
    // &String ("ref String") - a reference to a heap string
    //  str - a hardcoded, read-only piece of text encoded in the binary executable
    //  &str ("ref str") - a reference to the text in the memory that has loaded the binary executable file.

    let ice_cream = "Cookies and Cream"; // after running the code the text is embedded in the binary executable and then loaded to the memory. so the variable is a reference.
                                         // by default so it will be a reference to the value in the memory.
    println!("{}", ice_cream);
}
 */

/*
//THE COPY TRAIT WITH REFERENCES - earlier in this section, we said that the stack types implements the copy trait.
fn main() {
    //now both are reference to a same string the binary. 2 separate variables that store the same address.
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream;
    println!("{ice_cream}, {dessert}"); //these are stack datas, so both are valid. there is no transfer of ownership.
}
 */
///////////
///
/*
OWNERSHIP AND FUNCTION PARAMETERS
-when we use function parameters from variables, there could be creating a copy of the variable or transfer of ownership
  depending on the type of data.
*/
/*
fn main() {
    let apples = 6; //apple will NEVER transfer ownership to the value parameter as it is a stack data / i32 implements the copy trait.
    print_my_value(apples);
    println!("{apples}"); // apple is still value, it never loss ownership.
                          ///////////////////////////////////////
    let oranges: String = String::from("Oranges");
    print_my_value2(oranges);
    // println!("{oranges}"); //the oranges is not valid as the oranges variable is no more valid as it transer the ownership to the value parameter.
    //move occurs as "String" type does't implement the copy trait.
}

fn print_my_value(value: i32) {
    println!("your value is {value}");
}

fn print_my_value2(value: String) {
    println!("your value is {value}");
}
*/
/////
/*
///RETURN VALUES CAN TRANSFER OWNERSHIP AS WELL.
fn main() {
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
 */

/*
//MUTABLE PARAMETERS - like that of variables, parameters are immutable by default.
//we have to explicitly declare when we want a parameter to be mutable or capable of changing.
fn main() {
    let burger: String = String::from("Burger"); //making tthe burger variable mutable doesnot solve the problem, b/c ownership is tranferred to the meal variable.                                                 //we are transferring ownership to an immutable variable.
    add_fries(burger);
}

fn add_fries(mut meal: String) {
    // meal.push_str(" and Fries"); //this will be an error
    meal.push_str(" and Fries");
    println!("my meal today is {meal}");
}
 */

/*
///////////
///
//RETURN VALUE I
fn main() {
    let cake = bake_cake();
    println!("I now have a {cake} cake");
}

fn bake_cake() -> String {
    // let cake = String::from("Chocolate Mousee");
    // return cake;
    // //RUST moves the ownership from the cake  variable in the bake_cake function to the cake variable in the main function.

    String::from("Chololate Mousee"); //implicit return - the cake varibale in the main will be the owner after returned.
}
 */

/*
//RETURN VALUE 2
fn main() {
    let mut current_meal: String = String::new();
    // add_flour(current_meal); //if we don't store the returned value on a variable, the string will be cleared as well. b/c the after a return there is a move to ownership, but there is no one to take that, so automaticallt get deallocated from the heap.
    current_meal = add_flour(current_meal);
    current_meal = add_sugar(current_meal);
    //BUT EVERYTIME DOING THIS / HELL OF MOVING OWNERSHIP IS NOT IDEAL, SO THE VERY NEXT LESSON WE WILL SEE HOW TO DO THIS USING REFERENCES
    println!("my meal today is {current_meal}");
}

fn add_flour(mut meal: String) -> String {
    meal.push_str("Add flour");
    meal //the only way to keep the String alive is to return it. otherwise the heap will be cleared when the variable param variable goes out of scope.
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str(", Add Sugar");
    meal //the only way to keep the String alive is to return it. otherwise the heap will be cleared when the variable param variable goes out of scope.
}
 */

 /*
//USEING REFERENCES INSTEAD OF RETURNING
fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    add_sugar(&mut current_meal);

    println!("my meal today is {current_meal}");
}

fn add_flour(meal: &mut String) {
    meal.push_str("add flour");
}

fn add_sugar(meal: &mut String) {
    meal.push_str(", Add sugar");
}
 */

 /*
///////////
/// CODING CHALLENGE I
fn main() {
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert}");
    //there is no transfer of ownership as it is a stack data/ primitive type
    //they implement the copy trait.
    let sushi = "Salmon";
    let dinner = sushi;
    println!("{sushi}"); //&str is reference type, so we are borrowing the value, not transferring the ownership

    let sushi = String::from("Salmon");
    let mut dinner = sushi;
    // println!("{sushi}"); //this will be error, b/c there is transfer of ownership to the dinner variable

    //first the owner of the string was dinner and tranferred to the param meal, then the retured string ownership will be transfered to the the sushi variable
    let sushi = eat_meal(dinner);
    println!("****** {sushi} *****");
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    meal
}
 */
/////
/// REFERENCES AND BORROWING

/*
//IMMUTABLE AND MUTABLE REFERENCE PARAMETERS
fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal); //with this we don't transfer the ownership of the variable to the params.
    show_my_meal(&current_meal);
}

fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}")
}
 */
///////
///
//MULTIPLE IMMUTABLE REFERENCES
//the problem in other prog languages is that when there are multiple references, there should be a problem with the references expectation as one of the references could change the value.
//but RUST do hard to solve this problem.
//rust will allow any number of immutable variables at the same time. - there is no risk with it, b/c none of the references can change the data.

//MUTABLE REFERENCE RESTRICTIONS - a value can only have a single mutable reference at a time. and no non-mutable references as well if after them the mutable variable is utilized/ used.
/*
fn main() {
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;

    println!("{ref1}, {ref2} and {}", car);

    let mut car = String::from("Red");
    let ref1  = &mut car;
    ref1.push_str("and silver"); //we can do this. b/c the restriction is that there will be no change after the immutable variable, so that the immutable variable will use the expected value in the future.
                                 // let ref2 = &car; // this is not allowed if we use the mutable variable below, as there is a mutable variale above it.
    let ref2 = &car; //this will be okay if the mutable variable is not used. b/c there is no chance that the mutale changes the value as it not used after decalaration.

    // println!("{ref1} and {ref2}"); //so here we use the mutable variable, so there will be problem at line 439.
    println!("{ref2}");
}
 */
/*
///////
///
//OWNERSHIP WITH IMMUTABLE AND MUTABLE VARIABLES
//IMMUTABLE REFERENCE - implements the copy trait
//MUTABLE REFERENCE -don't implement the copy trait
fn main() {
    let mut coffee = String::from("Mocha");
    let a = &coffee;
    let b = a; //as an immutable variable implements the copy trait, there will no transfer of ownership. just a full copy will be created.
    println!("{a} and {b}"); // as both are valid, there is no transfer of ownership.

    let a = &mut coffee;
    let b = a;
    // println!("{a} and {b}"); //as there is transfer of ownership reference "a" will not be valid.
}
 */
//////
/// //
//DANGLING REFERENCES - is apointer to a memory address that has been deallocated.
//this creates a problem in other programming languages.
//RUST help us with that no reference can be used after the value on the memory is deallocated.
//so there is no issue with dangling reference in RUST, unlike other languages.
/*fn main() {}

fn create_city() -> &String {
    //here city is the owner of the string in the heap and responsible for deallocating it.
    //when does the deallocating happen? => at the end of the create_city function/ when the city variable goes out of scope.
    //the references we are returning is not valid, b/c the value it is pointing is deallcated at the end of the function. so RUST will not allow to do this.
    let city = String::from("Debre-zeit");
    &city //so this will create an error
}
*/
///////
///
//ONWERSHIP WITH ARRAYS AND TUPLES
//now we see ownership with collection types. we saw the scalar types before
/*
fn main() {
    //"registration" variable is the owner of the array, is the one which is responsible to clean the memory.
    let registration = [true, false, true];

    //the value is a boolean and as boolean implements the copy trait. so a full copy will be created and assigned to the first variable.
    //so there will no transfer of ownership for the value assigned from the array using indexing.
    let first = registration[0];

    //LETS SEE AN ARRAY OF HEAP DATAS, LIKE "STRINGS" - here this typ doesnot implement the copy trait.
    let languages = [String::from("Rust"), String::from("JavaScript")];
    //so there will be transfer fof ownership for the value being selected
    //but this create a weird state, that the variable containing the array will have partial ownership of the elements. and partial to the selector.
    // let first = languages[0]; //so this is PROHIBITED in RUST.
    //another way to do that recommended by the compiler is clone the selected value
    let _first = languages[0].clone(); //just create a full copy for heap data

    // or just use reference for the item to be selected from the array.
    let first = &languages[0];

    println!("{first} and {languages:#?}");

    //LETS SEE TUPLES NOW. - EXCATLY THE SAME RULES AS ARRAYS APPLIED HERE
    let registration = (true, false, true);
    let first = registration.0;
    let languages = (String::from("Rust"), String::from("JavaScript"));
    // let first = languages.0;  // so this is forbidden.

    // so  we have to use either cloning or reference (the one which prevents additional use of memory for the String)
    let first = languages.0.clone();
    //OR
    let first = &languages.0;
}
    */

//CODING CHALLENGE
fn main() {
    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and");
    visit_new_york(&mut trip);
    trip.push_str(" and");
    visit_boston(&mut trip);
    trip.push_str(".");

    show_itinerary(&trip);
}

fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_philadelphia(trip: &mut String) {
    trip.push_str(" Philadelphia");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str(" New York");
}

fn visit_boston(trip: &mut String) {
    trip.push_str(" Boston");
}

fn show_itinerary(trip: &String) {
    println!("{trip}");
}
