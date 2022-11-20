use crate::cards::Card;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrumpSuitEnum {
    Card(String),
    NotShown(bool),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrumpSuitRevealedEnum {
    Revealed(TrumpRevealer),
    NotRevealed(bool),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrumpRevealer {
    player_id: String,
    hand: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayPayload {
    pub player_id: String,
    pub player_ids: Vec<String>,
    pub time_remaining: i64,
    pub teams: Vec<Team>,
    pub cards: Vec<String>,
    pub bid_history: Vec<(String, i64)>,
    pub played: Vec<String>,
    pub hands_history: Vec<(String, Vec<String>, String)>,
    pub trump_suit: TrumpSuitEnum,
    pub trump_revealed: TrumpSuitRevealedEnum,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub players: Vec<String>,
    pub bid: i64,
    pub won: i64,
}

fn move_json_response(card: &String) -> String {
    format!("{{\"card\":\"{}\"}}", card)
}
fn reveal_trump_and_move_json_response(c: &String) -> String {
    r#"{"revealTrump": true,"card":""#.to_string() + c + r#""}"#
}

pub fn get_move(payload: &PlayPayload) -> String {
    // println!("{:#?}", payload);

    if payload.played.len() == 0 {
        return move_json_response(&payload.cards.last().unwrap().to_string());
    }
    let first_card = Card::new((&payload.played[0]).clone());
    let my_cards = Card::get_card_from_vec(payload.cards.clone());
    let same_suit_cards = Card::get_same_suit_cards(my_cards.clone(), first_card.suit);
    // log to terminal
    // Card::log_vec_cards(same_suit_cards.clone());

    if same_suit_cards.len() > 0 {
        return move_json_response(&same_suit_cards.last().unwrap().to_string());
    }

    match (payload.trump_suit.clone(), payload.trump_revealed.clone()) {
        (TrumpSuitEnum::Card(trump_suit), TrumpSuitRevealedEnum::Revealed(revealer)) => {
            let trump_revealed_this_round = revealer.hand == payload.hands_history.len() as i32 + 1;
            let did_reveal_trump = revealer.player_id == payload.player_id;

            if trump_revealed_this_round && did_reveal_trump {
                let trump_suit_cards = Card::get_same_suit_cards(
                    my_cards.clone(),
                    Card::new("X".to_string() + &trump_suit).suit,
                );
                match trump_suit_cards.last() {
                    None => {}
                    Some(c) => return move_json_response(&c.to_string()),
                }
            }
            match my_cards.last() {
                None => {}
                Some(c) => return move_json_response(&c.to_string()),
            }
        }
        (TrumpSuitEnum::Card(trump_suit), TrumpSuitRevealedEnum::NotRevealed(_)) => {
            let trump_suit_cards = Card::get_same_suit_cards(
                my_cards.clone(),
                Card::new("X".to_string() + &trump_suit).suit,
            );
            match trump_suit_cards.last() {
                None => {}
                Some(c) => {
                    return reveal_trump_and_move_json_response(&c.to_string());
                }
            }
            match my_cards.last() {
                None => {}
                Some(c) => {
                    return reveal_trump_and_move_json_response(&c.to_string());
                }
            }
        }

        (_, _) => {}
    }

    r#"{"revealTrump": true}"#.to_string()
}
