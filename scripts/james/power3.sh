#!/bin/bash
## Challenge Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/keys/batch_register_rsa3.json with-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/keys/unc.json send
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa e2678f53a51a46a8c76639cf37ed6c6070b995ed759d6fff0fad1c25ee87057d use-file /Users/jameswalstonn/.unc/keys/challenge5.json without-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/Unc4/signer_key.json send
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions create-challenge-rsa e2678f53a51a46a8c76639cf37ed6c6070b995ed759d6fff0fad1c25ee87057d use-file /Users/jameswalstonn/.unc/keys/challenge6.json without-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/Unc4/signer_key.json send
