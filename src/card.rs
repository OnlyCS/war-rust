use rand::{random, seq::SliceRandom, thread_rng};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub trait ValueIDGen {
    fn gen_value_by_id(id: i32) -> Result<String, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct Card {
    pub suit: String,
    pub suit_id: i32,
    pub value: String,
    pub value_id: i32,
    pub visible: bool,
}

impl Card {
    pub fn new(suit_id: i32, value_id: i32) -> Result<Card, Box<dyn Error>> {
        let mut card = Card {
            suit: String::new(),
            suit_id,
            value: String::new(),
            value_id,
            visible: true,
        };

        card.refresh().unwrap();
        Ok(card)
    }

    pub fn new_random() -> Result<Card, Box<dyn Error>> {
        let suit_id = random::<i32>() % 4;
        let value_id = random::<i32>() % 13;
        Card::new(suit_id, value_id)
    }

    pub fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        self.suit = match self.suit_id {
            0 => "Spades",
            1 => "Hearts",
            2 => "Clubs",
            3 => "Diamonds",
            _ => return Err("E_INVALID_SUIT".into()),
        }
        .to_string();

        self.value = match self.value_id {
            -1 => "Ace (as 1)",
            0 => "2",
            1 => "3",
            2 => "4",
            3 => "5",
            4 => "6",
            5 => "7",
            6 => "8",
            7 => "9",
            8 => "10",
            9 => "Jack",
            10 => "Queen",
            11 => "King",
            12 => "Ace",
            _ => return Err("E_INVALID_VALUE".into()),
        }
        .to_string();

        Ok(())
    }

    pub fn new_deck() -> Vec<Card> {
        let mut deck = Vec::new();

        for i in 0..4 {
            for j in 0..13 {
                deck.push(Card::new(i, j).unwrap());
            }
        }

        deck
    }

    pub fn new_random_deck() -> Vec<Card> {
        let mut deck = Card::new_deck();
        deck.shuffle(&mut thread_rng());

        deck
    }

    pub fn copy(&self) -> Card {
        Card {
            suit: self.suit.clone(),
            suit_id: self.suit_id,
            value: self.value.clone(),
            value_id: self.value_id,
            visible: self.visible,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}
