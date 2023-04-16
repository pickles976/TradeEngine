use std::{cmp::Ordering, time::SystemTime};
use uuid::Uuid;
use ordered_float::OrderedFloat; // 1.0.2

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub buyer: String,
    pub seller: String,
    pub amount: u32,
    pub price_per: OrderedFloat<f32>,
    pub time: SystemTime,
}

impl Transaction {
    pub fn new(buyer_id: String, seller_id: String, amount: u32, price_per: OrderedFloat<f32>) -> Transaction {
        Transaction {
            buyer: buyer_id,
            seller: seller_id,
            amount: amount,
            price_per: price_per,
            time: SystemTime::now()
        }
    }
}