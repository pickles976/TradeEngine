use std::cmp::Ordering;
use uuid::Uuid;
use ordered_float::OrderedFloat; // 1.0.2

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;


#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum OrderKind {
    BUY,
    SELL,
    MARKET_BUY,
    MARKET_SELL,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: Uuid,
    pub user_id: String,
    pub kind: OrderKind,
    pub amount: u32,
    pub price_per: OrderedFloat<f32>
}

impl Order {
    fn new(user_id: String, kind: OrderKind, amount: u32, price_per: f32) -> Order {
        Order {
            id: Uuid::new_v4(), 
            user_id: user_id,
            kind: kind,
            amount: amount,
            price_per: OrderedFloat(price_per),
        }
    }

    pub fn to_json(&self) -> OrderJSON {
        OrderJSON { 
            id: self.id.to_string(),
            user_id: self.user_id.clone(), 
            kind: self.kind, 
            amount: self.amount, 
            price_per: self.price_per.0 
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderJSON {
    pub id: String,
    pub user_id: String,
    pub kind: OrderKind,
    pub amount: u32,
    pub price_per: f32
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Order {}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price_per.cmp(&other.price_per)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price_per.partial_cmp(&other.price_per)
    }
}

pub struct OrderRequest {
    pub item: String,
    pub order: Order,
}

impl OrderRequest {
    pub fn new(user_id: String, item: String, kind: OrderKind, amount: u32, price_per: f32) -> OrderRequest {
        OrderRequest { 
            item: item.to_uppercase(), 
            order: Order::new(user_id, kind, amount, price_per), 
        }
    }

    pub fn from_json_string(json_str: &str) -> OrderRequest {
        let data: OrderRequestJSON = serde_json::from_str(&json_str).unwrap_throw();
        OrderRequest::new(data.user_id, data.item, OrderKind::BUY, data.amount, data.price_per)
    }
 }

#[derive(Serialize, Deserialize)]
struct OrderRequestJSON {
    pub user_id: String, 
    pub item: String, 
    pub amount: u32, 
    pub price_per: f32
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub buyer: String,
    pub seller: String,
    pub amount: u32,
    pub price_per: f32,
}

impl Transaction {
    pub fn new(buyer_id: String, seller_id: String, amount: u32, price_per: OrderedFloat<f32>) -> Transaction {
        Transaction {
            buyer: buyer_id,
            seller: seller_id,
            amount: amount,
            price_per: price_per.0,
        }
    }
}

/// Summary of what occured
#[derive(Debug, PartialEq)]
pub struct Summary {
    pub key: String,
    pub transactions: Vec<Transaction>,
    pub to_update: Vec<Order>,
    pub created: Option<Order>
}

impl Summary {
    pub fn new(key: String) -> Summary {
        Summary {
            key: key,
            transactions: vec![],
            to_update: vec![],
            created: None
        }
    }

    pub fn to_json_str(self) -> String {

        let summary_json = SummaryJSON {
            key: self.key,
            transactions: self.transactions,
            to_update: self.to_update.iter().map(|x| { x.to_json() }).collect(),
            created: match self.created {
                Some(order) => Some(order.to_json()),
                None => None,
            },
        };

        serde_json::to_string(&summary_json).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SummaryJSON {
    pub key: String,
    pub transactions: Vec<Transaction>,
    pub to_update: Vec<OrderJSON>,
    pub created: Option<OrderJSON>
}
