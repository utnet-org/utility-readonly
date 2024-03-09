#!/bin/bash

sleep 2
near create-account node4-validator --masterAccount jamesavechives --publicKey ed25519:7tnvJLon5BRLRUbs66wvVKyS8dmEddvrL4brjRievTwi

# send money to new accounts
near send jamesavechives node4-validator 300000000
## stake new accounts
#sleep 2
near stake node4-validator ed25519:7tnvJLon5BRLRUbs66wvVKyS8dmEddvrL4brjRievTwi 100000000 --keyPath /Users/jameswalstonn/.near/Near4/validator_key.json


# Register Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/near extensions register-rsa-keys jamesavechives use-file /Users/jameswalstonn/.near/Near1/signer2_key.json with-init-call json-args '{"power": "7000000000000"}' network-config my-private-chain-id sign-with-plaintext-private-key --signer-public-key ed25519:3mda2kgqvbybK9sHEEuoDWjeeZodJe9Fo64GRYCGBiZF --signer-private-key ed25519:2yVZSiz7hBsGmLBNyVL4Lx1GgMM3npcDSHu9aWwn3U7uPdZFc9MLfRkycDpPXTCJ5Ad8MqU4PYJvbZoKtrPHnp5f send


# Chanllenge Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/near extensions create-challenge-rsa node4-validator use-file /Users/jameswalstonn/.near/Near1/signer2_key.json without-init-call network-config my-private-chain-id sign-with-plaintext-private-key --signer-public-key ed25519:7tnvJLon5BRLRUbs66wvVKyS8dmEddvrL4brjRievTwi --signer-private-key ed25519:4Ft7TEWqdCcGUS6wywD3jg5wdGuNNMv5d6aw6avy5ri762k4ta35hNjCKhWtG9UGX4QCzkMsaf3X1bAnaFBqq97A send
