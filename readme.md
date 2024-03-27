# Insacn (Inscription Scan)

A [Rust](https://www.rust-lang.org/) tool that can extract inscription data (e.g. ordinals, arc20, brc20, runes, src20...) from blocks and transactions on [Bitcoin](https://bitcoin.org/).

## Supported Protocols
- **Ordinals**
    - [ ] Inscription: ✔️`mint`, ✖️`transfer`
    - [x] BRC20: ✔️`deploy`, ✔️`mint`, ✔️`inscripbeTransfer`, ✖️`transfer`
    - [x] BRC100: ✔️`deploy`, ✔️`mint`, ✔️`inscripbeTransfer`, ✖️`transfer`
    - [x] BRC420: ✔️`deploy`, ✔️`mint`, ✖️`transfer`
    - [x] Bitmap: ✔️`mint`, ✖️`transfer`
    - [x] SNS: ✔️`deploy`, ✔️`mint`, ✖️`transfer`
    - [x] Tap: ✔️`deploy`, ✔️`mint`, ✔️`inscripbeTransfer`, ✖️`transfer`
- **Atomicals**
    - [x] ARC20: ✔️`dft`(deploy), ✔️`ft`(mint), ✔️`dmt`(mint), ✔️`y`(split), ✖️`transfer`
    - [ ] Atom-NFT: ✔️`nft`->`request_container`, ✔️`nft`->`request_dmitem`, `nft`, ✖️`transfer`
    - [ ] Realm: ✔️`nft`->`request_realm`, ✔️`nft`->`request_subrealm`, ✖️`transfer`
    - [ ] Atom-Others: `mod`, `evt`, `dat`, `sl`, `x`
- **Stamps**
    - [ ] SRC20: `deploy`, `mint`, `transfer`
    - [ ] SRC721: 
- **Runes**
    - [ ] Runestone: 
    - [ ] Rune Alpha: 

> Note: it's only scan(decode) protocols stored in raw transaction data, not include the whole indexing data. So the `common` transaction depend on indexing data which marked as `✖️` will not be included.

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
- [ ] Indexer: add data indxer for each protocol to identify the invalid event and support transfer event
- [ ] API: 
- [ ] Optimize: iterator each txs of block and check which protocol this tx is.

## Reference
- https://github.com/ordinals/ord
- https://www.gate.io/zh/inscription/bitcoin/cbrc-20