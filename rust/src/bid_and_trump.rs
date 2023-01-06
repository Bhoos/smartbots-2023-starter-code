use crate::action_types::Action;
use crate::payload_types::{BidPayload, ChooseTrumpPayload};
const MIN_BID: u32 = 16;
const PASS: u32 = 0;

// a function to choose bid
pub fn get_bid(payload: &BidPayload) -> Action {
    let min_bid = if payload.bid_history.is_empty() {
        // if no one bid, yet, bid 16
        MIN_BID
    } else {
        // if someone else bid already (maybe 0 when passed or some 16, 17, .. values)
        if payload.player_id == payload.bid_state.defender_id {
            // I can match when I am defender
            payload.bid_state.challenger_bid
        } else {
            // I have to exceed when I am challenger
            payload.bid_state.defender_bid + 1
        }
    };

    if min_bid <= MIN_BID {
        // I cannot bid less than 16
        Action::Bid(MIN_BID)
    } else {
        // In starter code, I will not bid more than 16
        Action::Bid(PASS)
    }
}

// a function to choose trump suit
pub fn choose_trump(payload: &ChooseTrumpPayload) -> Action {
    for _ in payload.bid_history.iter() {} // dummy loop to silence the unused payload warning
    use rand::{prelude::SliceRandom, thread_rng};
    // choosing a random suit for starter code
    let response = crate::cards::Suit::the_suits()
        .choose(&mut thread_rng())
        .unwrap();
    Action::ChooseTrump(*response)
}
