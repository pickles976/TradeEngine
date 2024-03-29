use crate::structs::{Order, OrderJSON, OrderKind, OrderRequest, Summary, Transaction};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct Ledger {
    pub buy_orders: Vec<Order>,
    pub sell_orders: Vec<Order>,
}

#[derive(Serialize, Deserialize)]
pub struct LedgerJSON {
    pub buy_orders: Vec<OrderJSON>,
    pub sell_orders: Vec<OrderJSON>,
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger {
            buy_orders: vec![],
            sell_orders: vec![],
        }
    }

    pub fn to_json(&self) -> LedgerJSON {
        LedgerJSON {
            buy_orders: self.buy_orders.iter().map(|x| x.to_json()).collect(),
            sell_orders: self.sell_orders.iter().map(|x| x.to_json()).collect(),
        }
    }

    pub fn from_json(ledger_json: LedgerJSON) -> Ledger {
        Ledger {
            buy_orders: ledger_json
                .buy_orders
                .into_iter()
                .filter_map(|x| Order::from_json(x))
                .collect(),
            sell_orders: ledger_json
                .sell_orders
                .into_iter()
                .filter_map(|x| Order::from_json(x))
                .collect(),
        }
    }
}

pub struct Market {
    pub map: HashMap<String, Ledger>,
}

impl Market {
    pub fn new() -> Market {
        Market {
            map: HashMap::new(),
        }
    }

    pub fn place_order(&mut self, order_request: OrderRequest) -> Summary {
        let item = order_request.item;
        let order = order_request.order;

        let mut summary: Summary = Summary::new(item.clone());

        if !self.map.contains_key(&item) {
            // insert into ledger
            let mut ledger = Ledger::new();
            match order.kind {
                OrderKind::BUY => ledger.buy_orders.push(order.clone()),
                OrderKind::SELL => ledger.sell_orders.push(order.clone()),
                _ => { /* Do nothing */ }
            }
            self.map.insert(item, ledger);
            summary.created = Some(order);
        } else {
            // update ledger

            let ledger = &mut self.map.get_mut(&item).unwrap();

            // transact
            match order.kind {
                OrderKind::BUY => buy(order, ledger, &mut summary),
                OrderKind::SELL => sell(order, ledger, &mut summary),
                OrderKind::MARKET_BUY => market_buy(order, ledger, &mut summary),
                OrderKind::MARKET_SELL => market_sell(order, ledger, &mut summary),
            };
        }

        summary
    }

    pub fn cancel_order(&mut self, item: String, order: Order) -> Option<Order> {
        // Create an empty ptr
        let mut orders: Option<&mut Vec<Order>> = None;

        match self.map.get_mut(&item.to_uppercase()) {
            Some(ledger) => {
                match order.kind {
                    // Move ledger to ptr
                    OrderKind::BUY => orders = Some(&mut ledger.buy_orders),
                    OrderKind::SELL => orders = Some(&mut ledger.sell_orders),
                    _ => { /* Do nothing */ }
                };
            }
            None => { /* Do nothing */ }
        }

        // Modify ledger via ptr
        if let Some(order_list) = orders {
            for i in 0..order_list.len() {
                if order_list[i].id == order.id {
                    return Some(order_list.remove(i));
                }
            }
        }

        None
    }

    pub fn query_ledger(&mut self, item: String) -> Option<Ledger> {
        let result = self.map.get(&item.to_uppercase());

        match result {
            Some(ledger) => return Some(ledger.clone()),
            None => return None,
        }
    }

    pub fn get_best_buying_price(&mut self, item: String) -> Option<&Order> {
        let result = self.map.get(&item.to_uppercase());

        match result {
            Some(ledger) => return ledger.buy_orders.last(),
            None => return None,
        }
    }

    pub fn get_best_selling_price(&mut self, item: String) -> Option<&Order> {
        let result = self.map.get(&item.to_uppercase());

        match result {
            Some(ledger) => return ledger.sell_orders.first(),
            None => return None,
        }
    }

    pub fn to_json(&self) -> HashMap<String, LedgerJSON> {
        let mut market_copy = HashMap::new();

        for (key, value) in self.map.clone().into_iter() {
            market_copy.insert(key, value.clone().to_json());
        }

        market_copy
    }

    pub fn from_json(data: HashMap<String, LedgerJSON>) -> Market {
        let mut map = HashMap::new();

        for (key, value) in data.into_iter() {
            map.insert(key, Ledger::from_json(value));
        }

        Market {
            map: map,
        }
    }
}

