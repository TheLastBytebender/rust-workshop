use rand::Rng;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use std::time::{ SystemTime, UNIX_EPOCH };

type BidsMap = RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>;
type AsksMap = RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>;

#[derive(Clone)]
enum RestingOrderType {
    BidOrder(RestingOrder),
    AskOrder(RestingOrder),
    BidPrice(f64),
    AskPrice(f64)
}

#[derive(Clone, Debug, PartialEq)]
struct RestingOrder {
    price: f64,
    size: f64,
    ts: u128
}

#[derive(Clone, Debug, PartialEq)]
struct Orderbook {
    asks: AsksMap,
    bids: BidsMap,
    last_update_time: u128,
}

impl Orderbook {
    fn new() -> Orderbook {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get duration")
            .as_millis();

        Orderbook {
            asks: RefCell::new(BTreeMap::new()),
            bids: RefCell::new(BTreeMap::new()),
            last_update_time: current_time
        }
    }

    // Inserts resting order into orderbook 
    fn insert_order (&mut self, order: RestingOrderType) {

        match order {
            RestingOrderType::BidOrder(bid) => {
                let price = OrderedFloat(bid.price);
                self.last_update_time = bid.ts; 

                self.bids
                    .borrow_mut()
                    .insert(price, bid);
            }

            RestingOrderType::AskOrder(ask) => {
                let price = OrderedFloat(ask.price);
                self.last_update_time = ask.ts; 

                self.asks
                    .borrow_mut()
                    .insert(price, ask);
            }

            _ => todo!()
        }
    }
    // Returns ask closest to mid-price
    fn get_ask(&self) -> RestingOrder {

        let binding = self.asks
            .borrow();

        let (_key, value) = binding
            .first_key_value()
            .expect("Failed to get first key value");

        RestingOrder { price: value.price, size: value.size,  ts: value.ts }
    }
    // Returns bid closest to mid-price
    fn get_bid(&self) -> RestingOrder {

        let binding = self.bids
            .borrow();

        let (_key, value) = binding
            .last_key_value()
            .expect("Failed to get last key value");

        RestingOrder { price: value.price, size: value.size, ts: value.ts }
    }
    // Returns all asks
    fn get_asks(&self) -> &BidsMap {
        let asks = &self.asks;

        asks
    }
    // Returns all bids
    fn get_bids(&self) -> &AsksMap {
        let bids = &self.bids;

        bids
    }
    // Calculates the current orderbook skew
    fn get_ordebook_skew(&self) -> f64 {

        let buy_side_depth: f64 = self
            .bids
            .borrow()
            .values()
            .map(|order| order.size)
            .sum();

        let sell_side_depth: f64 = self
            .asks
            .borrow()
            .values()
            .map(|order| order.size)
            .sum();

        let value = buy_side_depth.ln() - sell_side_depth.ln();

        value
    }
    // Gets orderbook mid price
    fn get_mid_price(&self) -> f64 {
        let bid_price = self
            .get_bid()
            .price;

        let ask_price = self
            .get_ask()
            .price;

        let mid_price = (bid_price + ask_price) / 2.0;

        mid_price
    }
    // Gets orderbook spread
    fn get_orderbook_spread(&self) -> f64 {
        let bid_price = self
            .get_bid()
            .price;

        let ask_price = self
            .get_ask()
            .price;

        let spread = ask_price - bid_price;

        spread
    }
    // Checks to see if your trade size can be filled in full at a specific price
    // ToDo: Determine what should happen at equal
    // true = Trade is safe 
    // false = Trade is unsafe
    fn safety_check_size (&self, price: RestingOrderType, size: f64) -> bool {

        match price {
            RestingOrderType::BidPrice(bid) => {
                let check_price = OrderedFloat(bid);
                let check_size = self.bids
                    .borrow()
                    .get(&check_price)
                    .expect("Failed to get bid size")
                    .size;

                let result = size.partial_cmp(&check_size)
                    .expect("Failed to compare values");

                match result {
                    Ordering::Greater => false,
                    Ordering::Equal => true,
                    Ordering::Less => true
                }
            }

            RestingOrderType::AskPrice(ask) => {
                let check_price = OrderedFloat(ask);
                let check_size = self.asks 
                    .borrow()
                    .get(&check_price)
                    .expect("Failed to get ask size")
                    .size;

                let result = size.partial_cmp(&check_size)
                    .expect("Failed to compare values");

                match result {
                    Ordering::Greater => false,
                    Ordering::Equal => true,
                    Ordering::Less => true
                }
            }

            _ => todo!()
        }
    }
    // Safety Check
    fn safety_check_spread(&self, max_spread: f64) -> bool {
        // Goal of this function will check the spread to see if it is too volatile
        // If so stop trading until things calm down
        let current_spread = self.get_orderbook_spread();

        if current_spread > max_spread {
            false
        } else {
            true
        }
    }
}

