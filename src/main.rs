use std::{collections::HashMap, cmp::Ordering};

#[allow(non_snake_case)]

#[derive(Debug, PartialEq, Eq)]
enum OrderKind {
    BUY,
    SELL,
}

#[derive(Debug, PartialEq)]
struct Order {
    user_id: String,
    kind: OrderKind,
    amount: u32,
    price_per: f32
}

impl Order {
    fn new(user_id: String, kind: OrderKind, amount: u32, price_per: f32) -> Order {
        Order {
            user_id: user_id,
            kind: kind,
            amount: amount,
            price_per: price_per,
        }
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

struct Market {
    map: Box<HashMap<String, Vec<Order>>>,
    transactions: Box<Vec<Transaction>>,
}

#[derive(Debug, PartialEq)]
struct Transaction {
    buyer: String,
    seller: String,
    amount: u32,
    price_per: f32,
}

impl Transaction {
    fn new(buyer_id: String, seller_id: String, amount: u32, price_per: f32) -> Transaction {
        Transaction {
            buyer: buyer_id,
            seller: seller_id,
            amount: amount,
            price_per: price_per
        }
    }
}

impl Market {

    fn new() -> Market {
        Market {
            map: Box::new(HashMap::new()),
            transactions: Box::new(vec![]),
        }
    }

    // TODO: callback that uses the list of transactions
    pub fn place_order(&mut self, order_request: OrderRequest) {

        let item = order_request.item;
        let order = order_request.order;

        if !self.map.contains_key(&item) {
            self.map.insert(item, vec![order]);
        } else {

            let orders: &mut Vec<Order> = self.map.get_mut(&item).unwrap();
            orders.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let transactions: &mut Vec<Transaction> = self.transactions.as_mut();

            // transact
            match order.kind {
                OrderKind::BUY => buy(order, orders, transactions),
                OrderKind::SELL => sell(order, orders, transactions),
            };

            // loop over transactions and update customers
            println!("{:?}", self.transactions);

        }

        println!("{:?}", self.map);

    } 

}

fn buy(order: Order, orders: &mut Vec<Order>, transactions: &mut Vec<Transaction>){

    let temp_orders: &mut Vec<Order> = orders;
    let mut order = order;

    // low to high
    for i in 0..temp_orders.len() {
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

            }
        }
    }

    temp_orders.push(order);
    temp_orders.retain(|x| x.amount > 0);

}

fn sell(order: Order, orders: &mut Vec<Order>, transactions: &mut Vec<Transaction>){

    let temp_orders: &mut Vec<Order> = orders;
    let mut order = order;

    // high to low
    for i in (0..temp_orders.len()).rev() {
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
            }
        }
    }

    temp_orders.push(order);
    temp_orders.retain(|x| x.amount > 0);

}

fn main() {
    
    let mut market = Market::new();

    let user1 = "BOB_JONES".to_string();
    let user2 = "ALICE_SHLUMP".to_string();

    let order1 = OrderRequest::new(user1.clone(), "corn".to_string(), OrderKind::SELL, 100, 2.1);
    let order2 = OrderRequest::new(user1.clone(), "corn".to_string(), OrderKind::SELL, 50, 2.5);
    let order3 = OrderRequest::new(user2, "corn".to_string(), OrderKind::BUY, 115, 2.6);

    market.place_order(order1);
    market.place_order(order2);
    market.place_order(order3);
    
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn transact() {
//         let mut market = Market::new();

//         let order1 = OrderRequest::new("corn".to_string(), OrderKind::SELL, 100, 2.1);
//         let order2 = OrderRequest::new("corn".to_string(), OrderKind::SELL, 50, 2.5);
//         let order3 = OrderRequest::new("corn".to_string(), OrderKind::BUY, 115, 2.6);
    
//         market.place_order(order1);
//         market.place_order(order2);
//         market.place_order(order3);

//         assert_eq!(vec![Order::new(OrderKind::SELL, 35, 2.5)], *market.map.get("CORN").unwrap());
//     }

// }