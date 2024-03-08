#!/bin/bash

sleep 2
near create-account node4-validator --masterAccount jamesavechives --publicKey ed25519:7tnvJLon5BRLRUbs66wvVKyS8dmEddvrL4brjRievTwi

# send money to new accounts
near send jamesavechives node4-validator 100000000
## stake new accounts
#sleep 2
near stake node4-validator ed25519:7tnvJLon5BRLRUbs66wvVKyS8dmEddvrL4brjRievTwi 100000000 --keyPath /Users/jameswalstonn/.unc/Unc4/validator_key.json


# Register Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/near extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/Unc1/signer2_key.json with-init-call json-args '{"power": "7000000000000"}' network-config my-private-chain-id sign-with-plaintext-private-key --signer-public-key ed25519:44ft5VbgWzjdjnbUgpkPRznGTGx4ocTDevVMQGsyc2UN --signer-private-key ed25519:NuB7Vo3rtcVyRsVbeuqSaHL7mFuq3WA2LT44qUn6DWy1XfMc9oJjf1RmbFRpoB4oapYijXzf8mowcpQmioV7HuQ send


# Chanllenge Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/near extensions create-challenge-rsa node4-validator use-file /Users/jameswalstonn/.unc/Unc1/signer2_key.json without-init-call network-config my-private-chain-id sign-with-plaintext-private-key --signer-public-key ed25519:7tnvJLon5BRLRUbs66wvVKyS8dmEddvrL4brjRievTwi --signer-private-key ed25519:4Ft7TEWqdCcGUS6wywD3jg5wdGuNNMv5d6aw6avy5ri762k4ta35hNjCKhWtG9UGX4QCzkMsaf3X1bAnaFBqq97A send
