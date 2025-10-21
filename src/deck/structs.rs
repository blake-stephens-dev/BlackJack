use crate::deck::enums::{Rank, Suit};

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}
