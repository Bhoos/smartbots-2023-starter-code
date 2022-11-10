const express = require("express");
const cors = require("cors");

const app = express();
const port = 8001;

app.use(express.json());
app.use(cors({ origin: "*" }));

app.get("/hi", hi);
app.post("/bid", bid);
app.post("/chooseTrump", chooseTrump);
app.post("/play", play);

app.listen(port, () => {
  console.log(`Voila, your server is running on port: ${port}`);
});

function hi(req, res) {
  console.log("Hit the endpoint. Sending hello...");
  res.send({ value: "hello" });
}

function bid(req, res) {
  if (req.body.bidState["defender-bid"] === 0) {
    const minBid = 16;
    return res.send({ bid: minBid });
  } else {
    const passBid = 0;
    return res.send({ bid: passBid });
  }
}

function chooseTrump(req, res) {
  const cards = req.body.cards;
  const randomCard = cards[cards.length - 1];
  const suit = randomCard[randomCard.length - 1];

  return res.send({ suit });
}

function play(req, res) {
  const cards = req.body.cards;
  const randomCard = cards[cards.length - 1];

  return res.send({ card: randomCard });
}
