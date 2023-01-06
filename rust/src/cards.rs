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
            Self::SUITS[index]
        } else {
            Self::SUITS[0]
        }
    }

    /// converts string to suit value
    pub fn from_string(s: &str) -> Suit {
        Suit::from_char(s.chars().next().unwrap_or('D'))
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

#[derive(Copy, Clone, Debug)]
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
    pub fn new(c: &str) -> Card {
        let card: char = c.chars().next().unwrap_or('X');
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

    pub fn cmp_using_ongoing_and_trump_suit(
        self,
        other: Self,
        ongoing_suit: Suit,
        trump_suit: Suit,
    ) -> std::cmp::Ordering {
        // both same suit
        if self.suit == other.suit {
            return self.rank.cmp(&other.rank);
        }
        // both different suit
        if self.suit == trump_suit {
            return std::cmp::Ordering::Greater;
        }
        if other.suit == trump_suit {
            return std::cmp::Ordering::Less;
        }
        // both are not trump cards
        if self.suit == ongoing_suit {
            return std::cmp::Ordering::Greater;
        }
        
        return std::cmp::Ordering::Less;
        
        // both are not ongoing suits
        // return self.rank.cmp(&other.rank);
    }
}
impl fmt::Display for Card {
    // allows us to print the card
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.card as char, self.suit)
    }
}

/*impl core::cmp::Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}
impl Eq for Card {}
impl core::cmp::PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl core::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}*/

/// a function that convert Vector of String for card to Vector of Cards
pub fn get_card_from_vec(cards_vec: &[String]) -> Vec<Card> {
    cards_vec.iter().map(|card| Card::new(card)).collect()
}

/// a function that separates out cards of same suit from given Vector<Card> and give Suit.
pub fn get_same_suit_cards(cards: &[Card], suit: Suit) -> Vec<Card> {
    let cards = cards
        .iter()
        .filter(|x| x.suit == suit)
        .copied()
        .collect();
    cards
}

#[allow(dead_code)]
/// a function that gets only higher ranked cards, probably bad function
fn get_higher_ranked_cards_any_suit(cards_vec: &[Card], the_card: Card) -> Vec<Card> {
    cards_vec
        .iter()
        .filter(|&&a_card| a_card.rank > the_card.rank)
        .copied()
        .collect()
}
#[allow(dead_code)]
/// a function that gets higher ranked cards if available, better version of bad function
fn get_higher_ranked_cards_any_suit_if_available(
    cards: &[Card],
    card: Card,
) -> Option<Vec<Card>> {
    let cards = get_higher_ranked_cards_any_suit(cards, card);
    if cards.is_empty() {
        None
    } else {
        Some(cards)
    }
}
#[allow(dead_code)]
/// a function that gets maximum card by rank, probably bad function
fn get_max_card_of_any_suit(cards_vec: &[Card]) -> Option<Card> {
    cards_vec
        .iter()
        .copied()
        .max_by_key(|card| card.rank)
}

/// a function that gets higher ranked cards compared to cards, factoring the ongoing suit and trump suit
pub fn get_higher_rank_cards(
    cards_vec: &[Card],
    the_card: Card,
    ongoing_suit: Suit,
    trump_suit: Suit,
) -> Vec<Card> {
    cards_vec
        .iter()
        .filter(|&&a_card| {
            a_card.cmp_using_ongoing_and_trump_suit(the_card, ongoing_suit, trump_suit)
                == std::cmp::Ordering::Greater
        })
        .copied()
        .collect()
}

/// a function that returns Option of available higher cards, factoring the ongoing suit and trump suit
pub fn get_higher_rank_cards_if_available(
    cards: &[Card],
    card: Card,
    ongoing_suit: Suit,
    trump_suit: Suit,
) -> Option<Vec<Card>> {
    let cards = get_higher_rank_cards(cards, card, ongoing_suit, trump_suit);
    if cards.is_empty() {
         None
    } else {
        Some(cards)
    }
}

pub fn get_max_card(cards_vec: &[Card], ongoing_suit: Suit, trump_suit: Suit) -> Option<Card> {
    cards_vec
        .iter()
        .copied()
        .max_by(|&card_a, &card_b| {
            card_a.cmp_using_ongoing_and_trump_suit(card_b, ongoing_suit, trump_suit)
        })
}

/// a function that returns option representing Some(non_empty_vector_of_cards) or None, when same suit cards are none.
pub fn get_same_suit_cards_if_available(cards: &[Card], suit: Suit) -> Option<Vec<Card>> {
    let cards = get_same_suit_cards(cards, suit);
    if cards.is_empty() {
        None
    } else {
        Some(cards)
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
