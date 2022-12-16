package models

type Team struct {
	Players []string `json:"players"`
	Bid     int      `json:"bid"`
	Won     int      `json:"won"`
}

type PlayRequest struct {
	PlayerId      string          `json:"playerId"`
	PlayerIds     []string        `json:"playerIds"`
	TimeRemaining int             `json:"timeRemaining"`
	Teams         []Team          `json:"teams"`
	Cards         []string        `json:"cards"`
	BidHistory    [][]interface{} `json:"bidHistory"`
	Played        []string        `json:"played"`
	HandsHistory  [][]interface{} `json:"handsHistory"`
	TrumpSuit     interface{}     `json:"trumpSuit"`
	TrumpRevealed interface{}     `json:"trumpRevealed"`
}
type RevealTrumpNoCard struct {
	RevealTrump bool `json:"revealTrump"`
}
type RevealTrumpCard struct {
	RevealTrump bool   `json:"revealTrump"`
	Card        string `json:"card"`
}

type PlayCard struct {
	Card string `json:"card"`
}
