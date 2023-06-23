use std::rc::Rc;
use std::cell::RefCell;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

type BidsMap = Rc<RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>>;
type AsksMap = Rc<RefCell<BTreeMap<OrderedFloat<f64>, RestingOrder>>>;

#[derive(Clone, Debug, PartialEq)]
struct RestingOrder {
    price: f64,
    size: f64,
    ts: i32
}

enum OrderbookType {
    Bid(RestingOrder),
    Ask(RestingOrder),
}

struct Orderbook {
    asks: AsksMap,
    bids: BidsMap,
    last_update_time: i32,
}

impl Orderbook {
    fn insert_order(&mut self, order: OrderbookType) {

        match order {
            OrderbookType::Bid(bid) => {
                let price = OrderedFloat(bid.price);
                self.last_update_time = bid.ts; 

                self.bids
                    .borrow_mut()
                    .insert(price, bid);
            }

            OrderbookType::Ask(ask) => {
                let price = OrderedFloat(ask.price);
                self.last_update_time = ask.ts; 

                self.asks
                    .borrow_mut()
                    .insert(price, ask);
            }
        }
    }

    fn get_ask(&self) -> RestingOrder {

        let binding = self.asks
            .borrow();

        let (_key, value) = binding
            .first_key_value()
            .expect("Failed to get first key value");

        RestingOrder { price: value.price, size: value.size,  ts: value.ts }
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