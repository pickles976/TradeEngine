use std::{collections::HashMap, cmp::Ordering, time::SystemTime};
use rand::{seq::SliceRandom, Rng}; // 0.7.2
use std::time::Instant;
use uuid::Uuid;
use ordered_float::OrderedFloat; // 1.0.2

#[allow(non_snake_case)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderKind {
    BUY,
    SELL,
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

#[derive(Debug, PartialEq)]
pub struct Ledger {
    pub buy_orders: Vec<Order>,
    pub sell_orders: Vec<Order>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            buy_orders: vec![],
            sell_orders: vec![]
        }
    }
}

pub struct Market {
    pub map: HashMap<String, Ledger>,
}

impl Market {

    pub fn new() -> Market {
        Market {
            map: HashMap::new()
        }
    }

    pub fn place_order(&mut self, order_request: OrderRequest) ->  Vec<Transaction> {

        let item = order_request.item;
        let order = order_request.order;
        let mut transactions: Vec<Transaction> = Vec::new();

        if !self.map.contains_key(&item) { // insert into ledger
            let mut ledger = Ledger::new();
            match order.kind {
                OrderKind::BUY => ledger.buy_orders.push(order),
                OrderKind::SELL => ledger.sell_orders.push(order),
            }
            self.map.insert(item, ledger);
        } else { // update ledger

            let ledger = &mut self.map.get_mut(&item).unwrap();
            let buy_orders: &mut Vec<Order> = &mut ledger.buy_orders;
            let sell_orders: &mut Vec<Order> = &mut ledger.sell_orders;

            // transact
            match order.kind {
                OrderKind::BUY => buy(order, buy_orders, sell_orders, &mut transactions),
                OrderKind::SELL => sell(order, buy_orders, sell_orders, &mut transactions),
            };
        }

        transactions

    } 

}

pub struct History {
    pub map: HashMap<String, Vec<Transaction>>,
}

impl History {
    pub fn new() -> History {
        History {
            map: HashMap::new()
        }
    }

    pub fn add_transactions(&mut self, item: &String, transactions: Vec<Transaction>) {

        if !self.map.contains_key(item) {
            self.map.insert(item.to_owned(), transactions);
            return;
        }

        let hist: &mut Vec<Transaction> = self.map.get_mut(item).unwrap();
        hist.extend(transactions);
        
    }
}

fn buy(order: Order, buy_orders: &mut Vec<Order>, sell_orders: &mut Vec<Order>, transactions: &mut Vec<Transaction>) {

    let mut order = order;
    let end = sell_orders.binary_search(&order).unwrap_or_else(|e| e);

    // low to high
    for i in 0..end {

        if order.amount < 1 {break;}

        if sell_orders[i].price_per <= order.price_per {
            let amount;
            if sell_orders[i].amount > order.amount {
                amount = order.amount;
                sell_orders[i].amount -= order.amount;
                order.amount = 0;
            } else if sell_orders[i].amount < order.amount {
                amount = sell_orders[i].amount;
                order.amount -= sell_orders[i].amount;
                sell_orders[i].amount = 0;
            } else {
                amount = order.amount;
                sell_orders[i].amount = 0;
                order.amount = 0;
            }

            let transaction = Transaction::new(order.user_id.clone(), sell_orders[i].user_id.clone(), amount, sell_orders[i].price_per);
            transactions.push(transaction);

        } else {
            break;
        }
    }

    sell_orders.retain(|x| x.amount > 0);

    if order.amount < 1 { return }

    // Add our buy order to the buy ledger
    let pos = buy_orders.binary_search(&order).unwrap_or_else(|e| e);
    buy_orders.insert(pos, order);


}

fn sell(order: Order, buy_orders: &mut Vec<Order>, sell_orders: &mut Vec<Order>, transactions: &mut Vec<Transaction>) {

    let mut order = order;
    let end = buy_orders.binary_search(&order).unwrap_or_else(|e| e);

    // high to low
    for i in (end..buy_orders.len()).rev() {

        if order.amount < 1 {break;}

        if buy_orders[i].price_per >= order.price_per {
            let amount;
            if buy_orders[i].amount > order.amount {
                amount = order.amount;
                buy_orders[i].amount -= order.amount;
                order.amount = 0;
            } else if buy_orders[i].amount < order.amount {
                amount = buy_orders[i].amount;
                order.amount -= buy_orders[i].amount;
                buy_orders[i].amount = 0;
            } else {
                amount = order.amount;
                buy_orders[i].amount = 0;
                order.amount = 0;
            }

            let transaction = Transaction::new(order.user_id.clone(), buy_orders[i].user_id.clone(), amount, order.price_per);
            transactions.push(transaction);
            
        } else {
            break;
        }

    }

    buy_orders.retain(|x| x.amount > 0);

    if order.amount < 1 { return }

    // Add our buy order to the buy ledger
    let pos = sell_orders.binary_search(&order).unwrap_or_else(|e| e);
    sell_orders.insert(pos, order);

}