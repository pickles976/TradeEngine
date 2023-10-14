use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

pub mod market;
pub mod structs;

use crate::market::{Market, Ledger};
use crate::structs::{OrderRequest, OrderKind};


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub struct MarketWrapper {
    market: Market,
}

#[wasm_bindgen]
impl MarketWrapper {

    pub fn new() -> MarketWrapper {
        MarketWrapper { 
            market: Market::new()
        }
    }

    pub fn test(&mut self) -> String {
        "Constructor works".to_string()
    }

    pub fn test_serialization(&mut self, order_request_str: &str) -> String {
        order_request_str.to_string()
    }

    pub fn buy(&mut self, order_request_str: &str) -> String {
        let mut order_request: OrderRequest = OrderRequest::from_json_string(order_request_str);
        order_request.order.kind = OrderKind::BUY;
        self.market.place_order(order_request).to_json_str()
    }

    pub fn sell(&mut self, order_request_str: &str) -> String {
        let mut order_request: OrderRequest = OrderRequest::from_json_string(order_request_str);
        order_request.order.kind = OrderKind::SELL;
        self.market.place_order(order_request).to_json_str()
    }

    pub fn query_ledger(&mut self, item: &str) -> String {

        // TODO: add an unwrap here
        let mut ledger_copy: Ledger = self.market.query_ledger(item);
        serde_json::to_string(&ledger_copy.to_json()).unwrap()
    }

    // pub fn cancel_order(&mut self, item: &str, order: &str) -> String {
    //     // pass
    // }

    // pub fn get_best_buying_price(&mut self, item: &str) -> String {
    //     // pass
    // }

    // pub fn get_best_selling_price(&mut self, item: &str) -> String {
    //     // pass
    // }

}

#[wasm_bindgen]
pub fn test() -> String {
    "Module works".to_string()
}
