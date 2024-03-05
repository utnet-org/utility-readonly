#!/bin/bash
set -eux

account_id=$(grep account_id /home/ubuntu/.near/shardnet/validator_key.json | awk -F'"' '{print $4}')
mkdir -p /home/ubuntu/.unc-credentials/shardnet/
printf '{"account_id":"near","public_key":"%s","private_key":"%s"}' \
    "${1:?}" "${2:?}" > /home/ubuntu/.unc-credentials/shardnet/near.json
pk=$(grep public_key /home/ubuntu/.near/shardnet/validator_key.json | awk -F'"' '{print $4}')
cp /home/ubuntu/.near/shardnet/validator_key.json /home/ubuntu/.unc-credentials/shardnet/"$account_id".json
unc_ENV=shardnet near --nodeUrl=http://127.0.0.1:3030 \
        create-account "$account_id" --masterAccount near \
        --initialBalance 1000000 --publicKey "$pk"
