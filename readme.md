# Insacn (Inscription Scan)

A [Rust](https://www.rust-lang.org/) tool that can decode/index inscription events data (e.g. ordinals, arc20, brc20, runes, src20...) from blocks and transactions on [Bitcoin](https://bitcoin.org/).

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
- **Runes**
    - [x] rune-stone: ✔️`etching`(deploy), ✔️`edicts`(transfer), ✔️`mint`, ✖️`transfer`
    - [x] rune-alpha: ✔️`etching`(deploy), ✔️`edicts`(transfer), ✔️`mint`, ✖️`transfer`
- **Stamps**
    - [x] stamp-src20: ✔️`deploy`, ✔️`mint`, ✔️`transfer`
    - [ ] stamp-src721: ✖️`mint`, ✖️`transfer`

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

1. Decode from block
    ``` bash
    inscan -u devnet -w devnet --protocol ord-brc20 --out-file examples/block-838266.jsonl \
        decode --block 838266 #range blocks 838266:838270 or multi blocks 838266,838275,838279
    ```

2. Decode from transaction id
    ``` bash
    inscan -u devnet -w devnet --protocol ord-brc20 --out-file examples/aaabbb3.jsonl \
        decode --txid c631181e8f7740064ec5e832d773086369d30f5297713a0b098d6d95ffe0c78b
        #multi txids 913bebf12d6030a092890d22dbc565df2b2f32b33876568bca19e7e92fbe4f77,c631181e8f7740064ec5e832d773086369d30f5297713a0b098d6d95ffe0c78b

    ```
3. Index all blocks start from 838250
    ```bash
    inscan -u devnet -w devnet --protocol ord-brc20 --out-file examples/block-838266.jsonl \
        index --start 838266
    ```
4. save data to postgres
    ```bash
    inscan -u devnet -w devnet --protocol ord-brc20 --out-db postgres://postgres:postgres@localhost/postgres \
        index --start 838266
    ```

## Output
- **local jsonl file**: the output `jsonl` format is a nested line structures json, more details can be found at: [docs/data-structure.md](docs/data-structure.md)
- **database postgres**: save the event data to postgres.



## Reference
- https://github.com/ordinals/ord
- https://github.com/atomicals/atomicals-electrumx