/*
TEST ARE HERE
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_order_orderbook() {

        let mut orderbook = Orderbook::new();

        let resting_order_bid = RestingOrder {
            price: 10.0,
            size: 5.0,
            ts: 1_000_000
        };

        let resting_order_ask = RestingOrder {
            price: 9.0,
            size: 10.0,
            ts: 1_000_100
        };

        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask));

        assert_eq!(orderbook.asks.borrow().len(), 1);
        assert_eq!(orderbook.bids.borrow().len(), 1);
        assert_eq!(orderbook.last_update_time, 1_000_100)
    }
    
    #[test]
    fn test_get_ask_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_ask_1 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_ask_2 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let test_copy_1 = resting_order_ask_1.clone();
        let test_copy_2 = resting_order_ask_2.clone();

        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask_1));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask_2));

        let ask = orderbook.get_ask();

        if test_copy_1.price > test_copy_2.price {
            assert_eq!(ask, test_copy_2);

        } else {
            assert_eq!(ask, test_copy_1)
        }
    }

    #[test]
    fn test_get_bid_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_bid_1 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_bid_2 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let test_copy_1 = resting_order_bid_1.clone();
        let test_copy_2 = resting_order_bid_2.clone();

        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid_1));
        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid_2));

        let bid = orderbook.get_bid();

        if test_copy_1.price > test_copy_2.price {
            assert_eq!(bid, test_copy_1);

        } else {
            assert_eq!(bid, test_copy_2)
        }

    }

    #[test]
    fn test_get_bids_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_bid_1 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_bid_2 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid_1));
        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid_2));

        let bids = orderbook.get_bids();
        let should_be_bids = &orderbook.bids;

        assert_eq!(bids, should_be_bids);
    }

    #[test]
    fn test_get_asks_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_ask_1 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_ask_2 = RestingOrder {
            price: rng.gen::<f64>(),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask_1));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask_2));

        let asks = orderbook.get_asks();
        let should_be_asks = &orderbook.asks;

        assert_eq!(asks, should_be_asks);
    }

    #[test]
    fn test_safety_check_size_orderbook() {

        let mut orderbook = Orderbook::new();

        let resting_order_ask = RestingOrder {
            price: 11.0,
            size: 10.0,
            ts: 1_000_000
        };

        let resting_order_bid = RestingOrder {
            price: 10.0,
            size: 100.0,
            ts: 1_000_200
        };

        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask));
        // Bids Block
        assert_eq!(orderbook.safety_check_size(RestingOrderType::BidPrice(10.0), 110.0), false);
        assert_eq!(orderbook.safety_check_size(RestingOrderType::BidPrice(10.0), 95.0), true);
        assert_eq!(orderbook.safety_check_size(RestingOrderType::BidPrice(10.0), 100.0), true);
        // Asks Block
        assert_eq!(orderbook.safety_check_size(RestingOrderType::AskPrice(11.0), 11.0), false);
        assert_eq!(orderbook.safety_check_size(RestingOrderType::AskPrice(11.0), 9.0), true);
        assert_eq!(orderbook.safety_check_size(RestingOrderType::AskPrice(11.0), 10.0), true);
    }

    #[test]
    fn test_get_mid_price_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_ask = RestingOrder {
            price: rng.gen_range(50.0..100.0),
            size: 10.0,
            ts: 1_000_000
        };

        let resting_order_ask_2 = RestingOrder {
            price: rng.gen_range(50.0..100.0),
            size: 10.0,
            ts: 1_000_000
        };

        let resting_order_bid = RestingOrder {
            price: rng.gen_range(0.1..49.9),
            size: 100.0,
            ts: 1_000_200
        };

        let resting_order_bid_2 = RestingOrder {
            price: rng.gen_range(0.1..49.9),
            size: 100.0,
            ts: 1_000_200
        };

        let b1 = resting_order_bid.price;
        let b2 = resting_order_bid_2.price;
        let a1 = resting_order_ask.price;
        let a2 = resting_order_ask_2.price;

        let bid = f64::max(b1, b2);
        let ask = f64::min(a1, a2);

        let result = (ask + bid) / 2.0;

        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask));
        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid_2));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask_2));

        let mid_price = orderbook.get_mid_price();

        assert_eq!(mid_price, result);
    }

    #[test]
    fn test_get_ordebook_skew_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_ask_2 = RestingOrder {
            price: rng.gen_range(50.0..100.0),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_ask = RestingOrder {
            price: rng.gen_range(50.0..100.0),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_bid = RestingOrder {
            price: rng.gen_range(0.1..49.9),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_bid_2 = RestingOrder {
            price: rng.gen_range(0.1..49.9),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let b1 = resting_order_bid.size;
        let b2 = resting_order_bid_2.size;
        let a1 = resting_order_ask.size;
        let a2 = resting_order_ask_2.size;


        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask));
        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid_2));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask_2));

        let anw = ((b1 + b2) as f64).ln() - ((a1 + a2) as f64).ln();

        assert_eq!(anw, orderbook.get_ordebook_skew());
    }

    #[test]
    fn test_get_orderbook_spread_orderbook() {

        let mut orderbook = Orderbook::new();
        let mut rng = rand::thread_rng();

        let resting_order_ask = RestingOrder {
            price: rng.gen_range(50.0..100.0),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let resting_order_bid = RestingOrder {
            price: rng.gen_range(0.1..49.9),
            size: rng.gen::<f64>(),
            ts: rng.gen::<u128>()
        };

        let b1 = resting_order_bid.price;
        let a1 = resting_order_ask.price;

        orderbook.insert_order(RestingOrderType::BidOrder(resting_order_bid));
        orderbook.insert_order(RestingOrderType::AskOrder(resting_order_ask));

        let spread = orderbook.get_orderbook_spread();
        let anw = a1 - b1;

        assert_eq!(anw, spread);
    }
}
