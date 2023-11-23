use rand::{seq::SliceRandom, Rng};
use std::time::Instant; // 0.7.2

use wildmatch::WildMatch;
use MarketCore::{
    self,
    structs::{OrderKind, OrderRequest},
    MarketWrapper,
};

#[test]
fn test_buy() {
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    let summary = exchange.buy(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[],\"to_update\":[],\"created\":{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0}}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));
}

#[test]
fn test_sell() {
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[],\"to_update\":[],\"created\":{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6.0}}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));
}

#[test]
fn test_buy_and_sell() {
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[{\"buyer\":\"YOLANDE\",\"seller\":\"YOLANDE\",\"amount\":347,\"price_per\":6.0}],\"to_update\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":0,\"price_per\":6.0}],\"created\":null}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));
}

#[test]
fn test_buy_buy_sell() {
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

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
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

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
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

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
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let order_string = "{\"id\": \"fd45fe94-883f-498e-b32f-6bbb4bbe8d81\", \"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6}".to_string();

    let cancellation_status = exchange.cancel_order("NITROGEN".to_string(), order_string);

    println!("{}", cancellation_status);

    let test_str = "{ \"status\": \"FAILURE\", \"reason\" : \"Order does not exist\" }";

    assert!(WildMatch::new(test_str).matches(cancellation_status.as_str()));
}

#[test]
fn test_cancel_order_fail_bad_uuid() {
    let order_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str);
    exchange.buy(&order_request_str);
    let summary = exchange.sell(&order_request_str);

    println!("{}", summary);

    let order_string = "{\"id\": \"bad_uuid_string\", \"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6}".to_string();

    let cancellation_status = exchange.cancel_order("NITROGEN".to_string(), order_string);

    println!("{}", cancellation_status);

    let test_str = "{ \"status\": \"FAILURE\", \"reason\" : \"Invalid UUID string\" }";

    assert!(WildMatch::new(test_str).matches(cancellation_status.as_str()));
}

#[test]
fn test_get_best_buy_price() {
    let order_request_str_1 =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";
    let order_request_str_2 =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":8}";

    let mut exchange = MarketWrapper::new();

    exchange.buy(&order_request_str_1);
    exchange.buy(&order_request_str_2);

    println!("{}", exchange.query_ledger("NITROGEN".to_string()));

    let test_str =
        "{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":8.0}";

    println!("{}", test_str);

    let best_order = exchange.get_best_buying_price("NITROGEN".to_string());

    println!("{}", best_order);

    assert!(WildMatch::new(test_str).matches(best_order.as_str()));
}

#[test]
fn test_get_best_sell_price() {
    let order_request_str_1 =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";
    let order_request_str_2 =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":8}";

    let mut exchange = MarketWrapper::new();

    exchange.sell(&order_request_str_1);
    exchange.sell(&order_request_str_2);

    println!("{}", exchange.query_ledger("NITROGEN".to_string()));

    let test_str =
        "{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6.0}";

    println!("{}", test_str);

    let best_order = exchange.get_best_selling_price("NITROGEN".to_string());

    println!("{}", best_order);

    assert!(WildMatch::new(test_str).matches(best_order.as_str()));
}

#[test]
fn test_dump_market() {
    // Test Buy
    let buy_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"NITROGEN\",\"amount\":347,\"price_per\":6}";

    let mut exchange = MarketWrapper::new();
    exchange.buy(&buy_request_str);
    exchange.buy(&buy_request_str);

    let response = exchange.dump();
    let test_str: &str = "{\"NITROGEN\":{\"buy_orders\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0},{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0}],\"sell_orders\":[]}}";
    assert!(WildMatch::new(test_str).matches(response.as_str()));

    // Test Sell
    let mut exchange = MarketWrapper::new();
    let sell_request_str =
        "{\"user_id\":\"YOLANDE\",\"item\":\"WEED\",\"amount\":347,\"price_per\":6}";
    exchange.sell(&sell_request_str);

    let response = exchange.dump();
    let test_str: &str = "{\"WEED\":{\"buy_orders\":[],\"sell_orders\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6.0}]}}";
    assert!(WildMatch::new(test_str).matches(response.as_str()));
}

#[test]
fn test_load_market() {
    let market_string: &str = "{\"NITROGEN\":{\"buy_orders\":[{\"id\":\"38e7b46b-ae36-43f9-aa14-cf776625b58c\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0},{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0}],\"sell_orders\":[]},\"WEED\":{\"buy_orders\":[],\"sell_orders\":[{\"id\":\"38e7b46b-ae36-43f9-aa14-cf776625b58c\",\"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6.0}]}}";
    let mut exchange = MarketWrapper::load(market_string.to_string());

    let query_str = "NITROGEN".to_string();
    let response = exchange.query_ledger(query_str);
    let test_str = "{\"buy_orders\":[{\"id\":\"38e7b46b-ae36-43f9-aa14-cf776625b58c\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":347,\"price_per\":6.0}],\"sell_orders\":[]}";
    assert!(WildMatch::new(test_str).matches(response.as_str()));

    let query_str = "WEED".to_string();
    let response = exchange.query_ledger(query_str);
    let test_str = "{\"buy_orders\":[],\"sell_orders\":[{\"id\":\"38e7b46b-ae36-43f9-aa14-cf776625b58c\",\"user_id\":\"YOLANDE\",\"kind\":\"SELL\",\"amount\":347,\"price_per\":6.0}]}";
    assert!(WildMatch::new(test_str).matches(response.as_str()));
}
