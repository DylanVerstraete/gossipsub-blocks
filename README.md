# Gossipsub blocks

A basic gossip pub sub block program using `web3` and `libp2p`

## Setup

### Run hardhat node

```sh
cd node
npx hardhat node
```

### Open terminal pane 1 (publisher)

```sh
cargo run --bin pub
```

### Open terminal pane 2 (publisher)

```sh
cargo run -- bin pub
```

### Open terminal pane 3 (subscriber)

```sh
cargo run --bin sub
```
