use traits::logging::{Accommodation, AirBnb, Description, Hotel};
use traits::utils;

fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.summarize());
    hotel.book("Dana", 5);

    let mut airbnb = AirBnb::new("Parker");
    println!("{}", airbnb.get_descirption());
    utils::book_for_one_night(&mut airbnb, "Dan");

    utils::mix_and_match(&mut hotel, &mut airbnb, "Phil");
}
