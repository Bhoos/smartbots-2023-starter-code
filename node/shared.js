function last(items) {
  return items[items.length - 1];
}

function getSuit(card) {
  return card[1];
}

function getRank(card) {
  return card[0];
}

function getSuitCards(cards, cardSuit) {
  return cards.filter(card => getSuit(card) === cardSuit);
}

const cardsDict = {
  J: { points: 3, order: 8 },
  9: { points: 2, order: 7 },
  1: { points: 1, order: 6 },
  T: { points: 1, order: 5 },
  K: { points: 0, order: 4 },
  Q: { points: 0, order: 3 },
  8: { points: 0, order: 2 },
  7: { points: 0, order: 1 },
};

function getCardInfo(card) {
  return cardsDict[getRank(card)];
}

function isHigh(highestCard, compareCard, trumpSuit) {
  const isHighestCardTrump = getSuit(highestCard) === trumpSuit;
  const isCompareCardTrump = getSuit(compareCard) === trumpSuit;

  if (trumpSuit && isHighestCardTrump && !isCompareCardTrump) return true;
  if (trumpSuit && !isHighestCardTrump && isCompareCardTrump) return false;

  /** if both have similar suit, we could just check the points with order */
  if (getSuit(highestCard) === getSuit(compareCard)) {
    const high = getCardInfo(highestCard);
    const compare = getCardInfo(compareCard);

    return high.points >= compare.points && high.order > compare.order;
  }

  /** if both are of different suit, and the high card should win */
  return true;
}

function pickWinningCardIdx(cards, trumpSuit) {
  let winner = 0;
  const firstCard = cards[0];

  for (let i = winner; i < cards.length; i++) {
    const winningCard = cards[winner];
    const compareCard = cards[i];

    if (!trumpSuit && getSuit(firstCard) !== getSuit(compareCard)) continue;
    if (!isHigh(winningCard, compareCard, trumpSuit)) winner = i;
  }

  return winner;
}

function isPartner(myIdx, maybePartnerIdx) {
  return myIdx % 2 === maybePartnerIdx % 2;
}

function getPartnerIdx(myIdx) {
  return (myIdx + 4 - 2) % 4;
}

module.exports = {
  last,
  getSuit,
  getSuitCards,
  pickWinningCardIdx,
  isPartner,
  getPartnerIdx,
  isHigh,
};
