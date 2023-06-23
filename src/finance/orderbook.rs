use std::rc::Rc;
use std::cmp::Ordering;
use std::cell::RefCell;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

type BidsMap = Rc<RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>>;
type AsksMap = Rc<RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>>;

enum RestingOrderType {
    Bid(RestingOrder),
    Ask(RestingOrder),
}

enum OrderbookSide {
    Bid(f64),
    Ask(f64)
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
    // Inserts resting order into orderbook 
    fn insert_order (&mut self, order: RestingOrderType) {

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
    // Returns all asks
    fn get_asks(&self) -> BidsMap {
        let binding = self.asks
            .clone();

        binding
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
    // Returns all bids
    fn get_bids(&self) -> AsksMap {
        let binding = self.bids
            .clone();

        binding
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
    // Checks to see if your trade size can be filled in full at a specific price
    // ToDo: Determine what should happen at equal
    fn safety_check_size (&self, price: OrderbookSide, size: f64) -> bool {

        match price {
            OrderbookSide::Bid(bid) => {
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

            OrderbookSide::Ask(ask) => {
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
        }
    }
}