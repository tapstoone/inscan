# Insacn (Inscription Scan)

A [Rust](https://www.rust-lang.org/) tool that can extract inscription data (e.g. ordinals, arc20, brc20, runes, src20...) from blocks and transactions on [Bitcoin](https://bitcoin.org/).

## Supported Protocols
- Ordinals
    - [ ] NFT
    - [x] BRC20
    - [ ] BRC100
    - [ ] BRC420
    - [ ] bitmap
    - [ ] SNS
    - [x] Tap
- Atomicals
    - [ ] ARC20
    - [ ] ARC721
- Stamps
    - [ ] SRC20
    - [ ] SRC721
- Runes
    - [ ] Runestone
    - [ ] Rune Alpha


## Install
```
git clone 
cd inscan
cargo build --release
```


## Usage
`inscan` requires a synced bitcoind node with `-txindex`. `inscan` communicates with bitcoind via RPC to retrive data. 

1. Extract inscriptions from blocks.
    ``` bash
    inscan 
        --rpc-user
        --rpc-pass
    blocks
        --heights
        --protocol
        --output
        --help
    ```

2. Extract inscriptions from transactions
    ``` bash
    inscan 
        --rpc-user
        --rpc-pass
    txs
        --txids
        --protocol
        --output
        --help
    ```

## TODO
- [ ] speed up: iterator each txs of block and check which protocol this tx is.

## Reference
- https://github.com/ordinals/ord
- https://www.gate.io/zh/inscription/bitcoin/cbrc-20