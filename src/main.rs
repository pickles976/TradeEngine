use std::collections::HashMap;

#[allow(non_snake_case)]

#[derive(Debug)]
enum OrderKind {
    BUY,
    SELL,
}

#[derive(Debug)]
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

fn main() {
    
    let mut market: HashMap<String, Vec<Order>> = HashMap::new();

    let order = Order::new(OrderKind::BUY, 100, 2.0);
    let item = "corn".to_uppercase();

    if !market.contains_key(&item) {
        market.insert(item, vec![order]);
    } else {
        let orders = market.get_mut(&item).unwrap();
        orders.push(order);      
    }

    println!("{:?}", market);

}
