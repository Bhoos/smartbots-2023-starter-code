use crate::cards::{self};
pub enum TrumpCase {
    RevealOnly,
    RevealAndThrow(cards::Card),
}
pub enum PlayAction {
    CardThrow(cards::Card),
    Trump(TrumpCase),
}

pub enum Action {
    Hi,
    Bid(u32),
    ChooseTrump(cards::Suit),
    Play(PlayAction),
}
use Action::*;
use PlayAction::*;
use TrumpCase::*;
impl Action {
    pub fn json_format(&self) -> String {
        format!("{}", self)
    }
}
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
