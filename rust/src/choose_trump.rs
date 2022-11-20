use crate::cards::Suit;
use actix_web::Result;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChooseTrumpPayload {
    pub player_id: String,
    pub player_ids: Vec<String>,
    pub cards: Vec<String>,
    pub time_remaining: i64,
    pub bid_history: Vec<(String, i64)>,
}

pub fn select_trump(payload: &ChooseTrumpPayload) -> String {
    let suits = [Suit::Club, Suit::Heart, Suit::Diamond, Suit::Spade];
    suits.choose(&mut thread_rng());
    format!("{}", suits.choose(&mut thread_rng()).unwrap())
}
