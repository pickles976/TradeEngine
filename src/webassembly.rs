use wasm_bindgen::prelude::*;
use std::collections::HashMap;

pub mod market;
pub mod structs;

use crate::market::{Market, Ledger, LedgerJSON};
use crate::structs::{OrderRequest, OrderKind, Order};


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

    pub fn query_ledger(&mut self, item: String) -> String {

        let result: Option<Ledger> = self.market.query_ledger(item);
        match result {
            Some(ledger_copy) => serde_json::to_string(&ledger_copy.to_json()).unwrap(),
            None => return "{}".to_string()
        }
    }

    pub fn dump(&mut self) -> String {
        let result: HashMap<String, LedgerJSON> = self.market.to_json();
        serde_json::to_string(&result).unwrap()
    }

    pub fn load(data: String) -> MarketWrapper{
        MarketWrapper {
            market: Market::from_json(data)
        }
    }

    pub fn cancel_order(&mut self, item: String, order: String) -> String {

        match Order::from_json_string(&order) {
            Some(order) => {
                let result: Option<Order> = self.market.cancel_order(item, order);
                match result {
                    Some(_order) => "{ \"status\": \"SUCCESS\" }".to_string(),
                    None => "{ \"status\": \"FAILURE\", \"reason\" : \"Order does not exist\" }".to_string()
                }
            },
            None => "{ \"status\": \"FAILURE\", \"reason\" : \"Invalid UUID string\" }".to_string()
        }
    }

    pub fn get_best_buying_price(&mut self, item: String) -> String {
        
        match self.market.get_best_buying_price(item) {
            Some(order) => serde_json::to_string(&order.to_json()).unwrap(),
            None => "{}".to_string()
        }

    }

    pub fn get_best_selling_price(&mut self, item:String) -> String {
        match self.market.get_best_selling_price(item) {
            Some(order) => serde_json::to_string(&order.to_json()).unwrap(),
            None => "{}".to_string()
        }
    }

}

#[wasm_bindgen]
pub fn test() -> String {
    "Module works".to_string()
}
