use std::time::Instant;
use rand::{seq::SliceRandom, Rng}; // 0.7.2

use MarketCore::{self, structs::{OrderRequest, OrderKind}, market::Market};
use uuid::Uuid;
use wildmatch::WildMatch;

#[test]
fn test_buy() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);

    exchange.place_order(order1);
    exchange.place_order(order2);

    let buy_orders = &exchange.map.get("CORN").unwrap().buy_orders;

    let test_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 32, price_per: OrderedFloat(12.0) }, Order { id: *, user_id: \"ALICE\", kind: BUY, amount: 12, price_per: OrderedFloat(14.0) }]";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", buy_orders).as_str()));

}

#[test]
fn test_sell() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 20, 10.0);
    let order2 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 14, 15.0);

    exchange.place_order(order1);
    exchange.place_order(order2);

    let sell_orders = &exchange.map.get("CORN").unwrap().sell_orders;

    let test_str = "[Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 20, price_per: OrderedFloat(10.0) }, Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 14, price_per: OrderedFloat(15.0) }]";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", sell_orders).as_str()));

}

#[test]
fn test_buy_and_sell() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);
    let order3 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 20, 10.0);
    let order4 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 14, 15.0);

    exchange.place_order(order1);
    exchange.place_order(order2);
    exchange.place_order(order3);
    exchange.place_order(order4);

    let ledger = exchange.map.get("CORN").unwrap();

    let buy_orders = &ledger.buy_orders;
    let sell_orders = &ledger.sell_orders;

    let buy_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 24, price_per: OrderedFloat(12.0) }]";
    let sell_str = "[Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 14, price_per: OrderedFloat(15.0) }]";

    assert!(WildMatch::new(buy_str).matches(format!("{:?}", buy_orders).as_str()));
    assert!(WildMatch::new(sell_str).matches(format!("{:?}", sell_orders).as_str()));

}

#[test]
fn test_buy_and_sell_and_buy() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);
    let order3 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 20, 10.0);
    let order4 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 14, 15.0);
    let order5 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 14, 16.0);

    exchange.place_order(order1);
    exchange.place_order(order2);
    exchange.place_order(order3);
    exchange.place_order(order4);
    exchange.place_order(order5);

    let ledger = exchange.map.get("CORN").unwrap();

    let buy_orders = &ledger.buy_orders;
    let sell_orders = &ledger.sell_orders;

    let buy_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 24, price_per: OrderedFloat(12.0) }]";
    let sell_str = "[]";

    assert!(WildMatch::new(buy_str).matches(format!("{:?}", buy_orders).as_str()));
    assert!(WildMatch::new(sell_str).matches(format!("{:?}", sell_orders).as_str()));

}

#[test]
fn test_summary() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 12, 14.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::SELL, 32, 12.0);

    exchange.place_order(order1);
    let summary = exchange.place_order(order2);

    // println!("{:?}", summary);

    let transactions_str = "[Transaction { buyer: \"ALICE\", seller: \"BOB\", amount: 12, price_per: 12.0 }]";
    let to_update_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 0, price_per: OrderedFloat(14.0) }]";
    let created_str = "Some(Order { id: *, user_id: \"ALICE\", kind: SELL, amount: 20, price_per: OrderedFloat(12.0) })";

    assert_eq!("CORN", summary.key);
    println!("{:?}", summary.transactions);
    assert!(WildMatch::new(transactions_str).matches(format!("{:?}", summary.transactions).as_str()));
    assert!(WildMatch::new(to_update_str).matches(format!("{:?}", summary.to_update).as_str()));
    assert!(WildMatch::new(created_str).matches(format!("{:?}", summary.created).as_str()));

}

#[test]
fn test_cancel_buy_order_full() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 12, 14.0);

    let summary = exchange.place_order(order1);
    let order = summary.created.unwrap();
    let item = summary.key;

    exchange.cancel_order(item.clone(), order);

    assert_eq!(exchange.map.get(&item).unwrap().buy_orders.len(), 0);

}

#[test]
fn test_cancel_sell_order_full() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::SELL, 12, 14.0);

    let summary = exchange.place_order(order1);
    let order = summary.created.unwrap();
    let item = summary.key;

    exchange.cancel_order(item.clone(), order);

    assert_eq!(exchange.map.get(&item).unwrap().sell_orders.len(), 0);

}

#[test]
fn test_cancel_partial() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 12, 14.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(), OrderKind::SELL, 6, 12.0);

    exchange.place_order(order1);
    let summary = exchange.place_order(order2);

    let order = summary.to_update[0].clone();
    let item = summary.key;

    exchange.cancel_order(item.clone(), order);

    assert_eq!(exchange.map.get(&item).unwrap().buy_orders.len(), 0);

}

#[test]
fn test_cancel_failure_bad_item() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 12, 14.0);

    let summary = exchange.place_order(order1);

    let order = summary.created.unwrap();
    let item = summary.key;

    println!("{:?}", order);
    println!("{:?}", exchange.query_ledger("CORN".to_string()).unwrap());

    exchange.cancel_order("BRUH".to_string(), order);

    assert_eq!(exchange.map.get(&item).unwrap().buy_orders.len(), 1);

}

