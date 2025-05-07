//now we move the trait and structs to a separate file.

use std::{collections::HashMap, fmt::Display, vec};
pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32); //we don't make the method defns public. the visibility of them is defined by the visiblity of the trait
}

pub trait Description {
    fn get_descirption(&self) -> String {
        String::from("a wonderful place to stay")
    }
}

#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}

impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
        Self {
            name,
            reservations: HashMap::new(),
        }
    }
}

impl<T: Display> Hotel<T> {
    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_descirption())
    }
}

impl<T> Accommodation for Hotel<T> {
    //no need to use public for TRAIT methos as "trait items always share the visibility of their trait"
    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

impl<T> Description for Hotel<T> {}

#[derive(Debug)]
pub struct AirBnb {
    host: String,
    guests: Vec<(String, u32)>,
}

impl AirBnb {
    pub fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
}

impl Accommodation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnb {
    fn get_descirption(&self) -> String {
        format!("please enjoy {}'s apartment.", self.host)
    }
}
