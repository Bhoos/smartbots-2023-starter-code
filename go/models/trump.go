package models

type TrumpRequest struct {
	PlayerId      string        `json:"playerId"`
	PlayerIds     []string      `json:"playerIds"`
	TimeRemaining float64       `json:"timeRemaining"`
	Cards         []string      `json:"cards"`
	BidHistory    []interface{} `json:"bidHistory"`
}

type TrumpSuit struct {
	Suit string `json:"suit"`
}

type TrumpRevealed struct {
	Hand     string `json:"hand"`
	PlayerId string `json:"playerId"`
}
