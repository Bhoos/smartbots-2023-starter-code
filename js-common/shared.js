function last(items) {
  return items[items.length - 1];
}

function getSuit(card) {
  return card[1];
}

function getSuitCards(cards, cardSuit) {
  return cards.filter((card) => getSuit(card) === cardSuit);
}

module.exports = {
  last,
  getSuit,
  getSuitCards,
};
