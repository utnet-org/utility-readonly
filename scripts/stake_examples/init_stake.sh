#!/bin/bash

utility/target/debug/uncd  --home "~/.unc/Unc1" run  > "~/.unc/logfiles/logfile1" 2>&1 &
sleep 1
utility/target/debug/uncd  --home "~/.unc/Unc2" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "~/.unc/logfiles/logfile2" 2>&1 &
sleep 1
utility/target/debug/uncd  --home "~/.unc/Unc3" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "~/.unc/logfiles/logfile3" 2>&1 &
sleep 1
utility/target/debug/uncd  --home "~/.unc/Unc4" run --boot-nodes "ed25519:9e9JtarsJc3JR1PcnU6ykQgUmEf6LCAQi4ZjZjMuxrip@127.0.0.1:24567" > "~/.unc/logfiles/logfile4" 2>&1 &
sleep 10

## create new accounts
./target/debug/unc account create-account fund-later use-auto-generation save-to-folder ~/.unc-credentials/implicit
./target/debug/unc account create-account fund-later use-auto-generation save-to-folder ~/.unc-credentials/implicit
./target/debug/unc account create-account fund-later use-auto-generation save-to-folder ~/.unc-credentials/implicit
## as follows:
## 16438e347058391fdfdd98f13d0bf4fd4d64267d59b67328579d51846565ce9b.json
## 8e42ce2442abe82f49be2cc44d7b6216f406621da3c453f17f286fe78952d389.json
## 41e2f1cc1b5133917ba8b9e49f74e9cb57e45b0f4c2672830659ab8287168a87.json


## send money to other accounts
utility-cli/target/debug/unc tokens miner send-unc 16438e347058391fdfdd98f13d0bf4fd4d64267d59b67328579d51846565ce9b '35000000 unc' network-config testnet sign-with-keychain send
sleep 2
utility-cli/target/debug/unc tokens miner send-unc 8e42ce2442abe82f49be2cc44d7b6216f406621da3c453f17f286fe78952d389 '25000000 unc' network-config testnet sign-with-keychain send
sleep 2
utility-cli/target/debug/unc tokens miner send-unc 41e2f1cc1b5133917ba8b9e49f74e9cb57e45b0f4c2672830659ab8287168a87 '20000000 unc' network-config testnet sign-with-keychain send


## cargo install unc-validator
## pledge new accounts
sleep 2
utility-cli/target/debug/unc validator staking pledge-proposal 16438e347058391fdfdd98f13d0bf4fd4d64267d59b67328579d51846565ce9b ed25519:2JvmJLCnRfPLzUnYHZsEhSKcNLw7E2qFPAD8U3gmX2HU '30000000 unc' network-config testnet sign-with-keychain send
sleep 2
utility-cli/target/debug/unc validator staking pledge-proposal 8e42ce2442abe82f49be2cc44d7b6216f406621da3c453f17f286fe78952d389 ed25519:9CeceB9q57XdrFgE58byk9RpNyH4cotbRXFZSqLKcW6E '20000000 unc' network-config testnet sign-with-keychain send
sleep 2
utility-cli/target/debug/unc validator staking pledge-proposal 41e2f1cc1b5133917ba8b9e49f74e9cb57e45b0f4c2672830659ab8287168a87 ed25519:2VuiWqdedrmv9FNxRWFomr77hgykwXgrhptdKbHcgoFp '30000000 unc' network-config testnet sign-with-plaintext-private-key --signer-public-key ed25519:2VuiWqdedrmv9FNxRWFomr77hgykwXgrhptdKbHcgoFp --signer-private-key ed25519:3NWVkj5Gnz6obeGUBJK7NFeVzErm7uKGgnbnitQgVyXbDVjADTYLwNBPBBGKYqQJwcPTfBfB4wJwT8hhjxHDHFf8 send


## view tx status
./target/debug/unc transaction view-status EWHzhriCRTbDVd9SH6Vk88hSHqzJ7pipXW6eUhWTBkvS network-config testnet