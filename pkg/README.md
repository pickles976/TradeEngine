# MarketCore

A tiny market engine for matching buy/sell orders. Supports limit orders as well as market orders. Meant to be wrapped in a higher-level API.

## Usage

If using with node in the backend (or in the browser for some reason?) export as WASM package

```shell
wasm-pack build --target nodejs
```

In your Node project call it like:

```javascript
import { MarketWrapper, test } from "./pkg/MarketCore.js";
let market = MarketWrapper.new();

let order = {
  user_id: "ALICE",
  item: "CORN",
  amount: 200,
  price_per: 12.0,
};
order = JSON.stringify(order);

let summary = market.buy(order);
console.log(summary);
```

## Methodology

The market is a hashmap of ledgers. Each ledger has a vector of buy orders and sell orders, which are ordered in ascending value.

When an order is placed, it comes in as an OrderRequest. An OrderRequest can be a limit buy, limit sell, market buy, or market sell. If there is a corresponding order in the ledger, a transaction will occur. Partial order completions create a transaction as well as an order on the ledger. The data returned to javascript after placing an order looks like this:

```json
{
  "key": "PIKMIN",
  "transactions": [],
  "to_update": [],
  "created": {
    "id": "96e6a9ed-b95c-4302-9e6b-abf740aef559",
    "user_id": "ALICE",
    "kind": "SELL",
    "amount": 10,
    "price_per": 2.5
  }
}
```

created -- is the Order that was created.

transactions -- is the list of transactions which occured

to_update -- the new state of an order inside the ledger. Occurs when a partial transaction has happened.

## TODO

- [x] Add query_ledger to wasm
- [x] Add cancel_order to wasm
- [x] Add ability to handle invalid UUID to cancel_order
- [x] Add get_best_selling_price to wasm
- [x] Add get_best_selling_price to wasm
