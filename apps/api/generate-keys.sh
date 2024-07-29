#!/bin/bash

# Generate private key
openssl genpkey -algorithm RSA -out keys/private_key.pem -pkeyopt rsa_keygen_bits:2048

# Extract public key
openssl rsa -pubout -in keys/private_key.pem -out keys/public_key.pem

# Generate base64 encoded public key
echo "Embed the following public key in your application's environment variables:"
echo admin_public_key=\"$(cat keys/public_key.pem | base64)\"
