# `nagara` Network - Transfer Indexer Service

## Environment Variables

- `REDIS_CONNECTION_URL`; example: `"redis://127.0.0.1:6379/42"`
- `MYSQL_CONNECTION_URL`; example: `"mysql://user:password@127.0.0.1:3306/devnet_tx_indexer"`
- `CHAIN_URL`; example: `"wss://testnet-boot.nagara.network:443"`
- `WORKER_COUNT`; example: `"32"`

## Response Structure - `TransferHistory`

Fields:

- `id`: database sequence number, ie. `4`
- `hash`: block hash, ie. `0x3cabe3fa69e036abfb779fcd9923723de9c75cf94558d9303b63459fa69a847e`
- `sender`: transfer origin, ie. `gr6mwvfrS6KcnHtTKm8cY5QtnjWyPzMaiDV85SYq1yAfW8D5v`
- `receiver`: transfer destination, ie. `gr3gnBh93aFoyYgiZdcfPc4FbuhwocFpqA8GFETNMbQ92rQzx`
- `amount`: transfer amount in `integer * 10^9`, ie. `1000000000000000000000` which equals to 1,000,000,000,000 `NGR`
- `amount_str`: transfer amount in string with its unit, ie. `1000000000000000000000` which equals to `1 TNGR`
- `fee`: total fee paid by `sender` in `integer * 10^9`, ie. `297571000` which equals to 0.297571000 `NGR`
- `fee_str`: total fee paid by `sender` in string with its unit, ie. `297571000` which equals to `0.2975 NGR`
- `total_amount_str`: total balance deducted from `sender` in string with its unit, ie. `1000000000000297571000` which equals to `1 TNGR`
- `blocknumber`: transfer's finalized block, ie. `4668`
- `unixtime`: block timestamp in `rfc3339 format of UTC`, ie. `"2023-07-26T06:14:50Z"`

## Querying transfers

`BASE_URL` in production is `https://tx-indexer.nagara.network`, otherwise it is `http://[local-ip]:8686`.

### Query by Account

Request:

- Path: `GET /account/{ss58-account}` or `GET /account/0x{hex-public-key}`, ie. `GET /account/gr6mwvfrS6KcnHtTKm8cY5QtnjWyPzMaiDV85SYq1yAfW8D5v`
- [Optional] Query Parameter `from_utc` (inclusive): `rfc3339 format of UTC`, ie. `2019-10-12T07:20:50.52Z`
- [Optional] Query Parameter `to_utc` (inclusive): `rfc3339 format of UTC`, ie. `2019-10-12T07:20:50.52Z`
- [Optional] Query Parameter `limit` (count): `positive integer`, ie. `5`

Response:

- JSON Array of [TransferHistory](#response-structure---transferhistory)
