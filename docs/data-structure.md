## output format

### jsonl
the output `jsonl` format is a nested line structures json:
```json
{"height": 324,"blocktime": "2023-04-03","txhash": "jfidlajsoier920i43902","txindex": 12,"protocol": "rune-alpha","payload": {"edicts": [],"etching": {"divisibility": 4,"limit": 21000000,"rune": "THOR","symbol": null,"term": 2541777},"burn": false}}
{"height": 324,"blocktime": "2023-04-03","txhash": "jfidlajsoier920i43902","txindex": 12,"protocol": "rune-alpha","payload": {"edicts": [],"etching": {"divisibility": 4,"limit": 21000000,"rune": "THOR","symbol": null,"term": 2541777},"burn": false}}
...
```

You can convert it to pretty format.

```json
{
    "height": 324,
    "blocktime": "2023-04-03",
    "txhash": "jfidlajsoier920i43902",
    "txindex": 12,
    "protocol": "rune-alpha",
    "payload": {
        "edicts": [],
        "etching": {
            "divisibility": 4,
            "limit": 21000000,
            "rune": "THOR",
            "symbol": null,
            "term": 2541777
        },
        "burn": false
    }
}
...
```



