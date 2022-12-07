const { getSuitCards, last, getSuit, pickWinningCardIdx, getPartnerIdx, isHigh } = require('./shared');

/**
 * @payload
  {
    "playerId": "A2", // own player id
    "playerIds": ["A1", "B1", "A2", "B2"], // player ids in order
    "timeRemaining": 1500,
    "teams": [
      { "players": ["A1", "A2"], "bid": 17, "won": 0 }, // first team information
      { "players": ["B1", "B2"], "bid": 0, "won": 4 }, // second team information
    ],
    "cards": ["JS", "TS", "KH", "9C", "JD", "7D", "8D"], // own cards
    "bidHistory": [["A1", 16], ["B1",17], ["A1", 17], ["B1", 0], ["A2", 0], ["B2", 0]], // bidding history in chronological order
    "played": ["9S", "1S", "8S"],
    "handsHistory": [
        [
          "A1", // player who threw the first card ("7H") 
          ["7H", "1H", "8H", "JH"], // cards that thrown in the first hand
          "B2" // winner of this hand
        ]
    ],
    // represents the suit if available, the trumpSuit is only present for the player who reveals the trump
    // after the trump is revealed, the trumpSuit is present for all the players
    "trumpSuit": false | "H",

    // only after the trump is revealed by the player the information is revealed
    "trumpRevealed": false | {
      hand: 2, // represents the hand at which the trump was revealed
      playerId: "A2", // the player who revealed the trump
    },
  }
 */
function play(payload) {
  console.log('play', JSON.stringify(payload));

  const ownCards = payload.cards;
  const firstCard = payload.played[0];
  const trumpSuit = payload.trumpSuit;
  const trumpRevealed = payload.trumpRevealed;
  const handsHistory = payload.handsHistory;
  const ownId = payload.playerId;
  const played = payload.played;
  const playerIds = payload.playerIds;
  const myIdx = playerIds.findIndex(id => id === ownId);
  const myPartnerIdx = getPartnerIdx(myIdx);
  const firstTurn = (myIdx + 4 - played.length) % 4;
  const isBidder = trumpSuit && !trumpRevealed;

  /** we are the one to throw the first card in the hands */
  if (!firstCard) {
    return {
      card: last(ownCards),
    };
  }

  const firstCardSuit = getSuit(firstCard);
  const ownSuitCards = getSuitCards(payload.cards, firstCardSuit);

  /** if we have the suit with respect to the first card, we throw it */
  if (ownSuitCards.length > 0) {
    return {
      card: last(ownSuitCards),
    };
  }

  /**
   * we don't have cards that follow the suit
   * @example
   *  the first player played "7H" (7 of hearts)
   *  we don't have any cards of suit "hearts"
   *
   * We could either
   *
   * 1. throw any card
   * 2. reveal the trump
   */

  /**
   * trump has not been revealed yet, and we don't know what the trump is
   * let's reveal the trump
   */
  if (!trumpSuit && !trumpRevealed) {
    return {
      revealTrump: true,
    };
  }

  /**  we don't have any trump suit cards, throw random */
  const ownTrumpSuitCards = getSuitCards(ownCards, trumpSuit);
  if (ownTrumpSuitCards.length === 0) {
    return {
      card: last(ownCards),
    };
  }

  const didIRevealTheTrumpInThisHand =
    trumpRevealed && trumpRevealed.playerId === ownId && trumpRevealed.hand === handsHistory.length + 1;

  /**
   * trump was revealed by me in this hand
   * or
   * I'm going to reveal the trump, since I'm the bidder
   */
  if (isBidder || didIRevealTheTrumpInThisHand) {
    const response = {};
    if (isBidder) response.revealTrump = true;

    /** if there are no trumps in the played */
    if (getSuitCards(played, trumpSuit).length === 0) {
      response.card = last(ownTrumpSuitCards);
      return response;
    }

    const winningTrumpCardIdx = pickWinningCardIdx(played, trumpSuit);
    const winningCardPlayerIdx = (firstTurn + winningTrumpCardIdx) % 4;

    /**
     * if we revealed the trump in this round and the winning card is trump, there are two cases
     * 1. If the opponent is winning the hand, then you must throw the winning card of the trump suit against your opponent's card.
     * 2. If your partner is winning the hand, then you could throw any card of trump suit since your team is only winning the hand.
     */
    if (winningCardPlayerIdx === myPartnerIdx) {
      response.card = last(ownTrumpSuitCards);
      return response;
    }

    const winningTrumpCard = played[winningTrumpCardIdx];
    const winningCard = ownTrumpSuitCards.find(card => isHigh(card, winningTrumpCard)) || last(ownTrumpSuitCards);

    /** player who revealed the trump should throw the trump suit card */
    return {
      card: winningCard,
    };
  }

  return {
    card: last(ownCards),
  };
}

module.exports = play;