fn buy(order: Order, ledger: &mut Ledger, summary: &mut Summary) {
    let buy_orders: &mut Vec<Order> = &mut ledger.buy_orders;
    let sell_orders: &mut Vec<Order> = &mut ledger.sell_orders;

    let mut order = order;
    let end = sell_orders.binary_search(&order).unwrap_or_else(|e| e);

    let mut to_remove = vec![];

    // low to high
    for i in 0..end {
        if order.amount < 1 {
            break;
        }

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

            if sell_orders[i].amount < 1 {
                to_remove.push(i);
            }

            // Buyer sets transaction price
            let transaction = Transaction::new(
                order.user_id.clone(),
                sell_orders[i].user_id.clone(),
                amount,
                order.price_per,
            );
            summary.transactions.push(transaction);
            summary.to_update.push(sell_orders[i].clone());
        } else {
            break;
        }
    }

    to_remove.sort();
    to_remove.reverse();
    for i in 0..to_remove.len() {
        sell_orders.remove(to_remove[i]);
    }

    if order.amount < 1 {
        return;
    }

    // Add our buy order to the buy ledger
    let pos = buy_orders.binary_search(&order).unwrap_or_else(|e| e);
    buy_orders.insert(pos, order.clone());
    summary.created = Some(order);
}

fn sell(order: Order, ledger: &mut Ledger, summary: &mut Summary) {
    let buy_orders: &mut Vec<Order> = &mut ledger.buy_orders;
    let sell_orders: &mut Vec<Order> = &mut ledger.sell_orders;

    let mut order = order;
    let end = buy_orders.binary_search(&order).unwrap_or_else(|e| e);

    //
    let mut to_remove = vec![];

    // high to low
    for i in (end..buy_orders.len()).rev() {
        if order.amount < 1 {
            break;
        }

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

            if buy_orders[i].amount < 1 {
                to_remove.push(i);
            }

            // Buyer sets price
            let transaction = Transaction::new(
                order.user_id.clone(),
                buy_orders[i].user_id.clone(),
                amount,
                buy_orders[i].price_per,
            );
            summary.transactions.push(transaction);
            summary.to_update.push(buy_orders[i].clone());
        } else {
            break;
        }
    }

    to_remove.sort();
    to_remove.reverse();
    for i in 0..to_remove.len() {
        buy_orders.remove(to_remove[i]);
    }

    if order.amount < 1 {
        return;
    }

    // Add our buy order to the buy ledger
    let pos = sell_orders.binary_search(&order).unwrap_or_else(|e| e);
    sell_orders.insert(pos, order.clone());
    summary.created = Some(order);
}

fn market_sell(order: Order, ledger: &mut Ledger, summary: &mut Summary) {
    let buy_orders: &mut Vec<Order> = &mut ledger.buy_orders;

    let mut order = order;

    //
    let mut to_remove = vec![];

    // high to low
    for i in (0..buy_orders.len()).rev() {
        if order.amount < 1 {
            break;
        }

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

        if buy_orders[i].amount < 1 {
            to_remove.push(i);
        }

        let transaction = Transaction::new(
            order.user_id.clone(),
            buy_orders[i].user_id.clone(),
            amount,
            order.price_per,
        );
        summary.transactions.push(transaction);
        summary.to_update.push(buy_orders[i].clone());
    }

    to_remove.sort();
    to_remove.reverse();
    for i in 0..to_remove.len() {
        buy_orders.remove(to_remove[i]);
    }
}

fn market_buy(order: Order, ledger: &mut Ledger, summary: &mut Summary) {
    let sell_orders: &mut Vec<Order> = &mut ledger.sell_orders;

    let mut order = order;

    let mut to_remove = vec![];

    // low to high
    for i in 0..sell_orders.len() {
        if order.amount < 1 {
            break;
        }

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

        if sell_orders[i].amount < 1 {
            to_remove.push(i);
        }

        let transaction = Transaction::new(
            order.user_id.clone(),
            sell_orders[i].user_id.clone(),
            amount,
            sell_orders[i].price_per,
        );
        summary.transactions.push(transaction);
        summary.to_update.push(sell_orders[i].clone());
    }

    to_remove.sort();
    to_remove.reverse();
    for i in 0..to_remove.len() {
        sell_orders.remove(to_remove[i]);
    }
}
