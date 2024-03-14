#!/bin/bash

/Library/WebServer/Documents/utility/target/debug/uncd  --home "/Users/jameswalstonn/.unc/Unc1" run  > "/Users/jameswalstonn/.unc/logfiles/logfile1" 2>&1 &
sleep 1
/Library/WebServer/Documents/utility/target/debug/uncd  --home "/Users/jameswalstonn/.unc/Unc2" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "/Users/jameswalstonn/.unc/logfiles/logfile2" 2>&1 &
sleep 1
/Library/WebServer/Documents/utility/target/debug/uncd  --home "/Users/jameswalstonn/.unc/Unc3" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "/Users/jameswalstonn/.unc/logfiles/logfile3" 2>&1 &
sleep 1
/Library/WebServer/Documents/utility/target/debug/uncd  --home "/Users/jameswalstonn/.unc/Unc4" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "/Users/jameswalstonn/.unc/logfiles/logfile4" 2>&1 &
sleep 10

## create new accounts
near create-account unc --masterAccount miner --publicKey ed25519:2yMvZrTtjgFMtcpE12G3tdt7KsYKdKE6jufRnz4Yyxw3

## send money to other accounts
near send miner 13735a00a19b0b572ed183f517d66c93f22b7cd216b6b0cfd2444191088a86af 35000000
sleep 2
near send miner 79d96a47f387ae8f8e92a7f5b42e75d86c31c680581ad77cef1115a5b76b6e3b 25000000
sleep 2
near send miner e2678f53a51a46a8c76639cf37ed6c6070b995ed759d6fff0fad1c25ee87057d 20000000

## stake new accounts
sleep 2
near stake 13735a00a19b0b572ed183f517d66c93f22b7cd216b6b0cfd2444191088a86af ed25519:2JvmJLCnRfPLzUnYHZsEhSKcNLw7E2qFPAD8U3gmX2HU 30000000 --keyPath /Users/jameswalstonn/.unc/Unc2/validator_key.json
sleep 2
near stake 79d96a47f387ae8f8e92a7f5b42e75d86c31c680581ad77cef1115a5b76b6e3b ed25519:9CeceB9q57XdrFgE58byk9RpNyH4cotbRXFZSqLKcW6E 20000000 --keyPath /Users/jameswalstonn/.unc/Unc3/validator_key.json
sleep 2
near stake e2678f53a51a46a8c76639cf37ed6c6070b995ed759d6fff0fad1c25ee87057d ed25519:GEnjFuPBKc5LxX5YPVoYyRBZ5tyCfU2Vz9Sf1PvABwhN 10000000 --keyPath /Users/jameswalstonn/.unc/Unc4/validator_key.json

## Register Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/keys/batch_register_rsa1.json with-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/keys/unc.json send
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/keys/batch_register_rsa2.json with-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/keys/unc.json send

## Challenge Rsa2048 keys
#sleep 5
#/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa 13735a00a19b0b572ed183f517d66c93f22b7cd216b6b0cfd2444191088a86af use-file /Users/jameswalstonn/.unc/keys/challenge1.json without-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/Unc2/signer_key.json send
#sleep 5
#/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa 13735a00a19b0b572ed183f517d66c93f22b7cd216b6b0cfd2444191088a86af use-file /Users/jameswalstonn/.unc/keys/challenge2.json without-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/Unc2/signer_key.json send
#sleep 5
#/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa 79d96a47f387ae8f8e92a7f5b42e75d86c31c680581ad77cef1115a5b76b6e3b use-file /Users/jameswalstonn/.unc/keys/challenge3.json without-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/Unc3/signer_key.json send
#sleep 5
#/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa 79d96a47f387ae8f8e92a7f5b42e75d86c31c680581ad77cef1115a5b76b6e3b use-file /Users/jameswalstonn/.unc/keys/challenge4.json without-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/Unc3/signer_key.json send
