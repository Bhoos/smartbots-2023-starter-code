const { last, getSuit } = require("./shared");

/**
 * @payload
  {
    "playerId": "A1", // own player id
    "playerIds": ["A1", "B1", "A2", "B2"], // player ids in order
    "timeRemaining": 1200,
    "cards": ["JS", "TS", "KH", "9C"], // own cards
    "bidHistory": [["A1", 16], ["B1",17], ["A1", 17], ["B1", 0], ["A2", 0], ["B2", 0]], // bidding history in chronological order
  }
 */

function chooseTrump(payload) {
  console.log("chooseTrump", JSON.stringify(payload));

  const ownCards = payload.cards;
  const lastCard = last(ownCards);
  const lastCardSuit = getSuit(lastCard);

  return {
    suit: lastCardSuit,
  };
}

module.exports = chooseTrump;
