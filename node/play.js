const { getSuitCards, last, getSuit } = require("./shared");

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
  console.log("play", JSON.stringify(payload));

  const ownCards = payload.cards;
  const firstCard = payload.played[0];
  const trumpSuit = payload.trumpSuit;
  const trumpRevealed = payload.trumpRevealed;
  const handsHistory = payload.handsHistory;
  const ownId = payload.playerId;

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

  /** trump is already revealed, and everyone knows the trump */
  if (trumpSuit && trumpRevealed) {
    const wasTrumpRevealInThisRound =
      trumpRevealed.hand === handsHistory.length + 1;
    const didIRevealTheTrump = trumpRevealed.playerId === ownId;

    /**
     * if I'm the one who revealed the trump in this round.
     */
    if (wasTrumpRevealInThisRound && didIRevealTheTrump) {
      const trumpSuitCards = getSuitCards(ownCards, trumpSuit);

      /** player who revealed the trump should throw the trump suit card */
      return {
        card: last(trumpSuitCards) || last(ownCards),
      };
    }

    return {
      card: last(ownCards),
    };
  }

  /**
   * trump is revealed only to me
   * this means we won the bidding phase, and set the trump
   */
  if (trumpSuit && !trumpRevealed) {
    const trumpSuitCards = getSuitCards(ownCards, trumpSuit);

    return {
      /**  after revealing the trump, we must throw trump card */
      revealTrump: true,
      card: last(trumpSuitCards) || last(ownCards),
    };
  }

  /** trump has not yet been revealed, let's reveal the trump */
  return {
    revealTrump: true,
  };
}

module.exports = play;
