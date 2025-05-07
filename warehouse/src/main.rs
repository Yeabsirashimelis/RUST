//this is one way of how we declare a module. and every code inside the curly braces belongs to it.
//which is in the same file. but not good one.

// mod inventory {}

//this is how to import a module in other file
// mod inventory;

//we can also use folder. then "mod.rs" file for the modules in that folder. / instead of the source directory
//the file name should be "mod.ts", otherwise rust cannot know if there is a mod or a namespace in that directory

// mod orders {
//     pub const MANAGER: &str = "Dagimawi Shimelis";
// }
// mod orders;

//import from the standard library
use std::{
    fmt,
    io::{self, stdin, stdout},
};
// fmt
// io; //wrtting files, reading files, collecting user input and many more

//this is to import every name inside the collections module
use std::collections::*;

//this is just to show. b/c preludes are available for every file without the need to import
use std::prelude::v1::*; //this will happes behind the scenes when we run a RUST program

use fake::{Fake, Faker};

use warehouse::{Item, ProductCategory, FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

//we can import modules as they are names as well. - then we use "produts::fields" in our code
// use inventory::products;

//THE SELF KEYWORD - if we want to import the module and its fields at the same time
// Self - represents whatever before the last name. here represent "products"
// use inventory::products::{self, Item, ProductCategory};

//by using the "PUB USE" keyword in the inventory module, we reexport the products fields from the inventory module (parent)
//  so we can import them from there
// use inventory::{Item, ProductCategory};

// use inventory::products::{Item, ProductCategory};
// use inventory::{talk_to_manager, FLOOR_SPACE};

//let's see the name conflict b/c of the same names imported form different files
// use orders::MANAGER;
/*
so when the same name is shared b/n different modules, many RUST developers recommend providing
the original module name present / use the module name wherewever we use them instead of trying to import the names.
//and for consistency let's remove the manager name import from the inventory
*/

/*
IMPORT NAMES WITH ALIASES/alternate name/ the AS keyword- to avoid name collisions
// */
// use inventory::MANAGER as INVENTORY_MANAGER;
// use inventory::MANAGER as ORDERS_MANAGER;

/*
fn main() {
    //we run in to an issue, b/c the variable is not valid here/ out of scope.
    //like it is in another folder so you can't find it.
    // println!("the manager of our inventory is {}", MANAGER);

    /*SO inorder to reach into the inventory module, we need to use a special syntax.
    and that is the double colon syntax.// we RUSTEANS calls this the "SCOPE RESOLUTION OPERATOR"
    */

    //but this will not be enough. the rust compiler says, constant MANAGER is "PRIVATE".
    //private means items can only be accessed in their module
    // println!("the manager of our inventory is {}", inventory::MANAGER);

    /*
    so inorder to make an item public, we use a special keyword called "PUB", short for public
      before any field we want to make public
     */
    println!("the manager of our inventory is {}.", inventory::MANAGER);

    //duplicate name but valid b/c it is found in other module
    println!("the manager of our orders is {}.", orders::MANAGER);
}

//A BENEFIT OF NAMESPACES
/*
they enable has to have the same name in our program. that is b/c each duplicate name can live within
  its own module.
  -it is like we can't have a files with the same name in the same folder
  but, we can a files with the same name in DIFFERENT folders.
*/
 */

/*
THE USE KEYWORD - brings a bane into the current scope. it creates a
"shortcut" to a name in a nested module, which allows us to ki specifying the full path
to the name each time that we use the name.
 */

/*
fn main() {
    // println!(
    //     "Our managers are {} and {}. we have {} square feet of floor space.",
    //     inventory::MANAGER,
    //     orders::MANAGER,
    //     inventory::FLOOR_SPACE
    // );

    //an absolute path start from the top, concidentally this file is the top.
    //the absolute path is not much different from the relative. b/c "crate" is this file
    // println!(
    //     "Our managers are {} and {}. we have {} square feet of floor space.",
    //     crate::inventory::MANAGER,
    //     crate::orders::MANAGER,
    //     crate::inventory::FLOOR_SPACE
    // );

    //after using the use keyword and import the variables
    /*println!(
        "Our managers are {} and {}. we have {} square feet of floor space.",
        inventory::MANAGER,
        orders::MANAGER,
        FLOOR_SPACE
    );*/

    println!(
        "Our managers are {} and {}. we have {} square feet of floor space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    );

    // inventory::talk_to_manager();
    // talk_to_manager();

    // let favorite_category = inventory::products::ProductCategory::Hammer;

    //after importing the enum directly using the "use" keyword
    /*  let favorite_category = ProductCategory::Hammer;
        println!("my favorite category of item is {:?}.", favorite_category);
    */

    //this struct instantiation iwll give an error. b/c only the struct is public.
    //the fields are private.
    /*let tall_ladder = inventory::Item{
       name:String::from("ladder-o-matic 2000"),
       category: favorite_category,
       qunatity:100
    }*/

    /*ANYTHING DECLARED IN A MODULE IS PRIVATE BY DEFAULT. EVEN A SUBMODULE.
    so inorder to use the submodules in other files, we have to make the submodule
    a public as well
     */
    // let tall_ladder = inventory::products::Item {
    //     name: String::from("ladder-o-matic 2000"),
    //     category: favorite_category,
    //     quantity: 100,
    // };

    // let tall_ladder = Item {
    //     name: String::from("ladder-o-matic 2000"),
    //     category: favorite_category,
    //     quantity: 100,
    // };

    /* let tall_ladder = Item::new(String::from("ladder-o-matic 2000"), favorite_category, 100);

    println!("{:#?}", tall_ladder); */

    //Faker - is a struct imported from the fake library crate
    // it will see the types and creates a struct with prefilled values
    let fake_items: Item = Faker.fake();
    println!("{:#?}", fake_items);

    //this will randomly choose one type from the enum
    let random_category: ProductCategory = Faker.fake();
    println!("{:?}", random_category);
}
 */

//////////////////
////////////
////////////// DOCUMENTATION COMMENTS or "///"

/// primary entry point into our warehouse program
fn main() {
    //    /// - if you use this, your compiler will auto generate a webpage that will have living documentation for whatever construct you want to provide documentation for.
    //        -the compiler will auto-generate a webpage that will have living documentation for whatever construct you want to provide
    //        -the documentation could be for enums, structsm functions, ...
    //        -we write our documentation commen directly above the line that we want to apply the documentation for
}
