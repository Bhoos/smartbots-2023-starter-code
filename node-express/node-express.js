const express = require("express");
const cors = require("cors");

const app = express();
const port = 8001;

app.use(express.json());
app.use(cors({ origin: "*" }));

/*
  ####################################
  #     ENDPOINTS       #
  ####################################
*/

/**
 * These are the endpoints that you should implement on your own.
 * We call your endpoints and expect valid response.
 * According to the endpoints, the match is played and the tournament is run.
 */
app.get("/hi", hello);
app.post("/bid", bid);
app.post("/chooseTrump", chooseTrump);
app.post("/play", play);

// ----------------------------------------

app.listen(port, () => {
  console.log(`Voila, your server is listening on port: ${port}`);
});

function hello(req, res) {
  console.log("Hit the endpoint. Sending hello...");
  res.send({ value: "hello" });
}

/**
 * Please note: these are the bare starter code that should get you started
 * Take it as a reference or example, not necessarily the code you would like to start with
 */

const MIN_BID = 16;
const PASS_BID = 0;

function bid(req, res) {
  if (req.body.bidState["defender-bid"] === 0) {
    return res.send({ bid: MIN_BID });
  } else {
    return res.send({ bid: PASS_BID });
  }
}

function chooseTrump(req, res) {
  /**
   * Please note: this is bare implementation of the chooseTrump function
   * Do make changes to this function to throw valid suit according to the context of the game.
   */
  return res.send({ suit: "H" });
}

function play(req, res) {
  /**
   * Please note: this is bare implementation of the play function
   * It just returns the last card that we have.
   * Do make changes to this function to throw valid card according to the context of the game.
   */
  const cards = req.body.cards;
  const randomNotNeccessarilyValidCard = cards[cards.length - 1];

  return res.send({ card: randomNotNeccessarilyValidCard });
}
