use crate::deck::{
    enums::GameState,
    structs::{Deck, Hand},
};

mod deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    let mut player_hand = Hand { cards: vec![] };
    let mut dealer_hand = Hand { cards: vec![] };

    let mut state = GameState::RoundStart;

    loop {
        match state {
            GameState::RoundStart => {
                while dealer_hand.cards.len() < 2 {
                    deck.deal(&mut player_hand, 1);
                    deck.deal(&mut dealer_hand, 1);
                }
                state = GameState::PlayerTurn;
            }
            GameState::PlayerTurn => {
                let player_value = player_hand.value();

                if player_value == 21 {
                    println!("PLAYER BLACKJACK");
                    state = GameState::RoundEnd;
                } else if player_value < 16 {
                    println!("Player Hit on {}", player_value);
                    deck.deal(&mut player_hand, 1);
                } else if player_value >= 16 && player_value < 21 {
                    println!("Player Stand on {}", player_value);
                    state = GameState::DealerTurn;
                } else {
                    println!("PLAYER BUST");
                    state = GameState::RoundEnd;
                }
            }
            GameState::DealerTurn => {
                let dealer_value = dealer_hand.value();
                if dealer_value < 17 {
                    println!("Dealer Hit on {}", dealer_hand.value());
                    deck.deal(&mut dealer_hand, 1);
                } else {
                    println!("Dealer stands on {}", dealer_hand.value());
                    state = GameState::RoundEnd;
                }
            }
            GameState::RoundEnd => {
                let player_val = player_hand.value();
                let dealer_val = dealer_hand.value();

                if player_val > 21 {
                    println!("DEALER WINS (player bust)");
                } else if dealer_val > 21 {
                    println!("PLAYER WINS (dealer bust)");
                } else if player_val > dealer_val {
                    println!("PLAYER WINS");
                } else if dealer_val > player_val {
                    println!("DEALER WINS");
                } else {
                    println!("PUSH");
                }
                break;
            }
        }
    }
}
