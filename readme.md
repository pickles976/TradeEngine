# MarketCore

A tiny market engine for matching buy/sell orders. Supports limit orders as well as market orders. Meant to be wrapped in a higher-level API.

## Usage

If using with node in the backend (or in the browser for some reason?) export as WASM package

```
wasm-pack build --target nodejs
```

In your Node project call it like:

```
import { MarketWrapper, test } from './pkg/MarketCore.js'
let market = MarketWrapper.new()

let order = {
    user_id: "ALICE",
    item: "CORN",
    amount: 200,
    price_per: 12.0
}
order = JSON.stringify(order)

let summary = market.buy(order)
console.log(summary)
```