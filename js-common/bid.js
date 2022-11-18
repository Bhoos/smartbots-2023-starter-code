/**
 * @payload
  {
    "playerId": "A1", // own player id
    "playerIds": ["A1", "B1", "A2", "B2"], // player ids in order
    "timeRemaining": 1200,
    "cards": ["JS", "TS", "KH", "9C"], // own cards
    "bidHistory": [["A1", 16], ["B1",17], ["A1", 17], ["B1", 0], ["A2", 0], ["B2", 0]], // bidding history in chronological order
    "bidState": {
      "defenderId": "A1",
      "challengerId": "B1",
      "defenderBid": 16,
      "challengerBid": 17
    },
  }
 */

const MIN_BID = 16;
const PASS_BID = 0;

function bid(payload) {
  console.log("bid", JSON.stringify(payload));

  if (payload.bidHistory.length === 0) {
    return {
      bid: MIN_BID,
    };
  }

  return {
    bid: PASS_BID,
  };
}

module.exports = bid;
