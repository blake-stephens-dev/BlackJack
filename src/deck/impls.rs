use rand::rng;
use rand::seq::SliceRandom;

use crate::deck::enums::Rank;
use crate::deck::enums::Rank::{
    Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two,
};
use crate::deck::enums::Suit::{Club, Diamond, Heart, Spade};
use crate::deck::structs::{Card, Deck, Hand};

impl Hand {
    pub fn value(&self) -> u8 {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            total += card.rank.value();
            if let Rank::Ace = card.rank {
                aces += 1;
            }
        }

        // Adjust Aces from 11 → 1 if total > 21
        while total > 21 && aces > 0 {
            total -= 10;
            aces -= 1;
        }

        total
    }
}

impl Deck {
    pub fn new() -> Self {
        let suits = [Heart, Spade, Diamond, Club];
        let ranks = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];

        let mut cards = Vec::new();
        for &suit in &suits {
            for &rank in &ranks {
                cards.push(Card { suit, rank });
            }
        }

        Deck { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self, hand: &mut Hand, count: usize) {
        for _ in 0..count {
            if let Some(card) = self.cards.pop() {
                hand.cards.push(card);
            }
        }
    }
}

impl Rank {
    pub fn value(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
            Rank::Ace => 11, // default value; logic can adjust this later
        }
    }
}
