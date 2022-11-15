const { createServer } = require("http");

const PORT = 8001;

const server = createServer((req, res) => {
  if (req.url) res.setHeader("Content-Type", "application/json");
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Headers", "*");
  res.setHeader(
    "Access-Control-Allow-Methods",
    "GET, POST, PUT, DELETE, OPTIONS"
  );

  if (req.method === "OPTIONS") {
    res.end();
    return;
  }

  let payload = "";
  req.on("data", (chunk) => {
    payload += chunk;
  });

  req.on("end", () => {
    let result = { error: "Unknown request" };

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
    if (req.url.endsWith("hi")) {
      result = hello(payload);
    } else if (req.url.endsWith("bid")) {
      result = bid(JSON.parse(payload));
    } else if (req.url.endsWith("chooseTrump")) {
      result = chooseTrump(JSON.parse(payload));
    } else if (req.url.endsWith("play")) {
      result = play(JSON.parse(payload));
    }

    res.write(JSON.stringify(result));
    res.end();
  });
});

server.listen(PORT, () => {
  console.log(`Voila, your server is listening on port: ${PORT}`);
});
// ----------------------------------------

function hello(req, res) {
  console.log("Hit the endpoint. Sending hello...");
  return { value: "hello" };
}

/**
 * Please note: these are the bare starter code that should get you started
 * Take it as a reference or example, not necessarily the code you would like to start with
 */

const MIN_BID = 16;
const PASS_BID = 0;

function bid(payload) {
  if (payload.bidState["defender-bid"] === 0) {
    return {
      bid: MIN_BID,
    };
  } else {
    return {
      bid: PASS_BID,
    };
  }
}

function chooseTrump(payload) {
  /**
   * Please note: this is bare implementation of the chooseTrump function
   * Do make changes to this function to throw valid suit according to the context of the game.
   */
  return {
    suit: "H",
  };
}

function play(payload) {
  /**
   * Please note: this is bare implementation of the play function
   * It just returns the last card that we have.
   * Do make changes to this function to throw valid card according to the context of the game.
   */
  const cards = payload.cards;
  const randomNotNeccessarilyValidCard = cards[cards.length - 1];

  return {
    card: randomNotNeccessarilyValidCard,
  };
}
