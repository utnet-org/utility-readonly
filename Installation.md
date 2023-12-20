## Utility node installation


### Prerequisites

- Rust programming environment
- Cargo, Rust's package manager

#### Get the source code

download code from github repo

```bash

git clone git@github.com:utnet-org/utility.git

go to the project root path

```bash

cd /your/path/to/utility

```

```
switch to branch develp

```bash

git checkout develop

```
### Build the Project

Use Cargo to build the project:

```bash
cargo build --release
```

### Init node0

initial node before running neard

```bash
./target/debug/neard --home="~/Library/Near0" init --chain-id="my-private-chain-id" --account-id=superaccount
```

### Run node0

create logfiles folder in project root or any other location 

```bash
sudo mkdir -p logfiles
```

run node in background

```bash
./target/debug/neard --home="~/Library/Near0" run > logfiles/logfile1 2>&1 &
```

## Near Cli installation

### near cli installation and config

install near-cli 

```bash
npm install -g near-cli
```

set environment variable for near-cli to config the blockchain's network

```bash
export NEAR_ENV=mainnet
export NEAR_CLI_LOCALNET_KEY_PATH=[/path/to/node0/key/validator_key.json]
```

### near cli usage

#### get account state

```bash
near state [account-id]
```

## Run new node - node1
### customize near-cli
open near-cli project with your IDE which alway located at ：

```bash
/usr/local/lib/node_modules/near-cli
```
locate file commands/create-account.js
and modify const TLA_MIN_LENGTH to 1

```bash
const TLA_MIN_LENGTH = 1;
```

### init node1 in nearcore project root

```bash
./target/debug/neard --home="~/Library/Near1" init --chain-id="my-private-chain-id" --account-id=node1-validator
```
- the above cli will generate config files for node1, 

- delete genesis.json file in node1, and copy [genesis.json](genesis.json) from node0
  
- get public key for node1-validator from validator.json,
  
- and then create node1-validator from node0
 ```bash
near create-account node1-validator --masterAccount superaccount --initialBalance 10000 --networkId my-private-chain-id --publicKey [public-key you get from above step]
 ```

### stake node1 account

update environment variable NEAR_CLI_LOCALNET_KEY_PATH to point to node1's validator_key.json

```bash
export NEAR_CLI_LOCALNET_KEY_PATH=[/path/to/node1/key/validator_key.json]
```

stake node1-validator via near-cli

```bash
near stake <accountId> <stakingKey> <amount>
```

### run node1

modify near1's config.json file
```bash
rpc. "addr": "0.0.0.0:3031",
network. "addr": "0.0.0.0:24568",
```
run node1 with bootnodes of node0
```bash
./target/debug/neard --home="~/Library/Near1" run --boot-nodes="ed25519:DVtQUJKcUNZkN1qPSHWpZAvYUSjGoByqc5HeySnVASbs@192.168.10.4(198.18.194.73):24567" > logfiles/logfile2 2>&1 &
```

### run nodex

```bash
{
  "account_id": "node2-validator",
  "public_key": "ed25519:znYkQr9s94EmKXWFdRA9xrNTfHN125EKEVqAqSqQYzz",
  "secret_key": "ed25519:484kNA2u6mVXBrn58sRsmsmp5GsDgKbjWnZzPDswnoQuAJTeiBsqbmzcur8KfWsvQhyXmudkBNMx1gQMoknUpcZc"
}
```
