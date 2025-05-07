// this will create prefield items for datatypes it is implemented
use fake::Dummy;

/// A category of producst the our business sells
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}

/// A concrete item in stock with our warehouse
#[derive(Debug, Dummy)]

//public struct, but private fields
//we can also have some fields pub and some fields private. so here we need a public associated function/ constructor
//  initialize the private fields a hardcoded value and a dynamic value from the parameter for the publics. we can see this in advanced RUST code bases.

// pub struct Item {
//     name: String,
//     category: ProductCategory,
//     qunatity: u32,
// }

/*
THE SUPER KEYWORD - in opposite to the mod and use keywords,
it is used to access data from the parent/ super modules./ above heirarchy
*/

pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quantity: u32,
}

impl Item {
    // Create a new item
    pub fn new(name: String, category: ProductCategory, quantity: u32) -> Self {
        // this will go to the direct parent / the inventory module
        //and to access the fields, it is not neccessary to make them public
        //b/c a this file is already contained in the inventory modules, with that
        //  we can say inner scopes have access to values from outer scopes and outerscopes don't have access to values from inner scopes directly
        super::talk_to_manager();
        Self {
            name,
            category,
            quantity,
        }
    }
}
