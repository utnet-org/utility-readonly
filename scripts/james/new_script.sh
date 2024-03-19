#!/bin/bash

# run nodes
sleep 2
target/debug/uncd  --home "/Users/jameswalstonn/.unc/Unc5" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "/Users/jameswalstonn/.unc/logfiles/logfile5" 2>&1 &

# create new account
sleep 2
unc create-account node5-validator --masterAccount jamesavechives --publicKey ed25519:3xtwP9QUaUrjZyH4HXYhamZ357rWEFJZGw4bR1Z6peYm

## send money to new accounts
sleep 2
unc send jamesavechives node5-validator 20000000

## stake new accounts
sleep 2
unc stake node5-validator ed25519:3xtwP9QUaUrjZyH4HXYhamZ357rWEFJZGw4bR1Z6peYm 10000000 --keyPath /Users/jameswalstonn/.unc/Unc5/validator_key.json


# Register Rsa2048 keys
sleep 5
#/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/Unc1/signer3_key.json with-init-call json-args '{"power": "8000000000000"}' network-config my-private-chain-id sign-with-plaintext-private-key --signer-public-key ed25519:8varMxFYfy9MprpoK2wCeYCBdLs9j4tTFWwS31haQfTL --signer-private-key ed25519:4xwMSJ5rRjWPLRn28uzoyJGpzSUSBPsHfLBkhcFnhU3YdXewCWxQEUqjjqciqJbisrXfZUJ3KBn1kXntKpkq8BkS send


# Chanllenge Rsa2048 keys
sleep 5
#/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa node5-validator use-file /Users/jameswalstonn/.unc/Unc1/signer3_key.json without-init-call network-config my-private-chain-id sign-with-plaintext-private-key --signer-public-key ed25519:3xtwP9QUaUrjZyH4HXYhamZ357rWEFJZGw4bR1Z6peYm --signer-private-key ed25519:bVBPVrBNY1doZXezeFn9PPJyqAh8Wke9rASdqGoSt6GcBJMTTv2v8UjASqjWBGFTHvw2wqNg9vj3y3J4vyc2ebF send

