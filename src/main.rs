use market::Summary;
use rand::{seq::SliceRandom, Rng}; // 0.7.2
use std::time::Instant;

pub mod market;

use crate::market::{Market, History, OrderRequest, OrderKind};

struct Exchange {
    market: Market,
    // history: History
}

impl Exchange {
    fn new() -> Exchange {
        Exchange { 
            market: Market::new(), 
            // history: History::new()
        }
    }

    pub fn place_order(&mut self, order_request: OrderRequest) -> Summary {

        // Place an order on the market
        self.market.place_order(order_request)
        
    }

    // /// Get the most recent transaction
    // pub fn get_price(&mut self, item: String) -> Option<f32> {

    //     match self.history.map.get(&item) {
    //         Some(history) => {
    //             if history.len() < 1 { return None }
    //             return Some(history[history.len() - 1].price_per.0)
    //         },
    //         None => return None
    //     }

    // }

    // pub fn get_orders(&mut self, item: String) -> Ledger {

    //     if self.market.map.contains_key(&item)

    // }

}

fn main() {

    let now = Instant::now();

    let mut exchange = Exchange::new();
    
    let item = "corn".to_string();

    let names = vec!["ALICE", "BOB", "CLYDE", "DOOFUS", "EDGAR", "FRANK", "GOMEZ", 
    "HASAN", "ISKANDAR", "JOE", "KYLE", "LARRY", "MOE", "NIGEL", "OSACR", "PAUL", "QBERT", 
    "RON", "SEBASTIAN", "TOM", "ULANBATAAR", "VIKTOR", "WYOMING", "XANDER", "YOLANDE", "ZACHARY"];

    let items = vec!["APPLES", "BANANAS", "CORN", "DETERGENT", "EGGS", "FROGS", "GRUEL", 
    "HALO_3", "INCENSE", "JUUL", "KNIVES", "LAVA", "MYCELIUM", "NITROGEN", "OVALTINE", "POGS"];

    for _ in 0..300_000 {

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

    println!("{}", now.elapsed().as_millis());
    
}

#[cfg(test)]
mod tests {

    use super::*;
    use wildmatch::WildMatch;

    #[test]
    fn test_buy() {

        let mut exchange = Exchange::new();

        let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
        let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);
    
        exchange.place_order(order1);
        exchange.place_order(order2);

        let buy_orders = &exchange.market.map.get("CORN").unwrap().buy_orders;

        let test_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 32, price_per: OrderedFloat(12.0) }, Order { id: *, user_id: \"ALICE\", kind: BUY, amount: 12, price_per: OrderedFloat(14.0) }]";

        assert!(WildMatch::new(test_str).matches(format!("{:?}", buy_orders).as_str()));

    }

    #[test]
    fn test_sell() {

        let mut exchange = Exchange::new();

        let order1 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 20, 10.0);
        let order2 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 14, 15.0);
    
        exchange.place_order(order1);
        exchange.place_order(order2);

        let sell_orders = &exchange.market.map.get("CORN").unwrap().sell_orders;

        let test_str = "[Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 20, price_per: OrderedFloat(10.0) }, Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 14, price_per: OrderedFloat(15.0) }]";

        assert!(WildMatch::new(test_str).matches(format!("{:?}", sell_orders).as_str()));

    }

    #[test]
    fn test_buy_and_sell() {

        let mut exchange = Exchange::new();

        let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 32, 12.0);
        let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::BUY, 12, 14.0);
        let order3 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 20, 10.0);
        let order4 = OrderRequest::new("CAROL".to_string(), "CORN".to_string(),OrderKind::SELL, 14, 15.0);
    
        exchange.place_order(order1);
        exchange.place_order(order2);
        exchange.place_order(order3);
        exchange.place_order(order4);

        let ledger = exchange.market.map.get("CORN").unwrap();

        let buy_orders = &ledger.buy_orders;
        let sell_orders = &ledger.sell_orders;

        let buy_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 24, price_per: OrderedFloat(12.0) }]";
        let sell_str = "[Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 14, price_per: OrderedFloat(15.0) }]";

        assert!(WildMatch::new(buy_str).matches(format!("{:?}", buy_orders).as_str()));
        assert!(WildMatch::new(sell_str).matches(format!("{:?}", sell_orders).as_str()));

    }

    #[test]
    fn test_buy_and_sell_and_buy() {

        let mut exchange = Exchange::new();

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

        let ledger = exchange.market.map.get("CORN").unwrap();

        let buy_orders = &ledger.buy_orders;
        let sell_orders = &ledger.sell_orders;

        let buy_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 24, price_per: OrderedFloat(12.0) }]";
        let sell_str = "[]";

        assert!(WildMatch::new(buy_str).matches(format!("{:?}", buy_orders).as_str()));
        assert!(WildMatch::new(sell_str).matches(format!("{:?}", sell_orders).as_str()));

    }

    #[test]
    fn test_summary() {

        let mut exchange = Exchange::new();

        let order1 = OrderRequest::new("BOB".to_string(), "CORN".to_string(), OrderKind::BUY, 12, 14.0);
        let order2 = OrderRequest::new("ALICE".to_string(), "CORN".to_string(),OrderKind::SELL, 32, 12.0);
    
        exchange.place_order(order1);
        let summary = exchange.place_order(order2);

        println!("{:?}", summary);

        // assert_eq!(price, 12.0);

    }

}