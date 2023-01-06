use std::fmt::{self, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)] // to represent Suit using single byte only
pub enum Suit {
    // 4 possible suit values
    Spade,
    Diamond,
    Club,
    Heart,
}

use Suit::*;
impl Suit {
    pub const SUITS: [Suit; 4] = [Spade, Diamond, Club, Heart];

    /// reference to Constant array of suits, useful
    pub const fn the_suits() -> &'static [Suit; 4] {
        &Self::SUITS
    }

    /// converts Suit to numeric value
    pub fn as_index(self: Suit) -> usize {
        match self {
            Spade => 0,
            Diamond => 1,
            Club => 2,
            Heart => 3,
        }
    }

    /// converts index (numeric value) to suits
    pub fn from_index(index: usize) -> Suit {
        if index < 4 {
            Self::SUITS[index as usize]
        } else {
            Self::SUITS[0]
        }
    }

    /// converts string to suit value
    pub fn from_string(s: &String) -> Suit {
        Suit::from_char(s.chars().nth(0).unwrap_or('D'))
    }

    /// converts character to suit value
    pub fn from_char(c: char) -> Suit {
        match c.to_ascii_uppercase() {
            'S' => Spade,
            'D' => Diamond,
            'C' => Club,
            'H' => Heart,
            _ => Heart,
        }
    }
}

impl fmt::Display for Suit {
    // allows us to print the suit
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
    pub rank: u8,   // rank of card, used to compare the cards
    pub value: u8,  // value of card, 3, 2, 1, 1, 0, 0 ...
    pub card: u8,   // character value that represents cards, J, 9, 1, T, K, Q , ...
    pub suit: Suit, // suit value that represents suit of the card, Spade, Diamond, ...
}

#[inline]
/// a function that gets rank and value of card from character value
fn get_rank_value(card: char) -> (u8, u8) {
    let (rank, value) = match card.to_ascii_uppercase() {
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
        _ => (0, 0),
    };
    (rank, value)
}
impl Card {
    /// a function that gets a new Card struct based on the card string like, "JH", "9C", ...
    pub fn new(c: &String) -> Card {
        let card: char = c.chars().nth(0).unwrap_or('X');
        let (rank, value) = get_rank_value(card);
        let card: u8 = if card == 'A' { b'1' } else { card as u8 };
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
    // allows us to print the card
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.card as char, self.suit)
    }
}

/// a function that convert Vector of String for card to Vector of Cards
pub fn get_card_from_vec(cards_vec: &Vec<String>) -> Vec<Card> {
    cards_vec.iter().map(|card| Card::new(card)).collect()
}

/// a function that separates out cards of same suit from given Vector<Card> and give Suit.
pub fn get_same_suit_cards(cards: &Vec<Card>, suit: Suit) -> Vec<Card> {
    let cards = cards
        .iter()
        .filter(|x| x.suit == suit)
        .map(|&card| card)
        .collect();
    cards
}

/// a function that returns option representing Some(non_empty_vector_of_cards) or None, when same suit cards are none.
pub fn get_same_suit_cards_if_available(cards: &Vec<Card>, suit: Suit) -> Option<Vec<Card>> {
    let cards = get_same_suit_cards(cards, suit);
    if cards.len() == 0 {
        return None;
    } else {
        return Some(cards);
    }
}

/// a function to get list of all cards
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
        ['J', '9', '1', 'T', 'K', 'Q', '8', '7']
            .iter()
            .map(|&c| (get_rank_value(c), c as u8)),
        [Heart, Club, Spade, Diamond]
    )
    .map(|(((rank, value), card), suit)| Card {
        rank,
        value,
        card,
        suit,
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
