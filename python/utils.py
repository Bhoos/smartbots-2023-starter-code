def get_suit(card):
  """
  This function returns the suit of the given card.
  """
  
  return card[1]



def get_suit_card(cards, card_suit):
  """
  This function returns the list of cards of the given suit from the initial list of cards.
  """
  
  return [i for i in cards if get_suit(i) == card_suit]
  
