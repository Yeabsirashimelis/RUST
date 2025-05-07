//is the CRATE ROOT of the WAREHOUSE LIBRARY CRATE.
//the name of this library crate is the name in the cargo.toml file.
// which is "warehouse"

//here we commented out the imports from our modules in the main.rs and move
// them to the library crate file. so now we need to import them in the main.rs file
//    from the library crate.

/// Tools for inventory management
pub mod inventory;

///Tools for order manager
pub mod orders;

//pub key word reexport the fields in the inventory and orders modules
//therefore the fields are directy accessible from warehouse::{...} in main.rs
pub use inventory::{Item, ProductCategory, FLOOR_SPACE, MANAGER as INVENTORY_MANAGER};
pub use orders::MANAGER as ORDERS_MANAGER;
