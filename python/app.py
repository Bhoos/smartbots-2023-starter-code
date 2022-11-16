import os

from sanic import Sanic
from sanic.response import json
from sanic.request import Request
from sanic_cors import CORS

from bot import get_bid, get_trump_suit, get_play_card



# to enable debug, run app with `DEBUG=1 python src/app.py`
DEBUG = os.getenv("DEBUG", False)

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
    Please note: this is bare implementation of the bid function.
    Do make changes to this function to throw valid bid according to the context of the game.
    """
    body = request.json
    print(body)
    
    return json(get_bid(body))



@app.route("/chooseTrump", methods=["POST"])
def choose_trump(request: Request):
    """
    Please note: this is bare implementation of the chooseTrump function.
    Do make changes to this function to throw valid card according to the context of the game.
    """
    body = request.json
    print(body)
    
    return json(get_trump_suit(body))



@app.route("/play", methods=["POST"])
def play(request: Request):
    """
    Please note: this is bare implemenation of the play function.
    It just returns the last card that we have.
    Do make changes to the function to throw valid card according to the context of the game.
    """
    body = request.json
    print(body)
    
    return json(get_play_card(body))



if __name__ == "__main__":
    # Docker image should always listen in port 8001
    app.run(host="0.0.0.0", port=8001, debug=DEBUG, access_log=DEBUG)