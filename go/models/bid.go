package models

type BidState struct {
	DefenderId    string `json:"defenderId"`
	DefenderBid   int    `json:"defenderBid"`
	ChallengerId  string `json:"challengerId"`
	ChallengerBid int    `json:"challengerBid"`
}

type BidValue struct {
	Bid int `json:"bid"`
}

type BidRequest struct {
	PlayerId      string        `json:"playerId"`
	PlayerIds     []string      `json:"playerIds"`
	TimeRemaining float64       `json:"timeRemaining"`
	Cards         []string      `json:"cards"`
	BidHistory    []interface{} `json:"bidHistory"`
	BidState      BidState      `json:"bidState"`
}
