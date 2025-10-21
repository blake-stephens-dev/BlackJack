#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Rank {
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameState {
    RoundStart,
    PlayerTurn,
    DealerTurn,
    RoundEnd,
}
