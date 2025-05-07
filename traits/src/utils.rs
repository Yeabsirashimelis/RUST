// use super::logging::{Accommodation, Description}; //a relative path using the lib.rs /we use a parent module. so super
//the "crate" automatically starts form the crate root/ absolute path
use crate::logging::{Accommodation, Description};

pub fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

pub fn mix_and_match<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_descirption();

    second.book(guest, 1);
}
