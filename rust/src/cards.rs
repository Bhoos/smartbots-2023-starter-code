use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum Suit {
    Heart,
    Spade,
    Club,
    Diamond,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Spade => write!(f, "S"),
            Suit::Heart => write!(f, "H"),
            Suit::Club => write!(f, "C"),
            Suit::Diamond => write!(f, "D"),
            _ => panic!("Invalid suit"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Card {
    pub rank: i32,
    pub value: i32,
    pub suit: Suit,
}
impl Card {
    pub fn new(c: String) -> Self {
        // Rank of card in order by point value , face value
        let (rank, value) = match c.chars().nth(0).unwrap() {
            'J' => (8, 3),
            '9' => (7, 2),
            'T' => (6, 1),
            '1' => (5, 1),
            'K' => (4, 0),
            'Q' => (3, 0),
            '8' => (2, 0),
            '7' => (1, 0),
            'X' => (0, 0), //dummy face value
            _ => panic!("Invalid face value"),
        };
        let suit = match c.chars().nth(1).unwrap() {
            'H' => Suit::Heart,
            'C' => Suit::Club,
            'S' => Suit::Spade,
            'D' => Suit::Diamond,
            _ => panic!("Invalid suit"),
        };

        Card { rank, value, suit }
    }

    pub fn get_card_from_vec(cards_vec: Vec<String>) -> Vec<Card> {
        let mut cards = vec![];

        for c in cards_vec {
            cards.push(Card::new(c));
        }

        cards
    }

    pub fn log_vec_cards(cards: Vec<Card>) {
        for c in &cards {
            print!("{}  ", c);
        }
        println!("");
    }

    pub fn get_same_suit_cards(cards: Vec<Card>, suit: Suit) -> Vec<Card> {
        cards.into_iter().filter(|x| (x).suit == suit).collect()
    }

    pub fn get_deck() -> Vec<Card> {
        let mut deck = vec![];
        for i in ['K', 'Q', 'J', 'T', '9', '8', '7', '1'] {
            for j in [Suit::Club, Suit::Heart, Suit::Diamond, Suit::Spade] {
                deck.push(Card::new(format!("{}{}", i, j)));
            }
        }

        return deck;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            ['J', '9', 'T', '1', 'K', 'Q', '8', '7'][8 - self.rank as usize],
            self.suit
        )
    }
}
