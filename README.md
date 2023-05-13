# my_crypto_wallet
Simple cli wallet utility for interacting with various networks.

This wallet will try to read a json file that contains the public key, private key and the public address
in the following format:

{
  "secret_key": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
  "public_key": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
  "public_address": "0x................................"
}

It is also necessary to setup a websocket point to interact with the networks. This can be done with Infura.io, or any other free provider.

Add a file called '.env' to the directory and then place your keys in there... like this:

INFURA_SEPOLIA_WS=wss://sepolia.infura.io/ws/v3/....................
