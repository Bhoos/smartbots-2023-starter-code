const express = require("express");
const cors = require("cors");

const bid = require("../js-common/bid");
const chooseTrump = require("../js-common/chooseTrump");
const play = require("../js-common/play");

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
app.get("/hi", (req, res) => {
  console.log("Hit the endpoint. Sending hello...");
  res.send({ value: "hello" });
});

app.post("/bid", (req, res) => {
  const result = bid(req.body);
  res.send(result);
});

app.post("/chooseTrump", (req, res) => {
  const result = chooseTrump(req.body);
  res.send(result);
});

app.post("/play", (req, res) => {
  const result = play(req.body);
  res.send(result);
});

// ----------------------------------------

app.listen(port, () => {
  console.log(`Voila, your server is listening on port: ${port}`);
});
