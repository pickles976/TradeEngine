# TradeEngine

![Tests](https://github.com/pickles976/TradeEngine/actions/workflows/test.yaml/badge.svg)
![WASM Build](https://github.com/pickles976/TradeEngine/actions/workflows/build.yaml/badge.svg)

A tiny market engine for matching buy/sell orders. Supports limit orders as well as market orders. Meant to be wrapped in a higher-level API. 

The general idea is to be able to use this to host an amateur virtual marketplace, on the scale of the Runescape GE or somthing similar. I created it to eventually add to a game I'm working on.

A larger marketplace could be made by sharding ledgers by item type across multiple machines. I intend to use sqlite to handle user data storage and transaction history just because of its ease of use and portability, but you could use a more persistent data storage, you just need to write a wrapper for it.

This system offers no security, guarantee of transaction correctness, or persistence of data.


## Usage

### Building

If using with node in the backend (or in the browser for some reason?) export as WASM package

```shell
wasm-pack build --target nodejs
```

### Importing

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

### Available methods

```javascript

market.buy(order)
market.sell(order)

// TODO: add these
// market.market_buy(order)
// market.market_sell(order)

market.query_ledger(item_string)
market.cancel_order(item_string, order)

market.get_best_buying_price(item_string)
market.get_best_selling_price(item_string)


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

- [x] Fix segfault in query ledger
- [ ] Add market buy methods
- [ ] Ability to serialize and deserialize entire market to json or sqlite file to save snapshots of market status
