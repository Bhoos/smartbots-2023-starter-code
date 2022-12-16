package core

import (
	"errors"

	. "github.com/Bhoos/smartbots-2023-starter-code/go/models"
)

func getBid(bid_request BidRequest) int {
	BID := 0
	/*
		ENTER YOUR LOGIC HERE
	*/
	return BID
}

func selectTrumpSuit(trump_request TrumpRequest) string {
	TRUMP_SUIT := "H"
	/*
		ENTER YOUR LOGIC HERE
	*/
	return TRUMP_SUIT
}

func play(play_request PlayRequest) (string, bool, bool, error) {
	var PLAY_CARD string // the final card to play
	var current_suite string
	var trump_cards []string
	reveal_trump := false

	player_cards := play_request.Cards // the player's own cards
	hand := play_request.Played        // cards in the current hand

	if len(hand) == 0 {
		PLAY_CARD = player_cards[0] // play the first card from the deck if this is the first move
		return PLAY_CARD, false, false, nil
	}
	current_suite = string(hand[0][1])

	suite_cards_available, suite_cards := SuiteCards(player_cards, current_suite)

	if play_request.TrumpRevealed != false && len(suite_cards) == 0 {
		trump_suite_obj := play_request.TrumpSuit

		if trump_suite, ok := trump_suite_obj.(string); ok {
			_, trump_cards = SuiteCards(player_cards, trump_suite)
		} else {
			return "", false, false, errors.New("Trump card does not contain a suite in request")
		}

		if len(trump_cards) != 0 {
			PLAY_CARD = trump_cards[0] // play first trump card if player has it
		} else {
			PLAY_CARD = player_cards[0] // play first card if no trump card
		}
	} else if play_request.TrumpRevealed == false && suite_cards_available {
		PLAY_CARD = suite_cards[0] // Always play the first card of the suite for now
	} else if play_request.TrumpRevealed == false && !suite_cards_available {
		reveal_trump = true
	} else if suite_cards_available {
		PLAY_CARD = suite_cards[0] // play the first card of the hand's suite
	} else {
		PLAY_CARD = player_cards[0] // play the first card available
	}

	trump_player_id, err := TrumpPlayerId(play_request.BidHistory)
	if err != nil {
		return "", false, false, err
	}

	play_trump := play_request.PlayerId == trump_player_id

	if reveal_trump && play_trump {
		PLAY_CARD = trump_cards[0] // Play first trump card if the option to play the trump is available
	}
	return PLAY_CARD, reveal_trump, play_trump, nil
}
