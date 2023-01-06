use crate::cards::{self};

pub enum Action {
    Hi,                       // game starts with a greeting `hi`
    Bid(u32),                 // to chose  bid value 0 or 16 to 28
    ChooseTrump(cards::Suit), // to choose a trump suit
    Play(PlayAction),         // to play a game
}
pub enum PlayAction {
    CardThrow(cards::Card), // Throw a card
    Trump(TrumpCase),       // Reveal the trump, two cases:
}
pub enum TrumpCase {
    RevealOnly, // I don't know the  trump suit, so I request to reveal it, I cannot throw a card without knowing it.
    RevealAndThrow(cards::Card), // I know the trump suit, so I reveal it and throw a card with it.
}

use Action::*;
use PlayAction::*;
use TrumpCase::*;
impl Action {
    /// a method on Action that converts any action to suitable response in json format for output
    pub fn json_format(&self) -> String {
        format!("{}", self)
    }
}

/// this trait implementation allows us to format the response in json for any purpose, displaying or just formatting
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        match self {
            Hi => write!(f, r#""value":"hello""#)?,
            Bid(bid) => write!(f, "\"bid\":{}", bid)?,
            ChooseTrump(suit) => write!(f, "\"suit\":\"{}\"", suit)?,

            Play(CardThrow(card)) => write!(f, "\"card\":\"{}\"", card)?,
            Play(Trump(trumpcase)) => {
                write!(f, r#""revealTrump":true"#)?;
                match trumpcase {
                    RevealAndThrow(card) => write!(f, ",\"card\":\"{}\"", card)?,
                    RevealOnly => (),
                }
            }
        }
        write!(f, "}}")
    }
}
#[test]
pub fn display_action_jsons() {
    use cards::Card;
    use cards::Suit;
    // these three work
    println!("/hi \n{}", Hi.json_format());
    println!("/bid \n{}", Bid(5).json_format());
    println!("/chooseTrump \n{}", ChooseTrump(Suit::Heart).json_format());
    println!();
    println!(
        "/play \n{}",
        Play(CardThrow(Card::new(&String::from("JH")))).json_format()
    );
    println!("/play \n{}", Play(Trump(RevealOnly)).json_format());
    println!(
        "/play \n{}",
        Play(Trump(RevealAndThrow(Card::new(&String::from("JS"))))).json_format()
    );
}
