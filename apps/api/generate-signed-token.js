const fs = require("fs");
const jwt = require("jsonwebtoken");

// Read the private key
const privateKey = fs.readFileSync("keys/private_key.pem", "utf8");

// Create a payload
const payload = {
  sub: "admin@magmooty.com",
  name: "Magmooty Admin",
};

// Sign the token
const token = jwt.sign(payload, privateKey, {
  algorithm: "RS256",
  expiresIn: "1h",
});

console.log(token);
