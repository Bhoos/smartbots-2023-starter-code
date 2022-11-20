const MIN_BID: i32 = 16;
const PASS: i32 = 0;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BidPayload {
    pub player_id: String,
    pub player_ids: Vec<String>,
    pub cards: Vec<String>,
    pub time_remaining: i64,
    pub bid_history: Vec<(String, i64)>,
    pub bid_state: BidState,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BidState {
    pub defender_id: String,
    pub challenger_id: String,
    pub defender_bid: i64,
    pub challenger_bid: i64,
}

pub fn get_bid(payload: &BidPayload) -> i32 {
    if payload.bid_history.len() == 0 {
        return MIN_BID;
    }
    PASS
}
