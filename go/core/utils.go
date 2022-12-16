package core

import (
	"errors"
	"math"
)

// Returns the set of cards of a particular suite from the deck
func SuiteCards(deck []string, suite string) (bool, []string) {
	suite_cards := []string{}
	for _, card := range deck {
		card_suite := string(card[1])
		if card_suite == suite {
			suite_cards = append(suite_cards, card)
		}
	}
	return len(suite_cards) != 0, suite_cards
}

// algorithm to find the suite with the max value in the deck, currently always selects first suite
func MaxValuedSuite(deck []string) string {
	MAX_VAL_SUITE := string(deck[0][1]) //Always selects the suite of the first card for now
	return MAX_VAL_SUITE
}

// Returns the id of the player who selected the trump suite
//
// Returns an error if the bidHistory field has wrong data types
func TrumpPlayerId(bidHistory [][]interface{}) (string, error) {
	maxm := []interface{}{"", math.Inf(-1)}
	for _, item := range bidHistory {
		if bid, ok := item[1].(float64); ok {
			if max_bid, ok := maxm[1].(float64); ok {
				if bid >= max_bid {
					maxm = []interface{}{maxm[0], max_bid}
				}
			} else {
				return "", errors.New("bidHistory field contains unexpected types")
			}
		} else {
			return "", errors.New("bidHistory field contains unexpected types")
		}
	}
	if trump_player_id, ok := maxm[0].(string); ok {
		return trump_player_id, nil
	} else {
		return "", errors.New("bidHistory field contains unexpected types")
	}
}
