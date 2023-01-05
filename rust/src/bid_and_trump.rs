use crate::payload_types::{BidPayload, ChooseTrumpPayload};
use crate::action_types::Action;
const MIN_BID : u32 = 16;
const PASS    : u32 = 0;


pub fn get_bid(payload : &BidPayload) -> Action {
    let min_bid = if payload.bid_history.len() == 0 {
        MIN_BID
    } else {
        if payload.player_id == payload.bid_state.defender_id {
            payload.bid_state.challenger_bid
        } else {
            payload.bid_state.defender_bid + 1
        }
    };
    if min_bid <= MIN_BID {
        Action::Bid(MIN_BID )
    } else {
        Action::Bid(PASS)
    }
}

pub fn choose_trump(payload: &ChooseTrumpPayload) -> Action {
    for _ in payload.bid_history.iter() {}
    use rand::{thread_rng, prelude::SliceRandom};
    let response = crate::cards::Suit::the_suits().choose(&mut thread_rng()).unwrap();
    Action::ChooseTrump(*response)
}