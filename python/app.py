import os

# load contents from .env file
from dotenv import load_dotenv
load_dotenv()

from sanic import Sanic
from sanic.response import json
from sanic.request import Request
from sanic_cors import CORS

from bot import get_bid, get_trump_suit, get_play_card



# to disable debug mode, set DEBUG=0 in .env file, otherwise debug mode is enabled by default as DEBUG=1
DEBUG = os.getenv("DEBUG", True)
if type(DEBUG) == str:DEBUG = int(DEBUG)

app = Sanic(__name__)
CORS(app)

inbuilt_print = print

def print(args):
    # only log to output if in debug mode
    # logging to console is farily expensive so, log only when necessary
    if DEBUG:
        inbuilt_print(args)



@app.route("/hi", methods=["GET"])
def hi(request: Request):
    print("Hit the endpoint. Sending hello...")
    return json({"value": "hello"})



@app.route("/bid", methods=["POST"])
def bid(request: Request):
    """
    Request data format:
    {
        "playerId": "A1", # own player id
        "playerIds": ["A1", "B1", "A2", "B2"],  # player ids in order
        "timeRemaining": 1200,
        "cards": ["JS", "TS", "KH", "9C"],      # own cards
        "bidHistory": [ ["A1", 16],             # bidding history in chronological order
                        ["B1",17], 
                        ["A1", 17], 
                        ["B1", 0], 
                        ["A2", 0], 
                        ["B2", 0]
                    ],
        "bidState": {
            "defenderId": "A1",
            "challengerId": "B1",
            "defenderBid": 16,
            "challengerBid": 17
        },
    }
    """
    
    body = request.json
    print(body)
    
    return json(get_bid(body))



@app.route("/chooseTrump", methods=["POST"])
def choose_trump(request: Request):
    """
    Request data format:
    {
        "playerId": "A1",                       # own player id
        "playerIds": ["A1", "B1", "A2", "B2"],  # player ids in order
        "timeRemaining": 1200,
        "cards": ["JS", "TS", "KH", "9C"],      # own cards
        "bidHistory": [ ["A1", 16],             # bidding history in chronological order
                        ["B1",17], 
                        ["A1", 17], 
                        ["B1", 0], 
                        ["A2", 0], 
                        ["B2", 0]
                    ], 
    }
    """
    
    body = request.json
    print(body)
    
    return json(get_trump_suit(body))



@app.route("/play", methods=["POST"])
def play(request: Request):
    """
    Request data format:
    {
        "playerId": "A2", # own player id
        "playerIds": ["A1", "B1", "A2", "B2"],                  # player ids in order
        "timeRemaining": 1500,
        "teams": [
            { "players": ["A1", "A2"], "bid": 17, "won": 0 },   # first team information
            { "players": ["B1", "B2"], "bid": 0, "won": 4 },    # second team information
        ],
        "cards": ["JS", "TS", "KH", "9C", "JD", "7D", "8D"],    # own cards
        "bidHistory": [ ["A1", 16],                             # bidding history in chronological order
                        ["B1",17], 
                        ["A1", 17], 
                        ["B1", 0], 
                        ["A2", 0], 
                        ["B2", 0]
                    ],
        "played": ["9S", "1S", "8S"],
        "handsHistory": [
            [
                "A1", # player who threw the first card ("7H") 
                ["7H", "1H", "8H", "JH"],           # cards that thrown in the first hand
                "B2" # winner of this hand
            ]
        ],
        
        # represents the suit if available, the trumpSuit is only present for the player who reveals the trump
        # after the trump is revealed, the trumpSuit is present for all the players
        "trumpSuit": false | "H",

        # only after the trump is revealed by the player the information is revealed
        "trumpRevealed": false | {
            hand: 2,            # represents the hand at which the trump was revealed
            playerId: "A2",     # the player who revealed the trump
        },
    }
    """
    
    body = request.json
    print(body)
    
    return json(get_play_card(body))



if __name__ == "__main__":
    # Docker image should always listen in port 8001
    app.run(host="0.0.0.0", port=8001, debug=DEBUG, access_log=DEBUG)