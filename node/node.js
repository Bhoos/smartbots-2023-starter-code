const express = require("express");
const cors = require("cors");

const app = express();
const port = 8001;

app.use(express.json());
app.use(cors({ origin: "*" }));

app.get("/hi", (req, res) => {
  console.log("Hit the endpoint. Sending hello...");
  res.send("hello");
});

function response(res) {
  return {
    value: res,
  };
}

app.post("/bid", (req, res) => {
  if (req.body.bidState["defender-bid"] === 0) {
    const minBid = 16;
    return res.send(response({ bid: minBid }));
  } else {
    return res.send(response({ bid: "pass" }));
  }
});

app.post("/chooseTrump", (req, res) => {
  const cards = req.body.cards;
  const randomCard = cards[cards.length - 1];
  const suit = randomCard[randomCard.length - 1];

  return res.send(response({ suit }));
});

app.post("/play", (req, res) => {
  const cards = req.body.cards;
  const randomCard = cards[cards.length - 1];

  return res.send(response({ card: randomCard }));
});

app.listen(port, () => {
  console.log(`Voila, your server is running on port: ${port}`);
});
