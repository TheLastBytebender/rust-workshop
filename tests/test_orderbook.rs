use std::rc::Rc;
use std::cell::RefCell;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

type BidsMap = Rc<RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>>;
type AsksMap = Rc<RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>>;

enum RestingOrderType {
    Bid(RestingOrder),
    Ask(RestingOrder),
}

#[derive(Clone, Debug, PartialEq)]
struct RestingOrder {
    price: f64,
    size: f64,
    ts: i32
}

#[derive(Clone, Debug, PartialEq)]
struct Orderbook {
    asks: AsksMap,
    bids: BidsMap,
    last_update_time: i32,
}

impl Orderbook {
    fn insert_order(&mut self, order: RestingOrderType) {

        match order {
            RestingOrderType::Bid(bid) => {
                let price = OrderedFloat(bid.price);
                self.last_update_time = bid.ts; 

                self.bids
                    .borrow_mut()
                    .insert(price, bid);
            }

            RestingOrderType::Ask(ask) => {
                let price = OrderedFloat(ask.price);
                self.last_update_time = ask.ts; 

                self.asks
                    .borrow_mut()
                    .insert(price, ask);
            }
        }
    }

    fn get_asks(&self) -> BidsMap {
        let binding = self.asks
            .clone();

        binding
    }

    fn get_ask(&self) -> RestingOrder {

        let binding = self.asks
            .borrow();

        let (_key, value) = binding
            .first_key_value()
            .expect("Failed to get first key value");

        RestingOrder { price: value.price, size: value.size,  ts: value.ts }
    }

    fn get_bids(&self) -> AsksMap {
        let binding = self.bids
            .clone();

        binding
    }

    fn get_bid(&self) -> RestingOrder {

        let binding = self.bids
            .borrow();

        let (_key, value) = binding
            .last_key_value()
            .expect("Failed to get last key value");

        RestingOrder { price: value.price, size: value.size, ts: value.ts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_order_orderbook() {

        let mut orderbook = Orderbook {
            asks: Rc::new(RefCell::new(BTreeMap::new())),
            bids: Rc::new(RefCell::new(BTreeMap::new())),
            last_update_time: 0,
        };

        let resting_order_bid = RestingOrder {
            price: 10.0,
            size: 5.0,
            ts: 1_000_000
        };

        orderbook.insert_order(RestingOrderType::Bid(resting_order_bid));

        let resting_order_ask = RestingOrder {
            price: 9.0,
            size: 10.0,
            ts: 1_000_000
        };

        orderbook.insert_order(RestingOrderType::Ask(resting_order_ask));

        assert_eq!(orderbook.asks.borrow().len(), 1);
        assert_eq!(orderbook.bids.borrow().len(), 1);
    }
    
    #[test]
    fn test_get_ask_orderbook() {

        let mut orderbook = Orderbook {
            asks: Rc::new(RefCell::new(BTreeMap::new())),
            bids: Rc::new(RefCell::new(BTreeMap::new())),
            last_update_time: 0,
        };

        let resting_order_ask_1 = RestingOrder {
            price: 9.0,
            size: 10.0,
            ts: 1_000_000
        };

        let resting_order_ask_2 = RestingOrder {
            price: 10.0,
            size: 100.0,
            ts: 1_000_000
        };

        orderbook.insert_order(RestingOrderType::Ask(resting_order_ask_1));
        orderbook.insert_order(RestingOrderType::Ask(resting_order_ask_2));

        let ask = orderbook.get_ask();
        let should_be_ask =  RestingOrder { price: 9.0,  size: 10.0, ts: 1_000_000 };

        assert_eq!(ask, should_be_ask);
    }

    #[test]
    fn test_get_bid_orderbook() {

        let mut orderbook = Orderbook {
            asks: Rc::new(RefCell::new(BTreeMap::new())),
            bids: Rc::new(RefCell::new(BTreeMap::new())),
            last_update_time: 0,
        };

        let resting_order_bid_1 = RestingOrder {
            price: 10.0,
            size: 5.0,
            ts: 1_000_000
        };

        let resting_order_bid_2 = RestingOrder {
            price: 9.0,
            size: 10.0,
            ts: 1_000_000
        };

        orderbook.insert_order(RestingOrderType::Bid(resting_order_bid_1));
        orderbook.insert_order(RestingOrderType::Bid(resting_order_bid_2));

        let bid = orderbook.get_bid();

        assert_eq!(bid, RestingOrder { price: 10.0, size: 5.0, ts: 1_000_000});
    }

    #[test]
    fn test_get_bids_orderbook() {

        let mut orderbook = Orderbook {
            asks: Rc::new(RefCell::new(BTreeMap::new())),
            bids: Rc::new(RefCell::new(BTreeMap::new())),
            last_update_time: 0,
        };

        let resting_order_bid_1 = RestingOrder {
            price: 10.0,
            size: 5.0,
            ts: 1_000_000
        };

        let resting_order_bid_2 = RestingOrder {
            price: 9.0,
            size: 10.0,
            ts: 1_000_000
        };

        orderbook.insert_order(RestingOrderType::Bid(resting_order_bid_1));
        orderbook.insert_order(RestingOrderType::Bid(resting_order_bid_2));

        let bids = orderbook.get_bids();
        let should_be_bids = orderbook.bids;

        assert_eq!(bids, should_be_bids)
    }

    #[test]
    fn test_get_asks_orderbook() {

        let mut orderbook = Orderbook {
            asks: Rc::new(RefCell::new(BTreeMap::new())),
            bids: Rc::new(RefCell::new(BTreeMap::new())),
            last_update_time: 0,
        };

        let resting_order_ask_1 = RestingOrder {
            price: 9.0,
            size: 10.0,
            ts: 1_000_000
        };

        let resting_order_ask_2 = RestingOrder {
            price: 10.0,
            size: 100.0,
            ts: 1_000_000
        };

        orderbook.insert_order(RestingOrderType::Ask(resting_order_ask_1));
        orderbook.insert_order(RestingOrderType::Ask(resting_order_ask_2));

        let asks = orderbook.get_asks();
        let should_be_asks = orderbook.asks;

        assert_eq!(asks, should_be_asks)
    }
}
