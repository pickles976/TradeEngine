use std::time::Instant;
use rand::{seq::SliceRandom, Rng}; // 0.7.2

use MarketCore::{self, structs::{OrderRequest, OrderKind}, MarketWrapper};
use wildmatch::WildMatch;

#[test]
fn test_buy() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    let summary = exchange.buy(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[],\"to_update\":[],\"created\":{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0}}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));

}

#[test]
fn test_sell() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[],\"to_update\":[],\"created\":{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6.0}}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));

}

#[test]
fn test_buy_and_sell() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[{\"buyer\":\"YOLANDE\",\"seller\":\"YOLANDE\",\"amount\":347,\"price_per\":6.0}],\"to_update\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":0,\"price_per\":6.0}],\"created\":null}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));
}

#[test]
fn test_buy_buy_sell() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[{\"buyer\":\"YOLANDE\",\"seller\":\"YOLANDE\",\"amount\":347,\"price_per\":6.0}],\"to_update\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":0,\"price_per\":6.0}],\"created\":null}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));
}

#[test]
fn test_query_ledger() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let query_str = "NITROGEN".to_string();
    let response = exchange.query_ledger(query_str);

    println!("{}", response);

    let test_str = "{\"buy_orders\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0}],\"sell_orders\":[]}";

    assert!(WildMatch::new(test_str).matches(response.as_str()));

}

#[test]
fn test_query_ledger_empty() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let query_str = "CHEESE".to_string();
    let response = exchange.query_ledger(query_str);

    println!("{}", response);

    let test_str = "{}";

    assert!(WildMatch::new(test_str).matches(response.as_str()));

}

#[test]
fn test_cancel_order_fail_no_key() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let order_string = "{\"id\": \"fd45fe94-883f-498e-b32f-6bbb4bbe8d81\", \"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6}".to_string();

    let cancellation_status = exchange.cancel_order("NITROGEN".to_string(), order_string);

    println!("{}", cancellation_status);

    let test_str = "{ 'status': 'FAILURE', 'reason' : 'Order does not exist' }";

    assert!(WildMatch::new(test_str).matches(cancellation_status.as_str()));

}

#[test]
fn test_cancel_order_fail_bad_uuid() {

    let order_request_str = "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let order_string = "{\"id\": \"bad_uuid_string\", \"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6}".to_string();

    let cancellation_status = exchange.cancel_order("NITROGEN".to_string(), order_string);

    println!("{}", cancellation_status);

    let test_str = "{ 'status': 'FAILURE', 'reason' : 'Invalid UUID string' }";

    assert!(WildMatch::new(test_str).matches(cancellation_status.as_str()));

}

// #[test]
// fn test_get_best_buy_price() {

// }

// #[test]
// fn test_get_best_sell_price() {

// }