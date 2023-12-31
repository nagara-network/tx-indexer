# `nagara` Network - Transaction Indexer Service

## Environment Variables

- `ENDPOINT_SOCKET`; default is `"0.0.0.0:8765"`
- `RPC_URI`; default is `"wss://boot.nagara.network"`

## Response Structure - `RelatedTransaction`

Fields:

- `id`: database sequence number, ie. `4`
- `hash`: block hash, ie. `0x3cabe3fa69e036abfb779fcd9923723de9c75cf94558d9303b63459fa69a847e`
- `sender`: transaction origin, ie. `gr6mwvfrS6KcnHtTKm8cY5QtnjWyPzMaiDV85SYq1yAfW8D5v`
- `receiver`: transaction destination, ie. `gr3gnBh93aFoyYgiZdcfPc4FbuhwocFpqA8GFETNMbQ92rQzx`
- `amount`: transaction amount in `integer * 10^9`, ie. `1000000000000000000000` which equals to 1,000,000,000,000 `NGR`
- `amount_str`: transaction amount in string with its unit, ie. `1000000000000000000000` which equals to `1 TNGR`
- `fee`: total fee paid by `sender` in `integer * 10^9`, ie. `297571000` which equals to 0.297571000 `NGR`
- `fee_str`: total fee paid by `sender` in string with its unit, ie. `297571000` which equals to `0.2975 NGR`
- `blocknumber`: transaction's finalized block, ie. `4668`
- `unixtime`: block timestamp in `rfc3339 format of UTC`, ie. `"2023-07-26T06:14:50Z"`

## Querying Transactions

`BASE_URL` in production is `https://tx-indexer.nagara.network`, otherwise it is `http://[ENDPOINT_SOCKET]`.

### Query by Account

Request:

- Path: `GET /account/{ss58-account}`, ie. `GET /account/gr6mwvfrS6KcnHtTKm8cY5QtnjWyPzMaiDV85SYq1yAfW8D5v`
- [Optional] Query Parameter `from_utc` (inclusive): `rfc3339 format of UTC`, ie. `2019-10-12T07:20:50.52Z`
- [Optional] Query Parameter `to_utc` (inclusive): `rfc3339 format of UTC`, ie. `2019-10-12T07:20:50.52Z`
- [Optional] Query Parameter `limit` (count): `positive integer`, ie. `5`

Response:

- JSON Array of [RelatedTransaction](#response-structure---relatedtransaction)
