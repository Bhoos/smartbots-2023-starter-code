## Basic Server setup

1. Add dependencies - `express` and `cors`

```
yarn add express cors
```

We will use `express` as the server for ease of purpose.

2. Setup basic express server in your `node-express.js` file

```javascript
const express = require("express");
const cors = require("cors");

const app = express();
const port = 8001;

app.use(express.json());
app.use(cors({ origin: "*" }));

app.listen(port, () => {
  console.log(`Voila, your server is running on port: ${port}`);
});
```

1. Open the terminal and run the express server

```
node node-express.js
```

You should see the following message in the terminal

```
Voila, your server is running on port: 8001
```

4. Add `/hi` route to your express app.

```javascript
app.get("/hi", (req, res) => {
  console.log("Hit the endpoint. Sending hello...");
  res.send("hello");
});
```

5. Open your browser and enter `http://localhost:8001/hi` in the website section.
   You should get `hello` as the response.
