use std::fmt::{self, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)] // to represent Suit using single byte only
pub enum Suit {
    Spade,
    Diamond,
    Club,
    Heart,
}

use Suit::*;
impl Suit {
    pub const SUITS: [Suit; 4] = [Spade, Diamond, Club, Heart];

    pub fn the_suits() -> &'static [Suit; 4] {
        &Self::SUITS
    }

    pub fn as_index(self: Suit) -> usize {
        match self {
            Spade => 0,
            Diamond => 1,
            Club => 2,
            Heart => 3,
        }
    }

    pub fn from_index(index: usize) -> Suit {
        if index < 4 {
            Self::SUITS[index as usize]
        } else {
            Self::SUITS[0]
        }
    }

    pub fn from_string(s: &String) -> Suit {
        Suit::from_char(s.chars().nth(0).unwrap_or('D'))
    }

    pub fn from_char(c: char) -> Suit {
        match c {
            'S' => Spade,
            'D' => Diamond,
            'C' => Club,
            'H' => Heart,
            _ => Heart,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Spade => write!(f, "S"),
            Diamond => write!(f, "D"),
            Club => write!(f, "C"),
            Heart => write!(f, "H"),
            // _       => panic!("Invalid Suit"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
    pub rank: u8,
    pub value: u8,
    pub card: u8,
    pub suit: Suit,
}

#[inline]
fn get_rank_value(card: char) -> (u8, u8) {
    let (rank, value) = match card {
        'J' => (64, 3),
        '9' => (32, 2),
        '1' => (16, 1),
        'T' => (8, 1),
        'K' => (4, 0),
        'Q' => (2, 0),
        '8' => (1, 0),
        '7' => (0, 0),
        'X' => (0, 0),  //dummy face value
        'A' => (16, 1), // equivalent to '1'
        _ => panic!("Invalid face value"),
    };
    (rank, value)
}
impl Card {
    pub fn new(c: &String) -> Card {
        let card: char = c.chars().nth(0).unwrap_or('X');
        let (rank, value) = get_rank_value(card);
        let card: u8 = card as u8;
        let suit = match c.chars().nth(1).unwrap_or('D') {
            'S' => Spade,
            'D' => Diamond,
            'C' => Club,
            'H' => Heart,
            _ => panic!("Invalid suit"),
        };
        Card {
            rank,
            value,
            card,
            suit,
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.card as char, self.suit)
    }
}

pub fn get_card_from_vec(cards_vec: &Vec<String>) -> Vec<Card> {
    cards_vec.iter().map(|card| Card::new(card)).collect()
}

pub fn get_same_suit_cards(cards: &Vec<Card>, suit: Suit) -> Vec<Card> {
    let cards = cards
        .iter()
        .filter(|x| x.suit == suit)
        .map(|&card| card)
        .collect();
    cards
}

pub fn get_same_suit_cards_if_available(cards: &Vec<Card>, suit: Suit) -> Option<Vec<Card>> {
    let cards = get_same_suit_cards(cards, suit);
    if cards.len() == 0 {
        return None;
    } else {
        return Some(cards);
    }
}

pub fn get_deck() -> Vec<Card> {
    // I am doing iteration over cartesian product of two lists
    // after doing
    // `cargo add cute`
    // the following code does the job
    // ```
    //  extern crate cute;
    // use cute::c;
    // let deck = c![
    //     {
    //         let (rank, value) = get_rank_value(c);
    //         let card : u8 = c as u8;
    //         Card {rank, value, card, suit}
    //     }, for c in ['J', '9', '1', 'T', 'K', 'Q', '8', '7'],
    //     for suit in [Heart, Club, Spade, Diamond]
    // ];
    // deck
    // ```
    // However, I prefer another, itertools way

    let deck = itertools::iproduct!(
        ['J', '9', '1', 'T', 'K', 'Q', '8', '7'],
        [Heart, Club, Spade, Diamond]
    )
    .map(|(c, suit)| {
        let (rank, value) = get_rank_value(c);
        let card: u8 = c as u8;
        Card {
            rank,
            value,
            card,
            suit,
        }
    })
    .collect();
    deck
}

#[test]
pub fn show_test() {
    println!("The list of decks are");
    for (i, card) in get_deck().iter().enumerate() {
        println!(
            "   {}\t=> Card : {}{}, rank = {}, \tvalue = {}",
            i + 1,
            card.card as char,
            card.suit,
            card.rank,
            card.value
        );
    }
}
