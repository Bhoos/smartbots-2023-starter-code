use crate::payload_types::{self, TrumpRevealed, TrumpRevealer};
use crate::action_types::{Action, PlayAction, TrumpCase};
use crate::cards::{self, Card, Suit};
pub fn make_move(payload : &payload_types::PlayPayload ) -> Action {
    if payload.played.len() == 0 {
        return Action::Play(PlayAction::CardThrow(Card::new(payload.cards.last().unwrap())));
    }
    let first_card = Card::new(&payload.played[0]);
    let my_cards = cards::get_card_from_vec(&payload.cards);
    let same_suit_cards = cards::get_same_suit_cards_if_available(&my_cards, first_card.suit);
    if let Some(card_choices) = same_suit_cards { // I have same suit card, must throw that
        return Action::Play(PlayAction::CardThrow(*card_choices.last().unwrap()));
    } 
    
    use payload_types::TrumpSuit;
    // I don't have same suit card.
    match (&payload.trump_suit, &payload.trump_revealed) {
        (TrumpSuit( Some(suit) ), trump_revelation)  => { // Trump suit I know, (revealed or me-bidder)
            let trump_suit_cards = cards::get_same_suit_cards_if_available(&my_cards, Suit::from_string(&suit));
            
            if let TrumpRevealed( Some(trump_revealer)) = trump_revelation { // Trump revealed in game
                let TrumpRevealer {player_id, hand} = trump_revealer;
                let trump_revealer = player_id;
                let &hand = hand;
                // handling when trump has been revealed already:
                let trump_revealed_this_round =  hand == (payload.hands_history.len()+1) ;
                let i_revealed_trump = payload.player_id == *trump_revealer;
                
                if trump_revealed_this_round && i_revealed_trump { // I am not a bidder
                    if let Some(trump_suit_cards) = trump_suit_cards { // i throw best card
                        return Action::Play(PlayAction::CardThrow(*trump_suit_cards.last().unwrap()));
                    } else { // i throw any card
                        return Action::Play(PlayAction::CardThrow(*my_cards.last().unwrap()));
                    }
                } else if trump_revealed_this_round {
                    if let Some(trump_suit_cards) = trump_suit_cards { // i throw best card if wins
                        return Action::Play(PlayAction::CardThrow(*trump_suit_cards.last().unwrap()));
                    } else { // i throw any card
                        return Action::Play(PlayAction::CardThrow(*my_cards.last().unwrap()));
                    }
                } else {
                    if let Some(trump_suit_cards) = trump_suit_cards { // i throw best card if wins
                        return Action::Play(PlayAction::CardThrow(*trump_suit_cards.last().unwrap()));
                    } else { // i throw any card
                        return Action::Play(PlayAction::CardThrow(*my_cards.last().unwrap()));
                    }
                }
            } else { // I know the trump card, and it is not revealed
                if let Some(trump_suit_cards) = trump_suit_cards { // i reveal and thow best card
                    return Action::Play(PlayAction::Trump(TrumpCase::RevealAndThrow(*trump_suit_cards.last().unwrap())));
                } else  { // i throw any card
                    return Action::Play(PlayAction::CardThrow(*my_cards.last().unwrap())); 
                }
            }
        },
        ( TrumpSuit(None), _) => { // I don't know the unrevealed trump suit, maybe I can just win by revealing it
            return Action::Play(PlayAction::Trump(TrumpCase::RevealOnly))
        }
    }
    
}