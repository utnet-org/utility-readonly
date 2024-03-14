#!/bin/bash
## Register Rsa2048 keys
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/keys/batch_register_rsa1.json with-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/keys/unc.json send
sleep 5
/Library/WebServer/Documents/utility-cli-rs/target/debug/unc extensions register-rsa-keys unc use-file /Users/jameswalstonn/.unc/keys/batch_register_rsa2.json with-init-call network-config custom sign-with-access-key-file /Users/jameswalstonn/.unc/keys/unc.json send
