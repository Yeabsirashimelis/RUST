pub mod products;

//the fields imported are the properties of the products module. but importing here and making them public
//    make them accessible in the top level files from the this/ inventory module. no need to go deep down.
pub use products::{Item, ProductCategory};

//in a separate file, we don't need the module key word

pub const FLOOR_SPACE: i32 = 10000;
pub const MANAGER: &str = "Yeabsira Shimelis";

//public struct, but private fields
//we can also have some fields pub and some fields private. so here we need a public associated function/ constructor
//  initialize the private fields a hardcoded value and a dynamic value from the parameter for the publics. we can see this in advanced RUST code bases.

// pub struct Item {
//     name: String,
//     category: ProductCategory,
//     qunatity: u32,
// }

/*
//we can create submodules as inline in other modules
mod products {
    /////
    /////
    ///
}
 */

//the second and third choices are
//inventory/products.rs - file approach
//inventory/products/mod.rs -folder approach

pub fn talk_to_manager() {
    println!(
        "Hey, {}, how is your coffee? what do you think of {:?}",
        MANAGER,
        ProductCategory::Ladder
    );
    //the above was a relative location

    //let's use the crate keyword / use absolute path
    // println!("Hey, {}, how is your coffee?", crate::inventory::MANAGER);

    //this is a realtive path
    // println!("{:?}", products::ProductCategory::Ladder);
}
