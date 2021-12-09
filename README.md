## Getting Started


### Install Guide

Follow [Setup](https://docs.substrate.io/v3/getting-started/installation/) to guide you install the CESS development.

### Build Node

The `cargo run` command will perform an initial build. Use the following command to build the node without launching it:

```
# Fetch the code
git clone https://github.com/CESSProject/cess-node.git
cd cess-node

# Build the node (The first build will be long (~30min))
cargo build --release
```

## Run The CESS Node


After the node has finished compiling, you can follow these steps below to run it. 

### Generate Keys

If you already have keys for Substrate using the [SS58 address encoding format](https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58)), please see the next section.

Begin by compiling and installing the utility ([instructions and more info here](https://substrate.dev/docs/en/knowledgebase/integrate/subkey)). 

Generate a mnemonic (Secret phrase) and see the `sr25519` key and address associated with it.

```
# subkey command
subkey generate --scheme sr25519
```

Now see the `ed25519` key and address associated with the same mnemonic (Secret phrase).

```
# subkey command
subkey inspect --scheme ed25519 "SECRET PHRASE YOU JUST GENERATED"
```

We recommend that you record the above outputs and keep mnemonic in safe.

### Run CESS-Hacknet

Launch node on the cess-hacknet with:

```
# start
./target/release/cess-node --base-path /tmp/cess --chain cess-hacknet
```

Then you can add an account with:

```
# create key file
vim secretKey.txt

# add secret phrase for the node in the file
YOUR ACCOUNT'S SECRET PHRASE
```

```
# add key to node
./target/release/cess-node key insert --base-path /tmp/cess --chain cess-hacknet --scheme Sr25519  --key-type babe --suri /root/secretKey.txt

./target/release/cess-node key insert --base-path /tmp/cess --chain cess-hacknet --scheme Ed25519  --key-type gran --suri /root/secretKey.txt
```

Now you can launch node again:

```
# start
./target/release/cess-node --base-path /tmp/cess --chain cess-hacknet
```

### Run in Docker

Install [Docker](https://docs.docker.com/get-docker/) first, and run the following command to start a node on the CESS-Hacknet:

```
docker pull cesstech/cess-hacknet:v0.0.1
docker run --network host cesstech/cess-hacknet:v0.0.1 ./CESS-v0.0.1/target/release/cess-node --base-path /tmp/cess --chain cess-hacknet
```

## Run Tests


CESS has Rust unit tests, and can be run locally.

```
# Run all the Rust unit tests
cargo test --release
```

## Module Documentation


* [Files Bank](https://github.com/CESSProject/cess-node/tree/main/c-pallets/file-bank)
* [Segment Book](https://github.com/CESSProject/cess-node/tree/main/c-pallets/segment-book)
* [Sminer](https://github.com/CESSProject/cess-node/tree/main/c-pallets/sminer)