#[test]
fn test_cancel_failure_bad_order() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 12, 14.0);

    let summary = exchange.place_order(order1);

    let mut order = summary.created.unwrap();
    order.id = Uuid::new_v4();
    let item = summary.key;

    println!("{:?}", order);
    println!("{:?}", exchange.query_ledger("CORN".to_string()).unwrap());

    exchange.cancel_order("CORN".to_string(), order);

    assert_eq!(exchange.map.get(&item).unwrap().buy_orders.len(), 1);

}

#[test]
fn test_query() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);

    exchange.place_order(order1);
    exchange.place_order(order2);

    let test_str = "Some(Ledger { buy_orders: [Order { id: *, user_id: \"BOB\", kind: BUY, amount: 32, price_per: OrderedFloat(12.0) }, Order { id: *, user_id: \"ALICE\", kind: BUY, amount: 12, price_per: OrderedFloat(14.0) }], sell_orders: [] })";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", exchange.query_ledger("CORN".to_string())).as_str()));
    assert_eq!(None, exchange.query_ledger("STUFF".to_string()));

}

#[test]
fn test_market_buy() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::SELL, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::SELL, 12, 14.0);
    let order3 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::MARKET_BUY, 34, 0.0);

    exchange.place_order(order1);
    exchange.place_order(order2);
    exchange.place_order(order3);

    let sell_orders = &exchange.map.get("CORN").unwrap().sell_orders;

    let test_str = "[Order { id: *, user_id: \"ALICE\", kind: SELL, amount: 10, price_per: OrderedFloat(14.0) }]";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", sell_orders).as_str()));

}

#[test]
fn test_market_sell() {

    let mut exchange = Market::new();

    let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);
    let order3 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::MARKET_SELL, 34, 0.0);

    exchange.place_order(order1);
    exchange.place_order(order2);
    exchange.place_order(order3);

    let buy_orders = &exchange.map.get("CORN").unwrap().buy_orders;

    let test_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 10, price_per: OrderedFloat(12.0) }]";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", buy_orders).as_str()));

}

#[test]
fn test_sell_price() {

    let mut exchange = Market::new();
    let item = "CORN".to_string();

    let order1 = OrderRequest::new("BOB".to_string(), item.clone(), OrderKind::SELL, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), item.clone(),OrderKind::SELL, 12, 14.0);

    exchange.place_order(order1);
    exchange.place_order(order2);

    let best_order = exchange.get_best_selling_price(item).unwrap();

    let test_str = "Order { id: *, user_id: \"BOB\", kind: SELL, amount: 32, price_per: OrderedFloat(12.0) }";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", best_order).as_str()));

}

#[test]
fn test_buy_price() {

    let mut exchange = Market::new();
    let item = "CORN".to_string();

    let order1 = OrderRequest::new("BOB".to_string(), item.clone(), OrderKind::BUY, 32, 12.0);
    let order2 = OrderRequest::new("ALICE".to_string(), item.clone(),OrderKind::BUY, 12, 14.0);

    exchange.place_order(order1);
    exchange.place_order(order2);

    let best_order = exchange.get_best_buying_price(item).unwrap();

    let test_str = "Order { id: *, user_id: \"ALICE\", kind: BUY, amount: 12, price_per: OrderedFloat(14.0) }";

    assert!(WildMatch::new(test_str).matches(format!("{:?}", best_order).as_str()));

}

#[test]
fn speed_test() {

    let now = Instant::now();

    let mut exchange = Market::new();

    let names = vec!["ALICE", "BOB", "CLYDE", "DOOFUS", "EDGAR", "FRANK", "GOMEZ", 
    "HASAN", "ISKANDAR", "JOE", "KYLE", "LARRY", "MOE", "NIGEL", "OSACR", "PAUL", "QBERT", 
    "RON", "SEBASTIAN", "TOM", "ULANBATAAR", "VIKTOR", "WYOMING", "XANDER", "YOLANDE", "ZACHARY"];

    let items = vec!["APPLES", "BANANAS", "CORN", "DETERGENT", "EGGS", "FROGS", "GRUEL", 
    "HALO_3", "INCENSE", "JUUL", "KNIVES", "LAVA", "MYCELIUM", "NITROGEN", "OVALTINE", "POGS"];
    
    let num_trades = 300_000;
    println!("Now running {} trades...", num_trades);

    for _ in 0..num_trades {

        let user = names.choose(&mut rand::thread_rng()).unwrap().to_string();
        let item = items.choose(&mut rand::thread_rng()).unwrap().to_string();

        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0..1.0);

        let kind = if rand < 0.5 { OrderKind::BUY } else { OrderKind::SELL };

        let amount = rng.gen_range(1..1000);
        let price_per = rng.gen_range(1.0..25.0);

        let order = OrderRequest::new(user, item, kind, amount, price_per);
        exchange.place_order(order);

    }

    let elapsed = now.elapsed().as_millis();
    println!("Ran in {}ms", elapsed);
    println!("{} trades/sec", (num_trades / elapsed) * 1000);

}
