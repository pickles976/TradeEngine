use std::{collections::HashMap, cmp::Ordering, time::SystemTime};
use rand::{seq::SliceRandom, Rng}; // 0.7.2
use std::time::Instant;
use uuid::Uuid;
use ordered_float::OrderedFloat; // 1.0.2

#[allow(non_snake_case)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OrderKind {
    BUY,
    SELL,
}

#[derive(Debug, Clone)]
struct Order {
    id: Uuid,
    user_id: String,
    kind: OrderKind,
    amount: u32,
    price_per: OrderedFloat<f32>
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

struct OrderRequest {
    item: String,
    order: Order,
}

impl OrderRequest {
    fn new(user_id: String, item: String, kind: OrderKind, amount: u32, price_per: f32) -> OrderRequest {
        OrderRequest { 
            item: item.to_uppercase(), 
            order: Order::new(user_id, kind, amount, price_per), 
        }
    }
}

#[derive(Debug, PartialEq)]
struct Transaction {
    buyer: String,
    seller: String,
    amount: u32,
    price_per: OrderedFloat<f32>,
    time: SystemTime,
}

impl Transaction {
    fn new(buyer_id: String, seller_id: String, amount: u32, price_per: OrderedFloat<f32>) -> Transaction {
        Transaction {
            buyer: buyer_id,
            seller: seller_id,
            amount: amount,
            price_per: price_per,
            time: SystemTime::now()
        }
    }
}

struct Market {
    map: HashMap<String, Vec<Order>>,
}

impl Market {

    fn new() -> Market {
        Market {
            map: HashMap::new()
        }
    }

    pub fn place_order(&mut self, order_request: OrderRequest) ->  Vec<Transaction> {

        let item = order_request.item;
        let order = order_request.order;
        let mut transactions: Vec<Transaction> = Vec::new();

        if !self.map.contains_key(&item) {
            self.map.insert(item, vec![order]);
        } else {

            let orders: &mut Vec<Order> = self.map.get_mut(&item).unwrap();

            // transact
            match order.kind {
                OrderKind::BUY => buy(order, orders, &mut transactions),
                OrderKind::SELL => sell(order, orders, &mut transactions),
            };
        }

        transactions

    } 

}

struct History {
    map: HashMap<String, Vec<Transaction>>,
}

impl History {
    fn new() -> History {
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

fn buy(order: Order, orders: &mut Vec<Order>, transactions: &mut Vec<Transaction>){

    let temp_orders: &mut Vec<Order> = orders;
    let mut order = order;

    let end = temp_orders.binary_search(&order).unwrap_or_else(|e| e);

    // low to high
    for i in 0..end {

        if order.amount < 1 {break;}

        if temp_orders[i].kind == OrderKind::SELL {
            if temp_orders[i].price_per <= order.price_per {
                let mut amount = 0;
                if temp_orders[i].amount > order.amount {
                    amount = order.amount;
                    temp_orders[i].amount -= order.amount;
                    order.amount = 0;
                } else if temp_orders[i].amount < order.amount {
                    amount = temp_orders[i].amount;
                    order.amount -= temp_orders[i].amount;
                    temp_orders[i].amount = 0;
                } else {
                    amount = order.amount;
                    temp_orders[i].amount = 0;
                    order.amount = 0;
                }

                let transaction = Transaction::new(order.user_id.clone(), temp_orders[i].user_id.clone(), amount, temp_orders[i].price_per);
                transactions.push(transaction);

            } else {
                break;
            }
        }
    }

    let pos = temp_orders.binary_search(&order).unwrap_or_else(|e| e);
    temp_orders.insert(pos, order);

    // temp_orders.push(order);
    temp_orders.retain(|x| x.amount > 0);

}

fn sell(order: Order, orders: &mut Vec<Order>, transactions: &mut Vec<Transaction>){

    let temp_orders: &mut Vec<Order> = orders;
    let mut order = order;

    let end = temp_orders.binary_search(&order).unwrap_or_else(|e| e);

    // high to low
    for i in (end..temp_orders.len()).rev() {

        if order.amount < 1 {break;}

        if temp_orders[i].kind == OrderKind::BUY {
            if temp_orders[i].price_per >= order.price_per {
                let mut amount = 0;
                if temp_orders[i].amount > order.amount {
                    amount = order.amount;
                    temp_orders[i].amount -= order.amount;
                    order.amount = 0;
                } else if temp_orders[i].amount < order.amount {
                    amount = temp_orders[i].amount;
                    order.amount -= temp_orders[i].amount;
                    temp_orders[i].amount = 0;
                } else {
                    amount = order.amount;
                    temp_orders[i].amount = 0;
                    order.amount = 0;
                }

                let transaction = Transaction::new(order.user_id.clone(), temp_orders[i].user_id.clone(), amount, temp_orders[i].price_per);
                transactions.push(transaction);
            } else {
                break;
            }
        }
    }

    let pos = temp_orders.binary_search(&order).unwrap_or_else(|e| e);
    temp_orders.insert(pos, order);

    temp_orders.retain(|x| x.amount > 0);

}

struct Exchange {
    market: Market,
    history: History
}

impl Exchange {
    fn new() -> Exchange {
        Exchange { 
            market: Market::new(), 
            history: History::new()
        }
    }

    pub fn place_order(&mut self, order_request: OrderRequest) {
        let item = order_request.item.to_string();

        // Place an order on the market
        let transactions = self.market.place_order(order_request);

        // Update the order history with the transactions
        self.history.add_transactions(&item, transactions);

        // TODO: return the transations and do something
    }
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
        let mut rand = rng.gen_range(0.0..1.0);

        let kind = if rand < 0.5 { OrderKind::BUY } else { OrderKind::SELL };

        let amount = rng.gen_range(1..1000);
        let price_per = rng.gen_range(1.0..25.0);

        let order = OrderRequest::new(user, item, kind, amount, price_per);
        exchange.place_order(order);

    }

    // println!("{:?}", exchange.market.map);

    println!("{}", now.elapsed().as_millis());
    
}

#[cfg(test)]
mod tests {

    use super::*;
    use wildmatch::WildMatch;

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


        println!("{:?}", exchange.market.map.get("CORN").unwrap());

        let btree = exchange.market.map.get("CORN").unwrap();

        let test_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 24, price_per: OrderedFloat(12.0) }, Order { id: *, user_id: \"CAROL\", kind: SELL, amount: 14, price_per: OrderedFloat(15.0) }]";

        assert!(WildMatch::new(test_str).matches(format!("{:?}", btree).as_str()));

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


        println!("{:?}", exchange.market.map.get("CORN").unwrap());

        let btree = exchange.market.map.get("CORN").unwrap();

        let test_str = "[Order { id: *, user_id: \"BOB\", kind: BUY, amount: 24, price_per: OrderedFloat(12.0) }]";

        assert!(WildMatch::new(test_str).matches(format!("{:?}", btree).as_str()));

    }
}