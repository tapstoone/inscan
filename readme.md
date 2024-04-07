# Insacn (Inscription Scan)

A [Rust](https://www.rust-lang.org/) tool that can extract inscription data (e.g. ordinals, arc20, brc20, runes, src20...) from blocks and transactions on [Bitcoin](https://bitcoin.org/).

## Supported Protocols
- **Ordinals**
    - [x] ord: ✔️`mint`, ✖️`transfer`
    - [x] ord-brc20: ✔️`deploy`, ✔️`mint`, ✔️`inscripbeTransfer`, ✖️`transfer`
    - [x] ord-brc100: ✔️`deploy`, ✔️`mint`, ✔️`inscripbeTransfer`, ✖️`transfer`
    - [x] ord-brc420: ✔️`deploy`, ✔️`mint`, ✖️`transfer`
    - [x] ord-bitmap: ✔️`mint`, ✖️`transfer`
    - [x] ord-sns: ✔️`deploy`, ✔️`mint`, ✖️`transfer`
    - [x] ord-tap: ✔️`deploy`, ✔️`mint`, ✔️`inscripbeTransfer`, ✖️`transfer`
- **Atomicals**
    - [x] atom-arc20: ✔️`dft`(deploy), ✔️`ft`(mint), ✔️`dmt`(mint), ✔️`y`(split), ✖️`transfer`
    - [x] atom-nft: ✔️`nft`->`request_container`, ✔️`nft`->`request_dmitem`, `nft`, ✖️`transfer` Note: bytes was encoded in base64
    - [x] atom-realm: ✔️`nft`->`request_realm`, ✔️`nft`->`request_subrealm`, ✖️`transfer`
    - [x] atom-others: ✔️`mod`, ✔️`evt`, ✔️`dat`, ✔️`sl`, ✔️`x`
- **Stamps**
    - [x] stamp-src20: ✔️`deploy`, ✔️`mint`, ✔️`transfer`
    - [ ] stamp-src721: ✖️`mint`, ✖️`transfer`
- **Runes**
    - [ ] rune-stone: 
    - [x] rune-alpha: ✔️`etching`(deploy), ✔️`edicts`(transfer), ✔️`mint`

> Note: 
>1. It's only scan(decode) protocols stored in raw transaction data, not include the whole indexing data. So the `common` transaction depend on indexing data which marked as `✖️` will not be included.
>2. Currently, the protocols only perform basic checks and do not perform strict verification. It should not used in production environments.

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
    inscan \
        --rpc-user username \
        --rpc-pass password \
    decode \
        --height 836068 \
        --protocol rune-alpha \
        --output output/836068.jsonl
    ```

2. Extract inscriptions from transactions
    ``` bash
    inscan \
        --rpc-user username \
        --rpc-pass password \
    decode \
        --txid 913bebf12d6030a092890d22dbc565df2b2f32b33876568bca19e7e92fbe4f77 \
        --protocol ord \
        --output output/ord-4f77.jsonl
    ```

## Reference
- https://github.com/ordinals/ord
- https://github.com/atomicals/atomicals-electrumx