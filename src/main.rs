use std::collections::HashMap;

#[allow(non_snake_case)]

#[derive(Debug, PartialEq)]
enum OrderKind {
    BUY,
    SELL,
}

#[derive(Debug, PartialEq)]
struct Order {
    kind: OrderKind,
    amount: u32,
    price_per: f32
}

impl Order {
    fn new(kind: OrderKind, amount: u32, price_per: f32) -> Order {
        Order {
            kind: kind,
            amount: amount,
            price_per: price_per,
        }
    }
}

struct OrderRequest {
    item: String,
    order: Order,
}

impl OrderRequest {
    fn new(item: String, kind: OrderKind, amount: u32, price_per: f32) -> OrderRequest {
        OrderRequest { 
            item: item.to_uppercase(), 
            order: Order::new(kind, amount, price_per), 
        }
    }
}

struct Market {
    map: Box<HashMap<String, Vec<Order>>>,
}

impl Market {
    fn new() -> Market {
        Market {
            map: Box::new(HashMap::new()),
        }
    }

    pub fn place_order(&mut self, order_request: OrderRequest) {

        let item = order_request.item;
        let order = order_request.order;

        if !self.map.contains_key(&item) {
            self.map.insert(item, vec![order]);
        } else {
            let orders: &mut Vec<Order> = self.map.get_mut(&item).unwrap();

            // transact
            match order.kind {
                OrderKind::BUY => buy(order, orders), // TODO: pass in transaction history
                OrderKind::SELL => sell(order, orders),
            };

        }

        println!("{:?}", self.map);

    } 

}

fn buy(order: Order, orders: &mut Vec<Order>){

    let temp_orders: &mut Vec<Order> = orders;
    let mut order = order;

    for i in 0..temp_orders.len() {
        if temp_orders[i].kind == OrderKind::SELL {
            if temp_orders[i].price_per <= order.price_per {
                if temp_orders[i].amount > order.amount {
                    temp_orders[i].amount -= order.amount;
                    order.amount = 0;
                } else if temp_orders[i].amount < order.amount {
                    order.amount -= temp_orders[i].amount;
                    temp_orders[i].amount = 0;
                } else {
                    temp_orders[i].amount = 0;
                    order.amount = 0;
                }
            }
        }
    }

    temp_orders.push(order);
    temp_orders.retain(|x| x.amount > 0);

}

fn sell(order: Order, orders: &mut Vec<Order>){

    let temp_orders: &mut Vec<Order> = orders;
    let mut order = order;

    for i in 0..temp_orders.len() {
        if temp_orders[i].kind == OrderKind::BUY {
            if temp_orders[i].price_per >= order.price_per {
                if temp_orders[i].amount > order.amount {
                    temp_orders[i].amount -= order.amount;
                    order.amount = 0;
                } else if temp_orders[i].amount < order.amount {
                    order.amount -= temp_orders[i].amount;
                    temp_orders[i].amount = 0;
                } else {
                    temp_orders[i].amount = 0;
                    order.amount = 0;
                }
            }
        }
    }

    temp_orders.push(order);
    temp_orders.retain(|x| x.amount > 0);

}

fn main() {
    
    let mut market = Market::new();

    let order1 = OrderRequest::new("corn".to_string(), OrderKind::SELL, 100, 2.1);
    let order2 = OrderRequest::new("corn".to_string(), OrderKind::SELL, 50, 2.5);
    let order3 = OrderRequest::new("corn".to_string(), OrderKind::BUY, 115, 2.6);

    market.place_order(order1);
    market.place_order(order2);
    market.place_order(order3);
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn transact() {
        let mut market = Market::new();

        let order1 = OrderRequest::new("corn".to_string(), OrderKind::SELL, 100, 2.1);
        let order2 = OrderRequest::new("corn".to_string(), OrderKind::SELL, 50, 2.5);
        let order3 = OrderRequest::new("corn".to_string(), OrderKind::BUY, 115, 2.6);
    
        market.place_order(order1);
        market.place_order(order2);
        market.place_order(order3);

        assert_eq!(vec![Order::new(OrderKind::SELL, 35, 2.5)], *market.map.get("CORN").unwrap());
    }

}