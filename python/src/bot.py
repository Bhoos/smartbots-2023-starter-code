from utils import get_suit, get_suit_cards, get_partner_idx, pick_winning_card_idx, is_high, index, find


def get_bid(body):
    """
    Please note: this is bare implementation of the bid function.
    Do make changes to this function to throw valid bid according to the context of the game.
    """

    ####################################
    #     Input your code here.        #
    ####################################

    MIN_BID = 16
    PASS_BID = 0

    # when you are the first player to bid, use the minimum bid
    if len(body["bidHistory"]) == 0:
        return {"bid": MIN_BID}

    return {"bid": PASS_BID}


def get_trump_suit(body):
    """
    Please note: this is bare implementation of the chooseTrump function.
    Do make changes to this function to throw valid card according to the context of the game.
    """

    ####################################
    #     Input your code here.        #
    ####################################

    own_cards = body["cards"]
    last_card = own_cards[-1]
    last_card_suit = get_suit(last_card)

    return {"suit": last_card_suit}


def get_play_card(body):
    """
    Please note: this is bare implemenation of the play function.
    It just returns the last card that we have.
    Do make changes to the function to throw valid card according to the context of the game.
    """

    ####################################
    #     Input your code here.        #
    ####################################

    own_cards = body["cards"]
    first_card = None if len(body["played"]) == 0 else body["played"][0]
    trump_suit = body["trumpSuit"]
    trump_revealed = body["trumpRevealed"]
    hand_history = body["handsHistory"]
    own_id = body["playerId"]
    played = body["played"]
    player_ids = body["playerIds"]
    my_idx = player_ids.index(own_id)
    my_idx = index(
        player_ids, lambda id: id == own_id)
    my_partner_idx = get_partner_idx(my_idx)
    first_turn = (my_idx + 4 - len(played)) % 4
    is_bidder = trump_suit and not trump_revealed

    # if we are the one to throw the first card in the hands
    if (not first_card):
        return {"card": own_cards[-1]}

    first_card_suit = get_suit(first_card)
    own_suit_cards = get_suit_cards(own_cards, first_card_suit)

    # if we have the suit with respect to the first card, we throw it
    if len(own_suit_cards) > 0:
        return {"card": own_suit_cards[-1]}

    # if we don't have cards that follow the suit
    # @example
    # the first player played "7H" (7 of hearts)
    #
    # we could either
    #
    # 1. throw any card
    # 2. reveal the trump

    # trump has not been revealed yet, and we don't know what the trump is
    # let's reveal the trump
    if (not trump_suit and not trump_revealed):
        return {"revealTrump": True}

    # we don't have any trump suit cards, throw random
    own_trump_suit_cards = get_suit_cards(own_cards, trump_suit)
    if (len(own_trump_suit_cards) == 0):
        return {"card": own_cards[-1]}

    did_reveal_the_trump_in_this_hand = trump_revealed and trump_revealed["playerId"] == own_id and trump_revealed["hand"] == (
        len(hand_history) + 1)

    # trump was revealed by me in this hand
    # or
    # I am going to reveal the trump, since I am the bidder

    if (is_bidder or did_reveal_the_trump_in_this_hand):
        response = {}
        if (is_bidder):
            response["revealTrump"] = True

        # if there are no trumps in the played
        if (len(get_suit_cards(played, trump_suit)) == 0):
            response["card"] = own_trump_suit_cards[-1]
            return response

        winning_trump_card_idx = pick_winning_card_idx(played, trump_suit)
        winning_card_player_idx = (first_turn + winning_trump_card_idx) % 4

        # if we revealed the trump in this round and the winning card is trump, there are two cases
        # 1. If the opponent is winning the hand, then you must throw the winning card of the trump suit against your opponent's card.
        # 2. If your partner is winning the hand, then you could throw any card of trump suit since your team is only winning the hand.
        if (winning_card_player_idx == my_partner_idx):
            response["card"] = own_trump_suit_cards[-1]
            return response

        winning_trump_card = played[winning_trump_card_idx]
        winning_card = find(own_trump_suit_cards, lambda card: is_high(
            card, winning_trump_card)) or own_trump_suit_cards[-1]

        # player who revealed the trump should throw the trump suit card
        return {"card": winning_card}

    return {"card": own_cards[-1]}
