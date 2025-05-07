use warehouse::{FLOOR_SPACE, INVENTORY_MANAGER, ORDERS_MANAGER};

//this main function proves that they are independent programs.

/// get a summary of concurrent managers
fn main() {
    println!(
        "our managers are {} and {}. we have {} square feet space.",
        INVENTORY_MANAGER, ORDERS_MANAGER, FLOOR_SPACE
    )
}
