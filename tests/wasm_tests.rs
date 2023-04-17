use std::time::Instant;
use rand::{seq::SliceRandom, Rng}; // 0.7.2

use CRABSHAQ::{self, structs::{OrderRequest, OrderKind}, MarketWrapper};
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

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[{\"buyer\":\"YOLANDE\",\"seller\":\"YOLANDE\",\"amount\":347,\"price_per\":6.0,\"time\":{\"secs_since_epoch\":*,\"nanos_since_epoch\":*}}],\"to_update\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":0,\"price_per\":6.0}],\"created\":null}";

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

    let test_str = "{\"key\":\"NITROGEN\",\"transactions\":[{\"buyer\":\"YOLANDE\",\"seller\":\"YOLANDE\",\"amount\":347,\"price_per\":6.0,\"time\":{\"secs_since_epoch\":*,\"nanos_since_epoch\":*}}],\"to_update\":[{\"id\":\"*\",\"user_id\":\"YOLANDE\",\"kind\":\"BUY\",\"amount\":0,\"price_per\":6.0}],\"created\":null}";

    assert!(WildMatch::new(test_str).matches(summary.as_str()));
}
