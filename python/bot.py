import random

ALL_SUITS = ["C", "D", "H", "S"]
MIN_BID = 16
PASS_BID = 0



def get_bid(body):
    ####################################
    #     Input your code here.        #
    ####################################
    
    if body["bidState"]["defenderBid"] == 0:
        return {"bid": MIN_BID}
    
    else:
        return {"bid": PASS_BID}



def get_trump_suit(body):
    ####################################
    #     Input your code here.        #
    ####################################
    suit = random.choice(ALL_SUITS)
    
    return {"suit": suit}



def get_play_card(body):
    ####################################
    #     Input your code here.        #
    ####################################
    valid_cards = body["cards"]
    player_move = valid_cards[-1]
    
    return {"card": player_move}