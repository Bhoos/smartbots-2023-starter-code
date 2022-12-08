def get_suit(card):
    """
    This function returns the suit of the given card.
    """

    return card[1]


def get_suit_cards(cards, card_suit):
    """
    This function returns the list of cards of the given suit from the initial list of cards.
    """
    return [card for card in cards if get_suit(card) == card_suit]


def index(sequence, predicate):
    """
    This function returns the index of the first element in the sequence which satisfies the predicate, otherwise -1
    Just like javascript
    """
    return next((i for i, e in enumerate(sequence) if predicate(e)), -1)


def find(sequence, predicate):
    """
    This function returns the first element in the sequence which satisfies the given predicate, None otherwise
    Just like Javascript
    """
    return next((e for i, e in enumerate(sequence) if predicate(e)), None)


def get_partner_idx(my_idx):
    return (my_idx + 2) % 4


def get_rank(card):
    return card[0]


CARDS_DICT = {
    "J": {"points": 3, "order": 8},
    "9": {"points": 2, "order": 7},
    "1": {"points": 1, "order": 6},
    "T": {"points": 1, "order": 5},
    "K": {"points": 0, "order": 4},
    "Q": {"points": 0, "order": 3},
    "8": {"points": 0, "order": 2},
    "7": {"points": 0, "order": 1},
}


def get_card_info(card):
    return CARDS_DICT[get_rank(card)]


def is_high(highest_card, compare_card, trump_suit=None):
    is_highest_card_trump = get_suit(highest_card) == trump_suit
    is_compare_card_trump = get_suit(compare_card) == trump_suit

    if (trump_suit and is_highest_card_trump and not is_compare_card_trump):
        return True
    if (trump_suit and not is_highest_card_trump and is_compare_card_trump):
        return False
    # if both have similar suit, we could just check the points with order
    if (get_suit(highest_card) == get_suit(compare_card)):
        high = get_card_info(highest_card)
        compare = get_card_info(compare_card)

        return high["points"] >= compare["points"] and high["order"] > compare["order"]

    return True


def pick_winning_card_idx(cards, trump_suit):
    winner = 0
    first_card = cards[0]

    for i in range(winner, len(cards)):
        winning_card = cards[winner]
        compare_card = cards[i]

        if (not trump_suit and get_suit(first_card) != get_suit(compare_card)):
            continue
        if (not is_high(winning_card, compare_card, trump_suit)):
            winner = i

    return winner
