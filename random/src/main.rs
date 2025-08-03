/*
use rand::random;

//INTRO INTO THE RAND CRATE
fn main() {
    //generate a random floating point between 0 and 1
    // let random_float: f64 = rand::random();
    let random_float = random::<f64>(); //using turbofish operator
    println!("{}", random_float * 100.0); //will be b/n 0-100

    let random_int = random::<u8>(); //smaller range - u8
    println!("{}", random_int);

    let random_int = random::<u32>(); //very large range - u32
    println!("{}", random_int);
}
 */

/*
//////////////////////////////////////
//////////////////////////////////////
//THE THREADRNG STRUCT
/*
RNG - random number generator

The ThreadRng struct in Rust is part of the rand crate and represents a random number generator (RNG)
  local to the current thread. It's fast, automatically seeded, and safe to use in multi-threaded programs
   because each thread gets its own instance.
*/

use rand::{rng, Rng};
fn main() {
    let mut my_rng = rng();
    let random_float = my_rng.random::<f64>();
    println!("{}", random_float);

    let ten_random_values = (0..10).map(|_| my_rng.random::<i8>()).collect::<Vec<i8>>();
    println!("{:?}", ten_random_values);

    let random_number: i32 = my_rng.random_range(29..53);
    println!("{}", random_number);

    let random_bool = my_rng.random_bool(0.5); //50% chance of getting true
    println!("{}", random_bool);

    println!("{}", my_rng.random_ratio(1, 2)); // returns `true` 50% of the time(1/2)
}
 */

/*
/////////////////////////////////////////////////////
//////////////////////////////////////////////////
//RANDOMIZING VECTOR ELEMENTS WITH SHUFFLE METHOD

use std::vec;
use rand::{rng, seq::SliceRandom, Rng};

fn main() {
    let mut my_rng = rng();
    let mut candies = vec!["yeabsira", "shimelis", "Abiyot", "Dagim"];
    candies.shuffle(&mut my_rng);

    println!("{:?}", candies);
}
 */

use rand::{rng, seq::SliceRandom, Rng};

#[derive(Debug, Copy, Clone)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Copy, Clone)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Option<Suit>,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        let mut cards = Vec::with_capacity(52);

        for suit in suits.into_iter() {
            for rank in ranks.into_iter() {
                cards.push(Card {
                    suit: Some(suit),
                    rank,
                });
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut my_rng = rng();
        self.cards.shuffle(&mut my_rng);
    }

    fn insert_jokers(&mut self) {
        let mut my_rng = rng();
        for _ in 0..2 {
            let random_index = my_rng.random_range(0..self.cards.len());
            let joker_card = Card {
                suit: None,
                rank: Rank::Joker,
            };

            self.cards.insert(random_index, joker_card)
        }
    }

    fn delete_random_card(&mut self) {
        let mut my_rng = rng();
        let should_delete_card = my_rng.random_bool(0.65);
        if (should_delete_card) {
            let random_index = my_rng.random_range(0..self.cards.len());
            self.cards.remove(random_index);
        }
    }
}
fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    deck.insert_jokers();

    for _ in 0..10 {
        deck.delete_random_card();
    }

    println!("{:#?}", deck.cards.len());
}
