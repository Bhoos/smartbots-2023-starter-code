use crate::action_types::{Action, PlayAction, TrumpCase};
use crate::cards::{
    get_higher_rank_cards, get_higher_rank_cards_if_available, get_max_card, get_same_suit_cards,
    get_same_suit_cards_if_available,
};

// use Action::Play;
use PlayAction::CardThrow;
use PlayAction::Trump;
use TrumpCase::RevealAndThrow;
use TrumpCase::RevealOnly;

use crate::cards::{self, Card, Suit};
use crate::payload_types::{self, TrumpRevealed, TrumpRevealer};

/// This is the main function that makes the move.
/// Even for starter code, it is quite complex because it has to analyze
/// which card it can throw and only throw from the possible choices.
pub fn make_move(payload: &payload_types::PlayPayload) -> Action {
    // Case I: If no card is played, I can throw any card, no restriction
    if payload.played.is_empty() {
        return Action::Play(CardThrow(Card::new(payload.cards.last().unwrap())));
    }

    // Case II: If someone has already played, I have to match the suit if available.
    let my_cards = cards::get_card_from_vec(&payload.cards);

    let this_hand = cards::get_card_from_vec(&payload.played);
    let ongoing_suit = this_hand[0].suit;
    // matching the suits.
    let same_suit_cards = cards::get_same_suit_cards_if_available(&my_cards, ongoing_suit);

    // If I have cards matching the suit, I must throw that
    if let Some(card_choices) = same_suit_cards {
        return Action::Play(CardThrow(*card_choices.last().unwrap()));
    }

    // Case III: I don't have any card matching the suit.
    use payload_types::TrumpSuit;

    // Case III a: I don't know the trump suit. I am probably not the bidder then, haha.
    if let TrumpSuit(None) = payload.trump_suit {
        // best action may be just to reveal the trump
        return Action::Play(Trump(RevealOnly));
    }

    // Case III b: I know the trump suit, Someone Revealed it or I am the bidder, soon to be known
    if let TrumpSuit(Some(trump_suit)) = &payload.trump_suit {
        let trump_suit = Suit::from_string(trump_suit);

        let max_in_this_hand = get_max_card(&this_hand, ongoing_suit, trump_suit);
        let winning_cards = if let Some(max_in_this_hand) = max_in_this_hand {
            get_higher_rank_cards_if_available(
                &my_cards,
                max_in_this_hand,
                ongoing_suit,
                trump_suit,
            )
        } else {
            None
        };

        let trump_suit_cards = if let Some(cards) = winning_cards {
            get_same_suit_cards_if_available(&cards, trump_suit)
        } else {
            get_same_suit_cards_if_available(&my_cards, trump_suit)
        };

        // Knowing trump is useful only when I have trump cards
        if let Some(trump_suit_cards) = trump_suit_cards {
            // Case III b A: I know and have trump cards, Trump is not revealed.
            if let TrumpRevealed(None) = &payload.trump_revealed {
                return Action::Play(Trump(RevealAndThrow(*trump_suit_cards.last().unwrap())));
            }

            // Case III b B: I know trump card, which is already revealed. I can throw trump card or any card depending on game.
            if let TrumpRevealed(Some::<TrumpRevealer>(trump_revealer)) = &payload.trump_revealed {
                let trump_revealed_in_this_hand =
                    payload.hands_history.len() + 1 == trump_revealer.hand;
                let i_revealed_trump_in_this_hand =
                    trump_revealed_in_this_hand && (payload.player_id == trump_revealer.player_id);

                if !i_revealed_trump_in_this_hand || !trump_revealed_in_this_hand {
                    // If it is not me who revealed trump in this round, I can throw any card
                    // Or it is not this round when trump was revealed, I can throw any card
                    // return Action::Play(CardThrow(*my_cards.last().unwrap()));
                    return Action::Play(CardThrow(*trump_suit_cards.last().unwrap()));
                }
                return Action::Play(CardThrow(*trump_suit_cards.last().unwrap()));
            }
        }
    }

    return Action::Play(CardThrow(*my_cards.last().unwrap()));
}
