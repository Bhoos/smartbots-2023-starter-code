const { createServer } = require("http");

const bid = require("./bid");
const chooseTrump = require("./chooseTrump");
const play = require("./play");

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
