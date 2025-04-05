use rand::Rng;

#[derive(PartialEq, Debug, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let num = rand::thread_rng().gen_range(1..=4);
        Suit::translate(num)
    }

    pub fn translate(value: u8) -> Suit {
        match value{
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid Suit"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let num = rand::thread_rng().gen_range(1..=13);
        Rank::translate(num)
    }

    pub fn translate(value: u8) -> Rank {
        match value{
            1 => Rank::Ace,
            13 => Rank::King,
            12 => Rank::Queen,
            11 => Rank::Jack,
            2..=10 => Rank::Number(value),
            _ => panic!("Invalid rank value"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}