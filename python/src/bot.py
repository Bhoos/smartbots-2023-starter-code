from utils import get_suit, get_suit_card


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
        return {"bid" : MIN_BID}
    
    return {"bid" : PASS_BID}






def get_trump_suit(body):
    """
    Please note: this is bare implementation of the chooseTrump function.
    Do make changes to this function to throw valid card according to the context of the game.
    """
    
    
    ####################################
    #     Input your code here.        #
    ####################################
    
    last_card = body['cards'][-1]
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
    player_id = body["playerId"]


    # if we are the one to throw the first card in the hands
    if(not first_card):
        return {"card": own_cards[-1]}
    
    first_card_suit = get_suit(first_card)
    own_suit_cards = get_suit_card(own_cards, first_card_suit)
    
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
    
    
    # trump suit is already revealed, and everyone knows the trump
    if (trump_suit and trump_revealed):
        was_trump_revealed_in_this_round = trump_revealed["hand"] == len(hand_history) + 1
        did_i_reveal_the_trump = trump_revealed["playerId"] == player_id
    
        # if I'm the one who revealed the trump in this round
        if was_trump_revealed_in_this_round and did_i_reveal_the_trump:
            trump_suit_cards = get_suit_card(own_cards, trump_suit)
            
            # player who revealed the trump should throw the trump suit card 
            if len(trump_suit_cards) > 0:
                return {"card": trump_suit_cards[-1]}
            
        # if trump suit card is not available, throw any card
        return {"card": own_cards[-1]}
        
    
    
    # trump is revealed only to me
    # this means we won the bidding phase, and set the trump
    if (trump_suit and not trump_revealed):
        trump_suit_cards = get_suit_card(own_cards, trump_suit)
        
        # after revealing the trump, we should throw the trump suit card if we have one
        if len(trump_suit_cards) > 0:
            return {
                "revealTrump": True,
                "card": trump_suit_cards[-1]
            }
        
        else:
            return {
                "revealTrump": True,
                "card": own_cards[-1]
            }
    
    # trump has not yet been revealed, let's reveal the trump
    return {"revealTrump" : True}