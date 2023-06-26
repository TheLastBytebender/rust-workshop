use std::cmp::Ordering;
use std::cell::RefCell;
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

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

        Orderbook {
            asks: RefCell::new(BTreeMap::new()),
            bids: RefCell::new(BTreeMap::new()),
            last_update_time: 0
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
    // Calculates the current orderbook skew by range
    fn get_ordebook_skew_by_range(&self, diff: &f64) -> f64 {
        let mid_price = self.get_mid_price();

        let lower_bound = OrderedFloat(mid_price - diff);
        let upper_bound = OrderedFloat(mid_price + diff);

        let a_binding = self
            .asks
            .borrow();

        let b_binding = self
            .bids
            .borrow();

        let a_value = a_binding
            .range(..upper_bound);

        let b_value = b_binding
            .range(lower_bound..);

        let ask_side_depth_range: f64 = a_value
            .map(|order| order.1.size)
            .sum();

        println!("{:?}", ask_side_depth_range);

        let buy_side_depth_range: f64 = b_value
            .map(|order| order.1.size)
            .sum();

        println!("{:?}", buy_side_depth_range);

        let value = buy_side_depth_range.ln() - ask_side_depth_range.ln();

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
        let current_spread = self.get_orderbook_spread();

        if current_spread > max_spread {
            false
        } else {
            true
        }
    }
}